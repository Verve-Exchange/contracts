#![cfg(test)]

extern crate std;

use super::*;
use shared::PRECISION;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env};

#[test]
fn test_claim_usdc_transfer_mode() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token_id = token_contract.address();
    let token_admin = token::StellarAssetClient::new(&env, &token_id);
    let token_client = token::Client::new(&env, &token_id);

    let contract_id = env.register(Faucet, ());
    let client = FaucetClient::new(&env, &contract_id);

    // Initialize in transfer mode (can_mint = false)
    client.initialize(
        &admin,
        &token_id,
        &(100 * PRECISION),
        &60,
        &3,
        &(500 * PRECISION),
        &false,
    );

    // Refill faucet
    token_admin.mint(&admin, &(1_000 * PRECISION));
    client.refill_usdc(&(600 * PRECISION));

    let before = token_client.balance(&user);
    let claimed = client.claim_usdc(&user);
    let after = token_client.balance(&user);

    assert_eq!(claimed, 100 * PRECISION);
    assert_eq!(after - before, 100 * PRECISION);
    assert!(!client.can_mint());
}

#[test]
fn test_claim_usdc_mint_mode() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token_id = token_contract.address();
    let token_client = token::Client::new(&env, &token_id);

    let contract_id = env.register(Faucet, ());
    let client = FaucetClient::new(&env, &contract_id);

    // Initialize in mint mode (can_mint = true)
    client.initialize(
        &admin,
        &token_id,
        &(100 * PRECISION),
        &60,
        &3,
        &(500 * PRECISION),
        &true,
    );

    let before = token_client.balance(&user);
    let claimed = client.claim_usdc(&user);
    let after = token_client.balance(&user);

    assert_eq!(claimed, 100 * PRECISION);
    assert_eq!(after - before, 100 * PRECISION);
    assert!(client.can_mint());
}
