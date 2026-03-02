use shared::{Error, LPStake, PoolInfo, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    Pool(Address),
    LpBalance(Address, Address),
    Stake(Address, Address),
}

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_LIFETIME_THRESHOLD);
}

fn extend_persistent_ttl(env: &Env, key: &DataKey) {
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

pub fn set_pool(env: &Env, token: &Address, pool: &PoolInfo) {
    let key = DataKey::Pool(token.clone());
    env.storage().persistent().set(&key, pool);
    extend_persistent_ttl(env, &key);
}

pub fn get_pool(env: &Env, token: &Address) -> Result<PoolInfo, Error> {
    let key = DataKey::Pool(token.clone());
    env.storage().persistent().get(&key).ok_or(Error::PoolNotFound)
}

pub fn set_lp_balance(env: &Env, user: &Address, token: &Address, balance: i128) {
    let key = DataKey::LpBalance(user.clone(), token.clone());
    env.storage().persistent().set(&key, &balance);
    extend_persistent_ttl(env, &key);
}

pub fn get_lp_balance(env: &Env, user: &Address, token: &Address) -> i128 {
    let key = DataKey::LpBalance(user.clone(), token.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_stake(env: &Env, user: &Address, token: &Address, stake: &LPStake) {
    let key = DataKey::Stake(user.clone(), token.clone());
    env.storage().persistent().set(&key, stake);
    extend_persistent_ttl(env, &key);
}

pub fn get_stake(env: &Env, user: &Address, token: &Address) -> Option<LPStake> {
    let key = DataKey::Stake(user.clone(), token.clone());
    env.storage().persistent().get(&key)
}
