#![no_std]

mod storage;

#[cfg(test)]
mod test;

use shared::{
    calculate_lp_tokens, calculate_utilization_rate, calculate_withdrawal_amount,
    validate_positive_amount, Error, PoolInfo,
};
use soroban_sdk::{contract, contractimpl, token, Address, Env};
use storage::*;

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    /// Initialize the vault with configuration
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        deposit_fee_bps: u32,
        withdraw_fee_bps: u32,
    ) -> Result<(), Error> {
        if is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        if deposit_fee_bps > 1000 || withdraw_fee_bps > 1000 {
            return Err(Error::InvalidParameter);
        }

        set_admin(&env, &admin);
        set_token(&env, &token);
        set_deposit_fee_bps(&env, deposit_fee_bps);
        set_withdraw_fee_bps(&env, withdraw_fee_bps);
        set_initialized(&env, true);
        set_paused(&env, false);

        // Initialize pool state
        let pool = PoolInfo {
            token: token.clone(),
            total_liquidity: 0,
            available_liquidity: 0,
            reserved_liquidity: 0,
            total_lp_tokens: 0,
            accumulated_fees: 0,
            utilization_rate: 0,
        };
        set_pool(&env, &pool);

        extend_instance_ttl(&env);
        Ok(())
    }

    /// Deposit tokens and receive LP tokens
    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<i128, Error> {
        require_not_paused(&env)?;
        user.require_auth();
        validate_positive_amount(amount)?;

        let token_addr = get_token(&env);
        let fee_bps = get_deposit_fee_bps(&env);
        let fee = amount
            .checked_mul(fee_bps as i128)
            .and_then(|v| v.checked_div(10000))
            .ok_or(Error::Overflow)?;
        let net_amount = amount.checked_sub(fee).ok_or(Error::Overflow)?;

        let mut pool = get_pool(&env);
        let minted = calculate_lp_tokens(net_amount, pool.total_liquidity, pool.total_lp_tokens)?;

        // Transfer tokens from user to vault
        let token = token::Client::new(&env, &token_addr);
        token.transfer(&user, &env.current_contract_address(), &amount);

        // Update pool state
        pool.total_liquidity = pool
            .total_liquidity
            .checked_add(net_amount)
            .ok_or(Error::Overflow)?;
        pool.available_liquidity = pool
            .available_liquidity
            .checked_add(net_amount)
            .ok_or(Error::Overflow)?;
        pool.total_lp_tokens = pool
            .total_lp_tokens
            .checked_add(minted)
            .ok_or(Error::Overflow)?;
        pool.accumulated_fees = pool
            .accumulated_fees
            .checked_add(fee)
            .ok_or(Error::Overflow)?;
        pool.utilization_rate =
            calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &pool);

        // Update user LP balance
        let balance = get_lp_balance(&env, &user)
            .checked_add(minted)
            .ok_or(Error::Overflow)?;
        set_lp_balance(&env, &user, balance);

        extend_instance_ttl(&env);
        Ok(minted)
    }

    /// Withdraw tokens by burning LP tokens
    pub fn withdraw(env: Env, user: Address, lp_tokens: i128) -> Result<i128, Error> {
        require_not_paused(&env)?;
        user.require_auth();
        validate_positive_amount(lp_tokens)?;

        let user_balance = get_lp_balance(&env, &user);
        if user_balance < lp_tokens {
            return Err(Error::InsufficientBalance);
        }

        let mut pool = get_pool(&env);
        let gross_withdrawal =
            calculate_withdrawal_amount(lp_tokens, pool.total_liquidity, pool.total_lp_tokens)?;

        if pool.available_liquidity < gross_withdrawal {
            return Err(Error::InsufficientLiquidity);
        }

        let fee_bps = get_withdraw_fee_bps(&env);
        let fee = gross_withdrawal
            .checked_mul(fee_bps as i128)
            .and_then(|v| v.checked_div(10000))
            .ok_or(Error::Overflow)?;
        let net_withdrawal = gross_withdrawal.checked_sub(fee).ok_or(Error::Overflow)?;

        // Transfer tokens to user
        let token_addr = get_token(&env);
        let token = token::Client::new(&env, &token_addr);
        token.transfer(&env.current_contract_address(), &user, &net_withdrawal);

        // Update pool state
        pool.total_liquidity = pool
            .total_liquidity
            .checked_sub(gross_withdrawal)
            .ok_or(Error::Overflow)?;
        pool.available_liquidity = pool
            .available_liquidity
            .checked_sub(gross_withdrawal)
            .ok_or(Error::Overflow)?;
        pool.total_lp_tokens = pool
            .total_lp_tokens
            .checked_sub(lp_tokens)
            .ok_or(Error::Overflow)?;
        pool.accumulated_fees = pool
            .accumulated_fees
            .checked_add(fee)
            .ok_or(Error::Overflow)?;
        pool.utilization_rate =
            calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &pool);

        // Update user LP balance
        set_lp_balance(&env, &user, user_balance - lp_tokens);

        extend_instance_ttl(&env);
        Ok(net_withdrawal)
    }

    /// Reserve liquidity for trading (admin only)
    pub fn reserve_liquidity(env: Env, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;
        validate_positive_amount(amount)?;

        let mut pool = get_pool(&env);
        if pool.available_liquidity < amount {
            return Err(Error::InsufficientLiquidity);
        }

        pool.available_liquidity = pool
            .available_liquidity
            .checked_sub(amount)
            .ok_or(Error::Overflow)?;
        pool.reserved_liquidity = pool
            .reserved_liquidity
            .checked_add(amount)
            .ok_or(Error::Overflow)?;
        pool.utilization_rate =
            calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &pool);

        Ok(())
    }

    /// Release reserved liquidity (admin only)
    pub fn release_liquidity(env: Env, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;
        validate_positive_amount(amount)?;

        let mut pool = get_pool(&env);
        if pool.reserved_liquidity < amount {
            return Err(Error::InsufficientLiquidity);
        }

        pool.reserved_liquidity = pool
            .reserved_liquidity
            .checked_sub(amount)
            .ok_or(Error::Overflow)?;
        pool.available_liquidity = pool
            .available_liquidity
            .checked_add(amount)
            .ok_or(Error::Overflow)?;
        pool.utilization_rate =
            calculate_utilization_rate(pool.reserved_liquidity, pool.total_liquidity)?;
        set_pool(&env, &pool);

        Ok(())
    }

    /// Get pool information
    pub fn get_pool_info(env: Env) -> Result<PoolInfo, Error> {
        require_initialized(&env)?;
        Ok(get_pool(&env))
    }

    /// Get LP token balance for user
    pub fn get_lp_balance(env: Env, user: Address) -> i128 {
        get_lp_balance(&env, &user)
    }

    /// Get LP token price (liquidity per LP token)
    pub fn get_lp_price(env: Env) -> Result<i128, Error> {
        require_initialized(&env)?;
        let pool = get_pool(&env);
        if pool.total_lp_tokens == 0 {
            return Ok(10_000_000); // 1:1 ratio with 7 decimals
        }
        pool.total_liquidity
            .checked_mul(10_000_000)
            .and_then(|v| v.checked_div(pool.total_lp_tokens))
            .ok_or(Error::Overflow)
    }

    /// Pause the vault (admin only)
    pub fn pause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        set_paused(&env, true);
        Ok(())
    }

    /// Unpause the vault (admin only)
    pub fn unpause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        set_paused(&env, false);
        Ok(())
    }

    /// Check if paused
    pub fn is_paused(env: Env) -> bool {
        get_paused(&env)
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        require_initialized(&env)?;
        Ok(get_admin(&env))
    }

    /// Transfer admin role
    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        require_admin(&env)?;
        new_admin.require_auth();
        set_admin(&env, &new_admin);
        extend_instance_ttl(&env);
        Ok(())
    }
}
