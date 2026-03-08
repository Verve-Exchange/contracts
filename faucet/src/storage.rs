use shared::{ClaimRecord, Error, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_LIFETIME_THRESHOLD};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Initialized,
    Paused,
    UsdcToken,
    AmountPerClaim,
    CooldownSecs,
    MaxClaimsPerDay,
    DailyLimit,
    CanMint,
    Claim(Address),
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

pub fn set_usdc_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::UsdcToken, token);
}

pub fn get_usdc_token(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::UsdcToken).unwrap()
}

pub fn set_amount_per_claim(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::AmountPerClaim, &amount);
}

pub fn get_amount_per_claim(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::AmountPerClaim).unwrap_or(0)
}

pub fn set_cooldown_secs(env: &Env, value: u64) {
    env.storage().instance().set(&DataKey::CooldownSecs, &value);
}

pub fn get_cooldown_secs(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::CooldownSecs).unwrap_or(0)
}

pub fn set_max_claims_per_day(env: &Env, value: u32) {
    env.storage().instance().set(&DataKey::MaxClaimsPerDay, &value);
}

pub fn get_max_claims_per_day(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::MaxClaimsPerDay).unwrap_or(0)
}

pub fn set_daily_limit(env: &Env, value: i128) {
    env.storage().instance().set(&DataKey::DailyLimit, &value);
}

pub fn get_daily_limit(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::DailyLimit).unwrap_or(0)
}

pub fn set_can_mint(env: &Env, can_mint: bool) {
    env.storage().instance().set(&DataKey::CanMint, &can_mint);
}

pub fn get_can_mint(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::CanMint).unwrap_or(false)
}

pub fn set_claim_record(env: &Env, user: &Address, record: &ClaimRecord) {
    let key = DataKey::Claim(user.clone());
    env.storage().persistent().set(&key, record);
    extend_persistent_ttl(env, &key);
}

pub fn get_claim_record(env: &Env, user: &Address, token: &Address) -> ClaimRecord {
    let key = DataKey::Claim(user.clone());
    env.storage().persistent().get(&key).unwrap_or(ClaimRecord {
        user: user.clone(),
        token: token.clone(),
        amount_claimed: 0,
        last_claim_time: 0,
        total_claims: 0,
    })
}
