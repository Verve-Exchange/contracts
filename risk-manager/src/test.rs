#![cfg(test)]

use super::*;
use shared::{AssetClass, PRECISION};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, Symbol};

#[test]
fn test_assess_position_risk() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let contract_id = env.register(RiskManager, ());
    let client = RiskManagerClient::new(&env, &contract_id);

    client.initialize(&admin);
    let asset = Symbol::new(&env, "XAUUSD");

    client.update_risk_parameters(
        &asset,
        &AssetClass::Commodity,
        &1000,
        &500,
        &500,
        &10,
        &(10_000 * PRECISION),
        &(1_000_000 * PRECISION),
    );

    let profile = client.assess_position_risk(
        &user,
        &asset,
        &(1_000 * PRECISION),
        &(200 * PRECISION),
        &5,
    );

    assert!(profile.margin_available >= 0);
}
