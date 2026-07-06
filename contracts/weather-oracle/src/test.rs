#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Env};

#[test]
fn test_median_across_three_providers() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    let p3 = Address::generate(&env);

    let contract_id = env.register_contract(None, WeatherOracleContract);
    let client = WeatherOracleContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.register_provider(&admin, &p1);
    client.register_provider(&admin, &p2);
    client.register_provider(&admin, &p3);

    let location = symbol_short!("KADUNA");

    // One low outlier reading shouldn't move the median much.
    client.submit_reading(&p1, &location, &5_i128);
    client.submit_reading(&p2, &location, &42_i128);
    client.submit_reading(&p3, &location, &45_i128);

    let result = client.get_rainfall(&location);
    assert_eq!(result.value_mm, 42); // median of [5, 42, 45]
}

// TODO: test submit_reading fails for an unregistered provider
// TODO: test get_rainfall fails with NoDataForLocation for an unseen location
// TODO: test median with an even number of providers
// TODO: test register_provider fails if called by a non-admin address
