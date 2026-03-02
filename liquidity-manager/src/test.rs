#![cfg(test)]

use super::*;
use shared::PRECISION;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

#[test]
fn test_deposit_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token = Address::generate(&env);

    let contract_id = env.register(LiquidityManager, ());
    let client = LiquidityManagerClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.create_pool(&token);

    let minted = client.deposit(&user, &token, &(1_000 * PRECISION));
    assert_eq!(minted, 1_000 * PRECISION);

    let withdrawn = client.withdraw(&user, &token, &(500 * PRECISION));
    assert!(withdrawn > 0);
}
