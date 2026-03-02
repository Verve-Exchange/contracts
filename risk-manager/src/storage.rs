use shared::{Error, RiskParameters, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD};
use soroban_sdk::{contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    RiskParams(Symbol),
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

pub fn set_risk_params(env: &Env, params: &RiskParameters) {
    let key = DataKey::RiskParams(params.asset.clone());
    env.storage().persistent().set(&key, params);
    extend_persistent_ttl(env, &key);
}

pub fn get_risk_params(env: &Env, asset: &Symbol) -> Result<RiskParameters, Error> {
    let key = DataKey::RiskParams(asset.clone());
    env.storage().persistent().get(&key).ok_or(Error::InvalidParameter)
}
