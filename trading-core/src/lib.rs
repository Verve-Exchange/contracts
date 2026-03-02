#![no_std]

mod storage;

#[cfg(test)]
mod test;

use shared::{
    calculate_fee, calculate_liquidation_price, calculate_pnl, calculate_position_size,
    check_slippage, validate_collateral, validate_leverage, validate_positive_amount, Direction,
    Error, Order, OrderStatus, OrderType, Position, PositionStatus, MAX_LEVERAGE,
};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};
use storage::*;

#[contract]
pub struct TradingCore;

#[contractimpl]
impl TradingCore {
    pub fn initialize(
        env: Env,
        admin: Address,
        trading_fee_bps: u32,
        maintenance_margin_bps: u32,
    ) -> Result<(), Error> {
        if is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        set_admin(&env, &admin);
        set_initialized(&env, true);
        set_paused(&env, false);
        set_trading_fee_bps(&env, trading_fee_bps);
        set_maintenance_margin_bps(&env, maintenance_margin_bps);
        extend_instance_ttl(&env);
        Ok(())
    }

    pub fn open_position(
        env: Env,
        trader: Address,
        asset: Symbol,
        collateral_token: Address,
        collateral_amount: i128,
        leverage: u32,
        direction: Direction,
        entry_price: i128,
    ) -> Result<u64, Error> {
        require_not_paused(&env)?;
        trader.require_auth();

        validate_collateral(collateral_amount)?;
        validate_leverage(leverage, MAX_LEVERAGE)?;
        validate_positive_amount(entry_price)?;

        let position_size = calculate_position_size(collateral_amount, leverage)?;
        let liquidation_price = calculate_liquidation_price(
            entry_price,
            leverage,
            direction,
            get_maintenance_margin_bps(&env),
        )?;

        let id = next_position_id(&env);
        let now = env.ledger().timestamp();

        let position = Position {
            id,
            trader: trader.clone(),
            asset: asset.clone(),
            collateral_token,
            collateral_amount,
            position_size,
            entry_price,
            leverage,
            direction,
            liquidation_price,
            unrealized_pnl: 0,
            funding_accumulated: 0,
            opened_at: now,
            last_updated: now,
            status: PositionStatus::Open,
        };

        set_position(&env, &position);
        add_user_position(&env, &trader, id);

        let mut stats = get_market_stats(&env, &asset);
        stats.open_interest = stats
            .open_interest
            .checked_add(position_size)
            .ok_or(Error::Overflow)?;
        match direction {
            Direction::Long => {
                stats.total_long_size = stats
                    .total_long_size
                    .checked_add(position_size)
                    .ok_or(Error::Overflow)?
            }
            Direction::Short => {
                stats.total_short_size = stats
                    .total_short_size
                    .checked_add(position_size)
                    .ok_or(Error::Overflow)?
            }
        }
        set_market_stats(&env, &stats);
        extend_instance_ttl(&env);

        Ok(id)
    }

    pub fn close_position(
        env: Env,
        trader: Address,
        position_id: u64,
        exit_price: i128,
    ) -> Result<i128, Error> {
        require_not_paused(&env)?;
        trader.require_auth();

        let mut position = get_position(&env, position_id)?;
        if position.trader != trader {
            return Err(Error::NotPositionOwner);
        }
        if position.status != PositionStatus::Open {
            return Err(Error::PositionAlreadyClosed);
        }

        let pnl = calculate_pnl(
            position.position_size,
            position.entry_price,
            exit_price,
            position.direction,
        )?;
        let fee = calculate_fee(position.position_size, get_trading_fee_bps(&env))?;
        let realized_pnl = pnl.checked_sub(fee).ok_or(Error::Overflow)?;

        position.unrealized_pnl = realized_pnl;
        position.status = PositionStatus::Closed;
        position.last_updated = env.ledger().timestamp();
        set_position(&env, &position);

        let mut stats = get_market_stats(&env, &position.asset);
        stats.open_interest = stats
            .open_interest
            .checked_sub(position.position_size)
            .ok_or(Error::Overflow)?;
        match position.direction {
            Direction::Long => {
                stats.total_long_size = stats
                    .total_long_size
                    .checked_sub(position.position_size)
                    .ok_or(Error::Overflow)?
            }
            Direction::Short => {
                stats.total_short_size = stats
                    .total_short_size
                    .checked_sub(position.position_size)
                    .ok_or(Error::Overflow)?
            }
        }
        set_market_stats(&env, &stats);
        extend_instance_ttl(&env);

        Ok(realized_pnl)
    }

