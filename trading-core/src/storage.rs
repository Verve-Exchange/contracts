use shared::{Error, MarketStats, Order, Position, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD};
use soroban_sdk::{contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    PositionCounter,
    OrderCounter,
    TradingFeeBps,
    MaintenanceMarginBps,
    Position(u64),
    Order(u64),
    UserPositions(Address),
    UserOrders(Address),
    MarketStats(Symbol),
}

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_LIFETIME_THRESHOLD);
}

pub fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD);
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_initialized(env: &Env, initialized: bool) {
    env.storage().instance().set(&DataKey::Initialized, &initialized);
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Initialized).unwrap_or(false)
}

pub fn require_admin(env: &Env) -> Result<(), Error> {
    if !is_initialized(env) {
        return Err(Error::NotInitialized);
    }
    get_admin(env).require_auth();
    Ok(())
}

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

pub fn get_paused(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
}

pub fn require_not_paused(env: &Env) -> Result<(), Error> {
    if get_paused(env) {
        return Err(Error::Paused);
    }
    Ok(())
}

pub fn set_trading_fee_bps(env: &Env, fee_bps: u32) {
    env.storage().instance().set(&DataKey::TradingFeeBps, &fee_bps);
}

pub fn get_trading_fee_bps(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::TradingFeeBps).unwrap_or(10)
}

pub fn set_maintenance_margin_bps(env: &Env, margin_bps: u32) {
    env.storage().instance().set(&DataKey::MaintenanceMarginBps, &margin_bps);
}

pub fn get_maintenance_margin_bps(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::MaintenanceMarginBps).unwrap_or(100)
}

pub fn next_position_id(env: &Env) -> u64 {
    let id: u64 = env.storage().instance().get(&DataKey::PositionCounter).unwrap_or(0);
    let next = id + 1;
    env.storage().instance().set(&DataKey::PositionCounter, &next);
    next
}

pub fn next_order_id(env: &Env) -> u64 {
    let id: u64 = env.storage().instance().get(&DataKey::OrderCounter).unwrap_or(0);
    let next = id + 1;
    env.storage().instance().set(&DataKey::OrderCounter, &next);
    next
}

pub fn set_position(env: &Env, position: &Position) {
    let key = DataKey::Position(position.id);
    env.storage().persistent().set(&key, position);
    extend_persistent_ttl(env, &key);
}

pub fn get_position(env: &Env, position_id: u64) -> Result<Position, Error> {
    let key = DataKey::Position(position_id);
    env.storage().persistent().get(&key).ok_or(Error::PositionNotFound)
}

pub fn set_order(env: &Env, order: &Order) {
    let key = DataKey::Order(order.id);
    env.storage().persistent().set(&key, order);
    extend_persistent_ttl(env, &key);
}

pub fn get_order(env: &Env, order_id: u64) -> Result<Order, Error> {
    let key = DataKey::Order(order_id);
    env.storage().persistent().get(&key).ok_or(Error::OrderNotFound)
}

pub fn add_user_position(env: &Env, user: &Address, position_id: u64) {
    let key = DataKey::UserPositions(user.clone());
    let mut ids: Vec<u64> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
    ids.push_back(position_id);
    env.storage().persistent().set(&key, &ids);
    extend_persistent_ttl(env, &key);
}

pub fn get_user_position_ids(env: &Env, user: &Address) -> Vec<u64> {
    let key = DataKey::UserPositions(user.clone());
    env.storage().persistent().get(&key).unwrap_or(Vec::new(env))
}

pub fn add_user_order(env: &Env, user: &Address, order_id: u64) {
    let key = DataKey::UserOrders(user.clone());
    let mut ids: Vec<u64> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
    ids.push_back(order_id);
    env.storage().persistent().set(&key, &ids);
    extend_persistent_ttl(env, &key);
}

pub fn get_user_order_ids(env: &Env, user: &Address) -> Vec<u64> {
    let key = DataKey::UserOrders(user.clone());
    env.storage().persistent().get(&key).unwrap_or(Vec::new(env))
}

pub fn get_market_stats(env: &Env, asset: &Symbol) -> MarketStats {
    let key = DataKey::MarketStats(asset.clone());
    env.storage().persistent().get(&key).unwrap_or(MarketStats {
        asset: asset.clone(),
        total_long_size: 0,
        total_short_size: 0,
        open_interest: 0,
        funding_rate: 0,
        last_funding_time: 0,
    })
}

pub fn set_market_stats(env: &Env, stats: &MarketStats) {
    let key = DataKey::MarketStats(stats.asset.clone());
    env.storage().persistent().set(&key, stats);
    extend_persistent_ttl(env, &key);
}
