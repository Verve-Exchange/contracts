#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Bytes, Env, Symbol};
use shared::PRECISION;

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);

    assert_eq!(client.get_admin(), admin);
    assert!(!client.is_paused());
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.initialize(&admin); // Should panic
}

#[test]
fn test_register_and_update_price() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);

    let asset = Symbol::new(&env, "XAUUSD");
    let feed_id = Bytes::new(&env);

    // Register feed
    client.register_feed(&asset, &feed_id, &60, &100);

    // Update price
    let price = 2000 * PRECISION;
    client.update_price(&asset, &price, &100, &-7);

    // Get price
    let price_data = client.get_price(&asset);
    assert_eq!(price_data.price, price);
    assert_eq!(price_data.confidence, 100);
}

#[test]
fn test_price_staleness() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);

    let asset = Symbol::new(&env, "XAUUSD");
    let feed_id = Bytes::new(&env);

    // Register feed with 60 second staleness
    client.register_feed(&asset, &feed_id, &60, &100);

    // Update price
    client.update_price(&asset, &(2000 * PRECISION), &100, &-7);

    // Advance time by 70 seconds
    env.ledger().with_mut(|li| {
        li.timestamp += 70;
    });

    // Should fail due to staleness
    let result = client.try_get_price(&asset);
    assert!(result.is_err());
}

#[test]
fn test_circuit_breaker() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);

    let asset = Symbol::new(&env, "XAUUSD");
    let feed_id = Bytes::new(&env);

    // Register feed with 1% max deviation
    client.register_feed(&asset, &feed_id, &60, &100);

    // Update initial price
    client.update_price(&asset, &(2000 * PRECISION), &100, &-7);

    // Try to update with 2% deviation (should fail)
    let result = client.try_update_price(&asset, &(2040 * PRECISION), &100, &-7);
    assert!(result.is_err());

    // Enable circuit breaker
    client.set_circuit_breaker(&asset, &true);

    // Now the same update should succeed
    client.update_price(&asset, &(2040 * PRECISION), &100, &-7);

    let price_data = client.get_price(&asset);
    assert_eq!(price_data.price, 2040 * PRECISION);
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);

    let asset = Symbol::new(&env, "XAUUSD");
    let feed_id = Bytes::new(&env);

    client.register_feed(&asset, &feed_id, &60, &100);
    client.update_price(&asset, &(2000 * PRECISION), &100, &-7);

    // Pause
    client.pause();
    assert!(client.is_paused());

    // Should not be able to get price when paused
    let result = client.try_get_price(&asset);
    assert!(result.is_err());

    // Unpause
    client.unpause();
    assert!(!client.is_paused());

    // Should work again
    let price_data = client.get_price(&asset);
    assert_eq!(price_data.price, 2000 * PRECISION);
}

#[test]
fn test_get_multiple_prices() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(OracleManager, ());
    let client = OracleManagerClient::new(&env, &contract_id);

    client.initialize(&admin);

    let feed_id = Bytes::new(&env);

    // Register multiple assets
    let xau = Symbol::new(&env, "XAUUSD");
    let xag = Symbol::new(&env, "XAGUSD");
    let eur = Symbol::new(&env, "EURUSD");

    client.register_feed(&xau, &feed_id, &60, &100);
    client.register_feed(&xag, &feed_id, &60, &100);
    client.register_feed(&eur, &feed_id, &60, &100);

    // Update prices
    client.update_price(&xau, &(2000 * PRECISION), &100, &-7);
    client.update_price(&xag, &(25 * PRECISION), &100, &-7);
    client.update_price(&eur, &(1_0500000), &100, &-7); // 1.05

    // Get all prices at once
    let assets = soroban_sdk::vec![&env, xau.clone(), xag.clone(), eur.clone()];
    let prices = client.get_prices(&assets);

    assert_eq!(prices.len(), 3);
    assert_eq!(prices.get(0).unwrap().price, 2000 * PRECISION);
    assert_eq!(prices.get(1).unwrap().price, 25 * PRECISION);
    assert_eq!(prices.get(2).unwrap().price, 1_0500000);
}
