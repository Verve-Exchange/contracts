#![no_std]

mod storage;

#[cfg(test)]
mod test;

use shared::{
    should_liquidate, validate_collateral, validate_leverage, AssetClass, Error, PortfolioRisk,
    RiskParameters, BASIS_POINTS, MAX_LEVERAGE,
};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};
use storage::*;

#[contract]
pub struct RiskManager;

#[contractimpl]
impl RiskManager {
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

    pub fn update_risk_parameters(
        env: Env,
        asset: Symbol,
        asset_class: AssetClass,
        min_margin_ratio: u32,
        maintenance_margin_ratio: u32,
        liquidation_threshold: u32,
        max_leverage: u32,
        max_position_size: i128,
        max_open_interest: i128,
    ) -> Result<(), Error> {
        require_admin(&env)?;
        if min_margin_ratio > BASIS_POINTS || maintenance_margin_ratio > BASIS_POINTS {
            return Err(Error::InvalidParameter);
        }
        if max_leverage > MAX_LEVERAGE {
            return Err(Error::InvalidLeverage);
        }

        let params = RiskParameters {
            asset,
            asset_class,
            min_margin_ratio,
            maintenance_margin_ratio,
            liquidation_threshold,
            max_leverage,
            max_position_size,
            max_open_interest,
        };
        set_risk_params(&env, &params);
        Ok(())
    }

    pub fn assess_position_risk(
        env: Env,
        user: Address,
        asset: Symbol,
        position_size: i128,
        collateral: i128,
        leverage: u32,
    ) -> Result<PortfolioRisk, Error> {
        require_not_paused(&env)?;
        let params = get_risk_params(&env, &asset)?;

        validate_collateral(collateral)?;
        validate_leverage(leverage, params.max_leverage)?;
        if position_size > params.max_position_size {
            return Err(Error::PositionTooLarge);
        }

        let margin_used = position_size
            .checked_mul(params.min_margin_ratio as i128)
            .and_then(|v| v.checked_div(BASIS_POINTS as i128))
            .ok_or(Error::Overflow)?;

        if collateral < margin_used {
            return Err(Error::InsufficientMargin);
        }

        let margin_available = collateral.checked_sub(margin_used).ok_or(Error::Overflow)?;
        let risk_score = if margin_available == 0 {
            10000
        } else {
            ((margin_used * 10000) / collateral) as u32
        };

        Ok(PortfolioRisk {
            user,
            total_exposure: position_size,
            margin_used,
            margin_available,
            risk_score,
            liquidation_distance: margin_available,
        })
    }

    pub fn check_liquidation_threshold(
        env: Env,
        asset: Symbol,
        collateral: i128,
        unrealized_pnl: i128,
        funding_accumulated: i128,
        position_size: i128,
    ) -> Result<bool, Error> {
        require_not_paused(&env)?;
        let params = get_risk_params(&env, &asset)?;

        let maintenance_margin = position_size
            .checked_mul(params.maintenance_margin_ratio as i128)
            .and_then(|v| v.checked_div(BASIS_POINTS as i128))
            .ok_or(Error::Overflow)?;

        Ok(should_liquidate(
            collateral,
            unrealized_pnl,
            funding_accumulated,
            maintenance_margin,
        ))
    }

    pub fn calculate_margin_requirement(
        env: Env,
        asset: Symbol,
        position_size: i128,
        volatility_bps: u32,
    ) -> Result<i128, Error> {
        require_not_paused(&env)?;
        let params = get_risk_params(&env, &asset)?;

        let base_margin = position_size
            .checked_mul(params.min_margin_ratio as i128)
            .and_then(|v| v.checked_div(BASIS_POINTS as i128))
            .ok_or(Error::Overflow)?;

        let volatility_multiplier = (BASIS_POINTS + volatility_bps) as i128;
        base_margin
            .checked_mul(volatility_multiplier)
            .and_then(|v| v.checked_div(BASIS_POINTS as i128))
            .ok_or(Error::Overflow)
    }

    pub fn get_max_leverage(env: Env, asset: Symbol, volatility_bps: u32) -> Result<u32, Error> {
        let params = get_risk_params(&env, &asset)?;
        if volatility_bps >= BASIS_POINTS {
            return Ok(1);
        }
        let scale = BASIS_POINTS - volatility_bps;
        let adjusted = (params.max_leverage as u64 * scale as u64) / BASIS_POINTS as u64;
        Ok(if adjusted == 0 { 1 } else { adjusted as u32 })
    }

    pub fn get_risk_parameters(env: Env, asset: Symbol) -> Result<RiskParameters, Error> {
        get_risk_params(&env, &asset)
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
