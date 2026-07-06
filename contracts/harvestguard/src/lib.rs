#![no_std]
//! harvestguard: parametric crop insurance. Sells policies against a
//! rainfall threshold at a location; pays out automatically once
//! weather-oracle confirms the trigger condition. Early skeleton — see
//! docs/ARCHITECTURE.md for known gaps before using this with real funds.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env, Symbol};
use weather_oracle::WeatherOracleContractClient;

#[contracttype]
#[derive(Clone, Copy, PartialEq)]
pub enum Comparison {
    /// Triggers if rainfall is BELOW the threshold (drought protection).
    Below,
    /// Triggers if rainfall is ABOVE the threshold (flood protection).
    Above,
}

#[contracttype]
#[derive(Clone)]
pub struct Policy {
    pub farmer: Address,
    pub token: Address,
    pub payout: i128,
    pub location: Symbol,
    pub threshold_mm: i128,
    pub comparison: Comparison,
    pub expiry_ledger: u32,
    pub triggered: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    OracleContractId,
    Policy(u32),
    NextPolicyId,
}

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    PolicyNotFound = 1,
    AlreadyTriggered = 2,
    ConditionNotMet = 3,
    Expired = 4,
    NoOracleData = 5,
}

#[contract]
pub struct HarvestGuardContract;

#[contractimpl]
impl HarvestGuardContract {
    pub fn initialize(env: Env, admin: Address, oracle_contract_id: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::OracleContractId, &oracle_contract_id);
    }

    /// An insurer funds the shared payout pool held by this contract.
    ///
    /// KNOWN GAP: no accounting of how much of the pool is already
    /// committed to outstanding policies — see docs/ARCHITECTURE.md.
    pub fn deposit_liquidity(env: Env, insurer: Address, token: Address, amount: i128) {
        insurer.require_auth();
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&insurer, &env.current_contract_address(), &amount);
    }

    /// Farmer buys a policy: pays a premium (added to the pool) in
    /// exchange for a payout if the rainfall condition at `location` is
    /// met before `expiry_ledger`.
    ///
    /// KNOWN GAP: no check that the pool actually holds enough to cover
    /// `payout` — see docs/ARCHITECTURE.md, this is a real risk.
    pub fn create_policy(
        env: Env,
        farmer: Address,
        token: Address,
        premium: i128,
        payout: i128,
        location: Symbol,
        threshold_mm: i128,
        comparison: Comparison,
        expiry_ledger: u32,
    ) -> u32 {
        farmer.require_auth();

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&farmer, &env.current_contract_address(), &premium);

        let policy_id: u32 = env.storage().instance().get(&DataKey::NextPolicyId).unwrap_or(0);

        let policy = Policy {
            farmer,
            token,
            payout,
            location,
            threshold_mm,
            comparison,
            expiry_ledger,
            triggered: false,
        };

        env.storage().instance().set(&DataKey::Policy(policy_id), &policy);
        env.storage().instance().set(&DataKey::NextPolicyId, &(policy_id + 1));

        policy_id
    }

    /// Anyone can call this — the trigger condition is objective and
    /// publicly verifiable via the oracle, so no special permission is
    /// needed to fire a legitimate payout.
    pub fn check_and_trigger_payout(env: Env, policy_id: u32) -> Result<(), Error> {
        let mut policy: Policy = env
            .storage()
            .instance()
            .get(&DataKey::Policy(policy_id))
            .ok_or(Error::PolicyNotFound)?;

        if policy.triggered {
            return Err(Error::AlreadyTriggered);
        }
        if env.ledger().sequence() > policy.expiry_ledger {
            // KNOWN GAP: expired, unpaid premiums currently just stay in
            // the pool — no refund path back to the farmer. See ISSUES.md.
            return Err(Error::Expired);
        }

        let oracle_id: Address = env
            .storage()
            .instance()
            .get(&DataKey::OracleContractId)
            .ok_or(Error::NoOracleData)?;
        let oracle_client = WeatherOracleContractClient::new(&env, &oracle_id);

        // NOTE: get_rainfall returns Result<RainfallData, weather_oracle::Error>
        // on the oracle side. Cross-contract Result-unwrapping conventions
        // can shift between soroban-sdk versions — a contributor should
        // verify this against whatever version is pinned in Cargo.toml
        // (see the equivalent note in weather-oracle/src/lib.rs).
        let rainfall = oracle_client
            .try_get_rainfall(&policy.location)
            .map_err(|_| Error::NoOracleData)?
            .map_err(|_| Error::NoOracleData)?;

        let condition_met = match policy.comparison {
            Comparison::Below => rainfall.value_mm < policy.threshold_mm,
            Comparison::Above => rainfall.value_mm > policy.threshold_mm,
        };

        if !condition_met {
            return Err(Error::ConditionNotMet);
        }

        let token_client = token::Client::new(&env, &policy.token);
        token_client.transfer(&env.current_contract_address(), &policy.farmer, &policy.payout);

        policy.triggered = true;
        env.storage().instance().set(&DataKey::Policy(policy_id), &policy);

        Ok(())
    }

    pub fn get_policy(env: Env, policy_id: u32) -> Result<Policy, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Policy(policy_id))
            .ok_or(Error::PolicyNotFound)
    }
}

mod test;
