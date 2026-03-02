#![no_std]

mod storage;
mod price;
mod validation;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Symbol, Vec};
use shared::{Error, PriceData, PriceFeed};

use storage::*;
use validation::*;

#[contract]
pub struct OracleManager;

#[contractimpl]
impl OracleManager {
    /// Initialize the oracle manager
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if is_initialized(&env) {
            return Err(Error::Unauthorized);
        }

        admin.require_auth();

        set_admin(&env, &admin);
        set_initialized(&env, true);
        set_paused(&env, false);

        extend_instance_ttl(&env);

        Ok(())
    }

    /// Register a new price feed
    pub fn register_feed(
        env: Env,
        asset: Symbol,
        pyth_feed_id: Bytes,
        max_staleness: u64,
        max_deviation_bps: u32,
    ) -> Result<(), Error> {
        require_admin(&env)?;

        let feed = PriceFeed {
            asset: asset.clone(),
            pyth_feed_id,
            max_staleness,
            max_deviation_bps,
            circuit_breaker_enabled: false,
        };

        set_price_feed(&env, &asset, &feed);
        extend_instance_ttl(&env);

        Ok(())
    }

    /// Update price for an asset
    pub fn update_price(
        env: Env,
        asset: Symbol,
        price: i128,
        confidence: u32,
        expo: i32,
    ) -> Result<(), Error> {
        require_not_paused(&env)?;

        shared::validate_price(price)?;

        let timestamp = env.ledger().timestamp();

        let price_data = PriceData {
            asset: asset.clone(),
            price,
            confidence,
            timestamp,
            expo,
        };

        // Validate against existing price if available
        if let Some(existing) = get_price(&env, &asset) {
            let feed = get_price_feed(&env, &asset)?;
            
            if !feed.circuit_breaker_enabled {
                validate_price_update(&existing, &price_data, &feed)?;
            }
        }

        set_price(&env, &asset, &price_data);
        extend_instance_ttl(&env);

        Ok(())
    }

    /// Get current price for an asset
    pub fn get_price(env: Env, asset: Symbol) -> Result<PriceData, Error> {
        require_not_paused(&env)?;

        let price_data = get_price(&env, &asset).ok_or(Error::OracleUnavailable)?;
        let feed = get_price_feed(&env, &asset)?;

        // Validate freshness
        let current_time = env.ledger().timestamp();
        shared::validate_price_freshness(price_data.timestamp, current_time, feed.max_staleness)?;

        Ok(price_data)
    }

    /// Get multiple prices at once
    pub fn get_prices(env: Env, assets: Vec<Symbol>) -> Result<Vec<PriceData>, Error> {
        require_not_paused(&env)?;

        let mut prices = Vec::new(&env);

        for i in 0..assets.len() {
            let asset = assets.get(i).unwrap();
            let price = Self::get_price(env.clone(), asset)?;
            prices.push_back(price);
        }

        Ok(prices)
    }

    /// Enable/disable circuit breaker for an asset
    pub fn set_circuit_breaker(env: Env, asset: Symbol, enabled: bool) -> Result<(), Error> {
        require_admin(&env)?;

        let mut feed = get_price_feed(&env, &asset)?;
        feed.circuit_breaker_enabled = enabled;
        set_price_feed(&env, &asset, &feed);

        extend_instance_ttl(&env);

        Ok(())
    }

    /// Update price feed configuration
    pub fn update_feed_config(
        env: Env,
        asset: Symbol,
        max_staleness: u64,
        max_deviation_bps: u32,
    ) -> Result<(), Error> {
        require_admin(&env)?;

        let mut feed = get_price_feed(&env, &asset)?;
        feed.max_staleness = max_staleness;
        feed.max_deviation_bps = max_deviation_bps;
        set_price_feed(&env, &asset, &feed);

        extend_instance_ttl(&env);

        Ok(())
    }

    /// Pause the oracle manager
    pub fn pause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        set_paused(&env, true);
        Ok(())
    }

    /// Unpause the oracle manager
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
