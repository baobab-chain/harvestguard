#![no_std]
//! weather-oracle: a small, purpose-built oracle for rainfall data,
//! following SEP-40-style conventions (see docs/adr/001-oracle-design.md
//! for why this isn't a Reflector integration). Multiple registered
//! providers submit readings per location; queries return the median.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct RainfallData {
    pub value_mm: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
struct Reading {
    value_mm: i128,
    timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Provider(Address),
    LocationProviders(Symbol),
    Reading(Symbol, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    NotAdmin = 1,
    ProviderNotRegistered = 2,
    NoDataForLocation = 3,
}

#[contract]
pub struct WeatherOracleContract;

#[contractimpl]
impl WeatherOracleContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Admin approves a data provider.
    ///
    /// KNOWN GAP: single-admin controlled, not decentralized. See
    /// docs/adr/002-provider-trust.md — this is the top-priority issue.
    pub fn register_provider(env: Env, admin: Address, provider: Address) -> Result<(), Error> {
        admin.require_auth();
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotAdmin)?;
        if stored_admin != admin {
            return Err(Error::NotAdmin);
        }
        env.storage().instance().set(&DataKey::Provider(provider), &true);
        Ok(())
    }

    pub fn is_provider_registered(env: Env, provider: Address) -> bool {
        env.storage()
            .instance()
            .get::<_, bool>(&DataKey::Provider(provider))
            .unwrap_or(false)
    }

    /// A registered provider submits a rainfall reading (millimeters)
    /// for a location.
    pub fn submit_reading(
        env: Env,
        provider: Address,
        location: Symbol,
        value_mm: i128,
    ) -> Result<(), Error> {
        provider.require_auth();

        if !Self::is_provider_registered(env.clone(), provider.clone()) {
            return Err(Error::ProviderNotRegistered);
        }

        let reading = Reading { value_mm, timestamp: env.ledger().timestamp() };
        env.storage()
            .instance()
            .set(&DataKey::Reading(location.clone(), provider.clone()), &reading);

        let mut providers: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::LocationProviders(location.clone()))
            .unwrap_or(Vec::new(&env));

        if !providers.contains(&provider) {
            providers.push_back(provider);
            env.storage()
                .instance()
                .set(&DataKey::LocationProviders(location), &providers);
        }

        Ok(())
    }

    /// Returns the median rainfall reading across all providers who have
    /// submitted for this location.
    ///
    /// KNOWN GAP: no minimum-provider-count enforcement — with fewer
    /// than 3 providers, "median resists a bad actor" doesn't
    /// meaningfully hold. See docs/ARCHITECTURE.md.
    pub fn get_rainfall(env: Env, location: Symbol) -> Result<RainfallData, Error> {
        let providers: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::LocationProviders(location.clone()))
            .ok_or(Error::NoDataForLocation)?;

        if providers.is_empty() {
            return Err(Error::NoDataForLocation);
        }

        let mut values: Vec<i128> = Vec::new(&env);
        for provider in providers.iter() {
            if let Some(reading) = env
                .storage()
                .instance()
                .get::<_, Reading>(&DataKey::Reading(location.clone(), provider))
            {
                values.push_back(reading.value_mm);
            }
        }

        let median = median_of(values);

        Ok(RainfallData { value_mm: median, timestamp: env.ledger().timestamp() })
    }
}

/// Simple insertion sort + median — fine for the small provider counts
/// this oracle expects (a handful of providers per location, not
/// thousands). A contributor should double check the exact Vec
/// get/set/len method signatures against whatever soroban-sdk version
/// is pinned in Cargo.toml, since minor-version API shifts are common —
/// see ISSUES.md.
fn median_of(mut values: Vec<i128>) -> i128 {
    let len = values.len();

    for i in 1..len {
        let key = values.get(i).unwrap();
        let mut j = i;
        while j > 0 && values.get(j - 1).unwrap() > key {
            let prev = values.get(j - 1).unwrap();
            values.set(j, prev);
            j -= 1;
        }
        values.set(j, key);
    }

    if len % 2 == 1 {
        values.get(len / 2).unwrap()
    } else {
        let a = values.get(len / 2 - 1).unwrap();
        let b = values.get(len / 2).unwrap();
        (a + b) / 2
    }
}

mod test;
