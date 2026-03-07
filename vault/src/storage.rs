use shared::{Error, PoolInfo, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    Token,
    DepositFeeBps,
    WithdrawFeeBps,
    Pool,
    LPBalance(Address),
}

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_LIFETIME_THRESHOLD);
}

fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(
            key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_LIFETIME_THRESHOLD,
        );
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_initialized(env: &Env, initialized: bool) {
    env.storage()
        .instance()
        .set(&DataKey::Initialized, &initialized);
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

pub fn require_admin(env: &Env) -> Result<(), Error> {
    require_initialized(env)?;
    get_admin(env).require_auth();
    Ok(())
}

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
    require_initialized(env)?;
    if get_paused(env) {
        return Err(Error::Paused);
    }
    Ok(())
}

pub fn set_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Token, token);
}

pub fn get_token(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Token).unwrap()
}

pub fn set_deposit_fee_bps(env: &Env, fee: u32) {
    env.storage().instance().set(&DataKey::DepositFeeBps, &fee);
}

pub fn get_deposit_fee_bps(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::DepositFeeBps)
        .unwrap_or(0)
}

pub fn set_withdraw_fee_bps(env: &Env, fee: u32) {
    env.storage().instance().set(&DataKey::WithdrawFeeBps, &fee);
}

pub fn get_withdraw_fee_bps(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::WithdrawFeeBps)
        .unwrap_or(0)
}

pub fn set_pool(env: &Env, pool: &PoolInfo) {
    env.storage().instance().set(&DataKey::Pool, pool);
}

pub fn get_pool(env: &Env) -> PoolInfo {
    env.storage().instance().get(&DataKey::Pool).unwrap()
}

pub fn set_lp_balance(env: &Env, user: &Address, balance: i128) {
    let key = DataKey::LPBalance(user.clone());
    env.storage().persistent().set(&key, &balance);
    extend_persistent_ttl(env, &key);
}

pub fn get_lp_balance(env: &Env, user: &Address) -> i128 {
    let key = DataKey::LPBalance(user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}
