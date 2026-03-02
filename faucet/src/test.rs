#![cfg(test)]

extern crate std;

use super::*;
use shared::PRECISION;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env};

#[test]
fn test_claim_usdc_only() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let token_id = env.register_stellar_asset_contract(admin.clone());
    let token_admin = token::StellarAssetClient::new(&env, &token_id);
    let token_client = token::Client::new(&env, &token_id);

    let contract_id = env.register(Faucet, ());
    let client = FaucetClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &token_id,
        &(100 * PRECISION),
        &60,
        &3,
        &(500 * PRECISION),
    );

    token_admin.mint(&admin, &(1_000 * PRECISION));
    client.refill_usdc(&(600 * PRECISION));

    let before = token_client.balance(&user);
    let claimed = client.claim_usdc(&user);
    let after = token_client.balance(&user);

    assert_eq!(claimed, 100 * PRECISION);
    assert_eq!(after - before, 100 * PRECISION);
}
