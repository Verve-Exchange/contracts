#![no_std]

mod storage;

#[cfg(test)]
mod test;

use shared::{
    calculate_lp_tokens, calculate_utilization_rate, calculate_withdrawal_amount, validate_positive_amount,
    Error, LPStake, PoolInfo,
};
use soroban_sdk::{contract, contractimpl, Address, Env};
use storage::*;

#[contract]
pub struct LiquidityManager;

#[contractimpl]
impl LiquidityManager {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        set_admin(&env, &admin);
        set_initialized(&env, true);
        set_paused(&env, false);
        extend_instance_ttl(&env);
        Ok(())
    }

    pub fn create_pool(env: Env, token: Address) -> Result<(), Error> {
        require_admin(&env)?;
        let pool = PoolInfo {
            token: token.clone(),
            total_liquidity: 0,
            available_liquidity: 0,
            reserved_liquidity: 0,
            total_lp_tokens: 0,
            accumulated_fees: 0,
            utilization_rate: 0,
        };
        set_pool(&env, &token, &pool);
        Ok(())
    }

    pub fn deposit(env: Env, user: Address, token: Address, amount: i128) -> Result<i128, Error> {
        require_not_paused(&env)?;
        user.require_auth();
        validate_positive_amount(amount)?;

        let mut pool = get_pool(&env, &token)?;
        let minted = calculate_lp_tokens(amount, pool.total_liquidity, pool.total_lp_tokens)?;

        pool.total_liquidity = pool.total_liquidity.checked_add(amount).ok_or(Error::Overflow)?;
        pool.available_liquidity = pool.available_liquidity.checked_add(amount).ok_or(Error::Overflow)?;
        pool.total_lp_tokens = pool.total_lp_tokens.checked_add(minted).ok_or(Error::Overflow)?;
        pool.utilization_rate = calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &token, &pool);

        let balance = get_lp_balance(&env, &user, &token)
            .checked_add(minted)
            .ok_or(Error::Overflow)?;
        set_lp_balance(&env, &user, &token, balance);

        Ok(minted)
    }

    pub fn withdraw(env: Env, user: Address, token: Address, lp_tokens: i128) -> Result<i128, Error> {
        require_not_paused(&env)?;
        user.require_auth();
        validate_positive_amount(lp_tokens)?;

        let mut pool = get_pool(&env, &token)?;
        let user_balance = get_lp_balance(&env, &user, &token);
        if user_balance < lp_tokens {
            return Err(Error::InsufficientBalance);
        }

        let withdrawal = calculate_withdrawal_amount(lp_tokens, pool.total_liquidity, pool.total_lp_tokens)?;
        if pool.available_liquidity < withdrawal {
            return Err(Error::InsufficientLiquidity);
        }

        pool.total_liquidity = pool.total_liquidity.checked_sub(withdrawal).ok_or(Error::Overflow)?;
        pool.available_liquidity = pool.available_liquidity.checked_sub(withdrawal).ok_or(Error::Overflow)?;
        pool.total_lp_tokens = pool.total_lp_tokens.checked_sub(lp_tokens).ok_or(Error::Overflow)?;
        pool.utilization_rate = calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &token, &pool);

        set_lp_balance(&env, &user, &token, user_balance - lp_tokens);
        Ok(withdrawal)
    }

    pub fn reserve_liquidity(env: Env, token: Address, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;
        validate_positive_amount(amount)?;

        let mut pool = get_pool(&env, &token)?;
        if pool.available_liquidity < amount {
            return Err(Error::InsufficientLiquidity);
        }

        pool.available_liquidity = pool.available_liquidity.checked_sub(amount).ok_or(Error::Overflow)?;
        pool.reserved_liquidity = pool.reserved_liquidity.checked_add(amount).ok_or(Error::Overflow)?;
        pool.utilization_rate = calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &token, &pool);
        Ok(())
    }

    pub fn release_liquidity(env: Env, token: Address, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;
        validate_positive_amount(amount)?;

        let mut pool = get_pool(&env, &token)?;
        if pool.reserved_liquidity < amount {
            return Err(Error::InsufficientLiquidity);
        }

        pool.reserved_liquidity = pool.reserved_liquidity.checked_sub(amount).ok_or(Error::Overflow)?;
        pool.available_liquidity = pool.available_liquidity.checked_add(amount).ok_or(Error::Overflow)?;
        pool.utilization_rate = calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &token, &pool);
        Ok(())
    }

    pub fn settle_trader_pnl(env: Env, token: Address, pnl: i128) -> Result<(), Error> {
        require_admin(&env)?;
        let mut pool = get_pool(&env, &token)?;

        if pnl > 0 {
            if pool.available_liquidity < pnl {
                return Err(Error::InsufficientLiquidity);
            }
            pool.available_liquidity = pool.available_liquidity.checked_sub(pnl).ok_or(Error::Overflow)?;
            pool.total_liquidity = pool.total_liquidity.checked_sub(pnl).ok_or(Error::Overflow)?;
        } else if pnl < 0 {
            let gain = -pnl;
            pool.available_liquidity = pool.available_liquidity.checked_add(gain).ok_or(Error::Overflow)?;
            pool.total_liquidity = pool.total_liquidity.checked_add(gain).ok_or(Error::Overflow)?;
        }

        pool.utilization_rate = calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &token, &pool);
        Ok(())
    }

    pub fn stake_lp_tokens(
        env: Env,
        user: Address,
        token: Address,
        amount: i128,
        lock_period: u64,
    ) -> Result<(), Error> {
        require_not_paused(&env)?;
        user.require_auth();
        validate_positive_amount(amount)?;

        let user_balance = get_lp_balance(&env, &user, &token);
        if user_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        set_lp_balance(&env, &user, &token, user_balance - amount);
        let mut stake = get_stake(&env, &user, &token).unwrap_or(LPStake {
            user: user.clone(),
            staked_amount: 0,
            rewards_earned: 0,
            stake_timestamp: env.ledger().timestamp(),
            lock_period,
            boost_multiplier: if lock_period >= 180 * 24 * 3600 {
                20000
            } else if lock_period >= 90 * 24 * 3600 {
                15000
            } else if lock_period >= 30 * 24 * 3600 {
                12000
            } else {
                10000
            },
        });

        stake.staked_amount = stake.staked_amount.checked_add(amount).ok_or(Error::Overflow)?;
        stake.stake_timestamp = env.ledger().timestamp();
        stake.lock_period = lock_period;
        set_stake(&env, &user, &token, &stake);
        Ok(())
    }

    pub fn unstake_lp_tokens(env: Env, user: Address, token: Address, amount: i128) -> Result<(), Error> {
        require_not_paused(&env)?;
        user.require_auth();
        validate_positive_amount(amount)?;

        let mut stake = get_stake(&env, &user, &token).ok_or(Error::InvalidParameter)?;
        if stake.staked_amount < amount {
            return Err(Error::InsufficientBalance);
        }
        if env.ledger().timestamp() < stake.stake_timestamp + stake.lock_period {
            return Err(Error::Unauthorized);
        }

        stake.staked_amount = stake.staked_amount.checked_sub(amount).ok_or(Error::Overflow)?;
        set_stake(&env, &user, &token, &stake);

        let user_balance = get_lp_balance(&env, &user, &token)
            .checked_add(amount)
            .ok_or(Error::Overflow)?;
        set_lp_balance(&env, &user, &token, user_balance);

        Ok(())
    }

    pub fn get_pool_info(env: Env, token: Address) -> Result<PoolInfo, Error> {
        get_pool(&env, &token)
    }

    pub fn get_user_stake_info(env: Env, user: Address, token: Address) -> Option<LPStake> {
        get_stake(&env, &user, &token)
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
}
