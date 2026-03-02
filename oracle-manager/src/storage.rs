use soroban_sdk::{contracttype, Address, Env, Symbol};
use shared::{Error, PriceData, PriceFeed, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    PriceFeed(Symbol),
    Price(Symbol),
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

// Admin
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn require_admin(env: &Env) -> Result<(), Error> {
    if !is_initialized(env) {
        return Err(Error::NotInitialized);
    }
    let admin = get_admin(env);
    admin.require_auth();
    Ok(())
}

// Initialized
pub fn set_initialized(env: &Env, initialized: bool) {
    env.storage().instance().set(&DataKey::Initialized, &initialized);
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Initialized)
        .unwrap_or(false)
}

pub fn require_initialized(env: &Env) -> Result<(), Error> {
    if !is_initialized(env) {
        return Err(Error::NotInitialized);
    }
    Ok(())
}

// Paused
pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

pub fn get_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

pub fn require_not_paused(env: &Env) -> Result<(), Error> {
    if get_paused(env) {
        return Err(Error::Paused);
    }
    Ok(())
}

// Price Feed
pub fn set_price_feed(env: &Env, asset: &Symbol, feed: &PriceFeed) {
    let key = DataKey::PriceFeed(asset.clone());
    env.storage().persistent().set(&key, feed);
    extend_persistent_ttl(env, &key);
}

pub fn get_price_feed(env: &Env, asset: &Symbol) -> Result<PriceFeed, Error> {
    let key = DataKey::PriceFeed(asset.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(Error::InvalidPriceFeed)
}

// Price
pub fn set_price(env: &Env, asset: &Symbol, price: &PriceData) {
    let key = DataKey::Price(asset.clone());
    env.storage().persistent().set(&key, price);
    extend_persistent_ttl(env, &key);
}

pub fn read_price(env: &Env, asset: &Symbol) -> Option<PriceData> {
    let key = DataKey::Price(asset.clone());
    env.storage().persistent().get(&key)
}