    pub fn place_order(
        env: Env,
        trader: Address,
        asset: Symbol,
        order_type: OrderType,
        direction: Direction,
        trigger_price: i128,
        size: i128,
        collateral: i128,
        leverage: u32,
        slippage_tolerance: u32,
        linked_position_id: u64,
    ) -> Result<u64, Error> {
        require_not_paused(&env)?;
        trader.require_auth();
        validate_positive_amount(trigger_price)?;
        validate_positive_amount(size)?;
        validate_collateral(collateral)?;
        validate_leverage(leverage, MAX_LEVERAGE)?;

        let id = next_order_id(&env);
        let order = Order {
            id,
            trader: trader.clone(),
            asset,
            order_type,
            direction,
            trigger_price,
            size,
            collateral,
            leverage,
            slippage_tolerance,
            created_at: env.ledger().timestamp(),
            status: OrderStatus::Pending,
            linked_position_id,
        };

        set_order(&env, &order);
        add_user_order(&env, &trader, id);
        extend_instance_ttl(&env);

        Ok(id)
    }

    pub fn cancel_order(env: Env, trader: Address, order_id: u64) -> Result<(), Error> {
        require_not_paused(&env)?;
        trader.require_auth();

        let mut order = get_order(&env, order_id)?;
        if order.trader != trader {
            return Err(Error::NotOrderOwner);
        }
        if order.status != OrderStatus::Pending {
            return Err(Error::OrderNotPending);
        }

        order.status = OrderStatus::Cancelled;
        set_order(&env, &order);
        Ok(())
    }

    pub fn execute_order(env: Env, order_id: u64, execution_price: i128) -> Result<u64, Error> {
        require_not_paused(&env)?;

        let mut order = get_order(&env, order_id)?;
        if order.status != OrderStatus::Pending {
            return Err(Error::OrderNotPending);
        }

        check_slippage(order.trigger_price, execution_price, order.slippage_tolerance)?;

        match order.order_type {
            OrderType::LimitEntry => {
                let position_id = Self::open_position(
                    env.clone(),
                    order.trader.clone(),
                    order.asset.clone(),
                    order.trader.clone(),
                    order.collateral,
                    order.leverage,
                    order.direction,
                    execution_price,
                )?;
                order.status = OrderStatus::Executed;
                set_order(&env, &order);
                Ok(position_id)
            }
            OrderType::StopLoss | OrderType::TakeProfit => {
                let linked = order.linked_position_id;
                if linked == 0 {
                    return Err(Error::InvalidParameter);
                }
                Self::close_position(env.clone(), order.trader.clone(), linked, execution_price)?;
                order.status = OrderStatus::Executed;
                set_order(&env, &order);
                Ok(linked)
            }
        }
    }

    pub fn liquidate_position(
        env: Env,
        position_id: u64,
        liquidation_price: i128,
    ) -> Result<(), Error> {
        require_not_paused(&env)?;

        let mut position = get_position(&env, position_id)?;
        if position.status != PositionStatus::Open {
            return Err(Error::PositionAlreadyClosed);
        }

        let should_liquidate = match position.direction {
            Direction::Long => liquidation_price <= position.liquidation_price,
            Direction::Short => liquidation_price >= position.liquidation_price,
        };

        if !should_liquidate {
            return Err(Error::NotLiquidatable);
        }

        position.status = PositionStatus::Liquidated;
        position.last_updated = env.ledger().timestamp();
        set_position(&env, &position);

        let mut stats = get_market_stats(&env, &position.asset);
        stats.open_interest = stats
            .open_interest
            .checked_sub(position.position_size)
            .ok_or(Error::Overflow)?;
        match position.direction {
            Direction::Long => {
                stats.total_long_size = stats
                    .total_long_size
                    .checked_sub(position.position_size)
                    .ok_or(Error::Overflow)?
            }
            Direction::Short => {
                stats.total_short_size = stats
                    .total_short_size
                    .checked_sub(position.position_size)
                    .ok_or(Error::Overflow)?
            }
        }
        set_market_stats(&env, &stats);

        Ok(())
    }

    pub fn get_position(env: Env, position_id: u64) -> Result<Position, Error> {
        get_position(&env, position_id)
    }

    pub fn get_user_positions(env: Env, trader: Address) -> Result<Vec<Position>, Error> {
        let ids = get_user_position_ids(&env, &trader);
        let mut out = Vec::new(&env);
        for i in 0..ids.len() {
            let id = ids.get(i).unwrap();
            out.push_back(get_position(&env, id)?);
        }
        Ok(out)
    }

    pub fn get_order(env: Env, order_id: u64) -> Result<Order, Error> {
        get_order(&env, order_id)
    }

    pub fn get_user_orders(env: Env, trader: Address) -> Result<Vec<Order>, Error> {
        let ids = get_user_order_ids(&env, &trader);
        let mut out = Vec::new(&env);
        for i in 0..ids.len() {
            let id = ids.get(i).unwrap();
            out.push_back(get_order(&env, id)?);
        }
        Ok(out)
    }

    pub fn get_market_stats(env: Env, asset: Symbol) -> shared::MarketStats {
        get_market_stats(&env, &asset)
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
