#![cfg(test)]

use super::*;
use shared::{Direction, OrderType, PRECISION};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, Symbol};

#[test]
fn test_open_close_position() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let trader = Address::generate(&env);
    let contract_id = env.register(TradingCore, ());
    let client = TradingCoreClient::new(&env, &contract_id);

    client.initialize(&admin, &10, &100);

    let asset = Symbol::new(&env, "XAUUSD");
    let collateral = 100 * PRECISION;
    let entry = 2000 * PRECISION;

    let pid = client.open_position(
        &trader,
        &asset,
        &trader,
        &collateral,
        &10,
        &Direction::Long,
        &entry,
    );

    let pnl = client.close_position(&trader, &pid, &(2100 * PRECISION));
    assert!(pnl > 0);
}

#[test]
fn test_order_execution_limit_entry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let trader = Address::generate(&env);
    let contract_id = env.register(TradingCore, ());
    let client = TradingCoreClient::new(&env, &contract_id);

    client.initialize(&admin, &10, &100);

    let asset = Symbol::new(&env, "EURUSD");
    let oid = client.place_order(
        &trader,
        &asset,
        &OrderType::LimitEntry,
        &Direction::Long,
        &1_0500000,
        &(1000 * PRECISION),
        &(100 * PRECISION),
        &10,
        &100,
        &0,
    );

    let pid = client.execute_order(&oid, &1_0510000);
    assert!(pid > 0);
}
