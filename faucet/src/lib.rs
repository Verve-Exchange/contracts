#![no_std]

mod storage;

#[cfg(test)]
mod test;

use shared::{validate_positive_amount, ClaimRecord, Error};
use soroban_sdk::{contract, contractimpl, token, Address, Env};
use storage::*;

#[contract]
pub struct Faucet;

#[contractimpl]
impl Faucet {
    pub fn initialize(
        env: Env,
        admin: Address,
        usdc_token: Address,
        amount_per_claim: i128,
        cooldown_secs: u64,
        max_claims_per_day: u32,
        daily_limit: i128,
    ) -> Result<(), Error> {
        if is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        validate_positive_amount(amount_per_claim)?;
        validate_positive_amount(daily_limit)?;

        set_admin(&env, &admin);
        set_usdc_token(&env, &usdc_token);
        set_amount_per_claim(&env, amount_per_claim);
        set_cooldown_secs(&env, cooldown_secs);
        set_max_claims_per_day(&env, max_claims_per_day);
        set_daily_limit(&env, daily_limit);
        set_initialized(&env, true);
        set_paused(&env, false);
        extend_instance_ttl(&env);
        Ok(())
    }

    pub fn refill_usdc(env: Env, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;
        validate_positive_amount(amount)?;

        let token = token::Client::new(&env, &get_usdc_token(&env));
        let admin = get_admin(&env);
        let current = env.current_contract_address();
        token.transfer(&admin, &current, &amount);
        Ok(())
    }

    pub fn claim_usdc(env: Env, user: Address) -> Result<i128, Error> {
        require_not_paused(&env)?;
        user.require_auth();

        let token_id = get_usdc_token(&env);
        let amount = get_amount_per_claim(&env);
        let cooldown = get_cooldown_secs(&env);
        let max_claims = get_max_claims_per_day(&env);
        let daily_limit = get_daily_limit(&env);
        let now = env.ledger().timestamp();

        let mut record: ClaimRecord = get_claim_record(&env, &user, &token_id);

        if record.last_claim_time != 0 && now < record.last_claim_time + cooldown {
            return Err(Error::CooldownNotElapsed);
        }

        let day_secs = 24 * 3600;
        if record.last_claim_time != 0 && now / day_secs != record.last_claim_time / day_secs {
            record.amount_claimed = 0;
            record.total_claims = 0;
        }

        if record.total_claims >= max_claims {
            return Err(Error::ClaimLimitExceeded);
        }

        if record.amount_claimed + amount > daily_limit {
            return Err(Error::ClaimLimitExceeded);
        }

        let token = token::Client::new(&env, &token_id);
        let current = env.current_contract_address();
        let faucet_balance = token.balance(&current);
        if faucet_balance < amount {
            return Err(Error::FaucetDepleted);
        }

        token.transfer(&current, &user, &amount);

        record.amount_claimed += amount;
        record.last_claim_time = now;
        record.total_claims += 1;
        set_claim_record(&env, &user, &record);

        Ok(amount)
    }

    pub fn get_claim_info(env: Env, user: Address) -> ClaimRecord {
        let token_id = get_usdc_token(&env);
        get_claim_record(&env, &user, &token_id)
    }

    pub fn get_next_claim_time(env: Env, user: Address) -> u64 {
        let token_id = get_usdc_token(&env);
        let record = get_claim_record(&env, &user, &token_id);
        if record.last_claim_time == 0 {
            0
        } else {
            record.last_claim_time + get_cooldown_secs(&env)
        }
    }

    pub fn pause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        set_paused(&env, true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        set_paused(&env, false);
        Ok(())
    }

    pub fn get_usdc_token(env: Env) -> Address {
        storage::get_usdc_token(&env)
    }
}
