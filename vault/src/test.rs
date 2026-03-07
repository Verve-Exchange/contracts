#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token = token::Client::new(env, &contract.address());
    let admin_client = token::StellarAssetClient::new(env, &contract.address());
    (token, admin_client)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token, _) = create_token_contract(&env, &token_admin);

    let vault_id = env.register(Vault, ());
    let vault = VaultClient::new(&env, &vault_id);

    vault.initialize(&admin, &token.address, &30, &30);

    assert_eq!(vault.get_admin(), admin);
    assert!(!vault.is_paused());
}

#[test]
fn test_deposit_and_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token, admin_client) = create_token_contract(&env, &token_admin);

    // Mint tokens to user
    admin_client.mint(&user, &1000_0000000);

    let vault_id = env.register(Vault, ());
    let vault = VaultClient::new(&env, &vault_id);

    vault.initialize(&admin, &token.address, &30, &30);

    // Deposit
    let lp_tokens = vault.deposit(&user, &100_0000000);
    assert!(lp_tokens > 0);

    let pool = vault.get_pool_info();
    assert!(pool.total_liquidity > 0);
    assert_eq!(vault.get_lp_balance(&user), lp_tokens);

    // Withdraw
    let withdrawn = vault.withdraw(&user, &lp_tokens);
    assert!(withdrawn > 0);
    assert_eq!(vault.get_lp_balance(&user), 0);
}

#[test]
fn test_reserve_and_release() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token, admin_client) = create_token_contract(&env, &token_admin);

    admin_client.mint(&user, &1000_0000000);

    let vault_id = env.register(Vault, ());
    let vault = VaultClient::new(&env, &vault_id);

    vault.initialize(&admin, &token.address, &30, &30);
    vault.deposit(&user, &100_0000000);

    let pool_before = vault.get_pool_info();
    let available_before = pool_before.available_liquidity;

    // Reserve
    vault.reserve_liquidity(&50_0000000);
    let pool_after = vault.get_pool_info();
    assert_eq!(
        pool_after.available_liquidity,
        available_before - 50_0000000
    );
    assert_eq!(pool_after.reserved_liquidity, 50_0000000);

    // Release
    vault.release_liquidity(&50_0000000);
    let pool_final = vault.get_pool_info();
    assert_eq!(pool_final.available_liquidity, available_before);
    assert_eq!(pool_final.reserved_liquidity, 0);
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token, _) = create_token_contract(&env, &token_admin);

    let vault_id = env.register(Vault, ());
    let vault = VaultClient::new(&env, &vault_id);

    vault.initialize(&admin, &token.address, &30, &30);

    assert!(!vault.is_paused());

    vault.pause();
    assert!(vault.is_paused());

    vault.unpause();
    assert!(!vault.is_paused());
}
