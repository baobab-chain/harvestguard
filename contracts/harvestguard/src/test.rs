#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Env};
use weather_oracle::WeatherOracleContract;

#[test]
fn test_drought_policy_triggers_payout() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let farmer = Address::generate(&env);
    let insurer = Address::generate(&env);
    let provider = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());

    // Deploy and seed the oracle.
    let oracle_id = env.register_contract(None, WeatherOracleContract);
    let oracle_client = WeatherOracleContractClient::new(&env, &oracle_id);
    oracle_client.initialize(&admin);
    oracle_client.register_provider(&admin, &provider);

    let location = symbol_short!("KADUNA");
    oracle_client.submit_reading(&provider, &location, &5_i128); // well below any reasonable threshold

    // Deploy the insurance contract.
    let contract_id = env.register_contract(None, HarvestGuardContract);
    let client = HarvestGuardContractClient::new(&env, &contract_id);
    client.initialize(&admin, &oracle_id);

    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&insurer, &1_000_000);
    token_client.mint(&farmer, &10_000);

    client.deposit_liquidity(&insurer, &token_id, &500_000_i128);

    let policy_id = client.create_policy(
        &farmer,
        &token_id,
        &1_000_i128,  // premium
        &50_000_i128, // payout
        &location,
        &20_i128, // threshold_mm
        &Comparison::Below,
        &1_000_000_u32, // far-future expiry ledger
    );

    client.check_and_trigger_payout(&policy_id);

    let policy = client.get_policy(&policy_id);
    assert!(policy.triggered);
}

// TODO: test check_and_trigger_payout fails with ConditionNotMet when rainfall is above threshold
// TODO: test check_and_trigger_payout fails with AlreadyTriggered on a second call
// TODO: test check_and_trigger_payout fails with Expired past the expiry ledger
// TODO: test an Above (flood) comparison policy
