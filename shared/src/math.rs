use crate::{constants::*, errors::Error, types::Direction};

/// Calculate position size from collateral and leverage
pub fn calculate_position_size(collateral: i128, leverage: u32) -> Result<i128, Error> {
    collateral
        .checked_mul(leverage as i128)
        .ok_or(Error::Overflow)
}

/// Calculate liquidation price for a position
pub fn calculate_liquidation_price(
    entry_price: i128,
    leverage: u32,
    direction: Direction,
    maintenance_margin_bps: u32,
) -> Result<i128, Error> {
    if leverage == 0 {
        return Err(Error::InvalidLeverage);
    }

    let leverage_i128 = leverage as i128;
    let maintenance_margin = (maintenance_margin_bps as i128 * PRECISION) / BASIS_POINTS as i128;

    match direction {
        Direction::Long => {
            // liq_price = entry_price * (1 - 1/leverage + maintenance_margin)
            let factor = PRECISION - (PRECISION / leverage_i128) + maintenance_margin;
            entry_price
                .checked_mul(factor)
                .and_then(|v| v.checked_div(PRECISION))
                .ok_or(Error::Overflow)
        }
        Direction::Short => {
            // liq_price = entry_price * (1 + 1/leverage - maintenance_margin)
            let factor = PRECISION + (PRECISION / leverage_i128) - maintenance_margin;
            entry_price
                .checked_mul(factor)
                .and_then(|v| v.checked_div(PRECISION))
                .ok_or(Error::Overflow)
        }
    }
}

/// Calculate PnL for a position
pub fn calculate_pnl(
    position_size: i128,
    entry_price: i128,
    exit_price: i128,
    direction: Direction,
) -> Result<i128, Error> {
    if entry_price == 0 {
        return Err(Error::DivisionByZero);
    }

    let price_diff = match direction {
        Direction::Long => exit_price.checked_sub(entry_price),
        Direction::Short => entry_price.checked_sub(exit_price),
    }
    .ok_or(Error::Overflow)?;

    position_size
        .checked_mul(price_diff)
        .and_then(|v| v.checked_div(entry_price))
        .ok_or(Error::Overflow)
}

/// Calculate trading fee
pub fn calculate_fee(amount: i128, fee_bps: u32) -> Result<i128, Error> {
    amount
        .checked_mul(fee_bps as i128)
        .and_then(|v| v.checked_div(BASIS_POINTS as i128))
        .ok_or(Error::Overflow)
}

/// Calculate funding rate based on long/short imbalance
pub fn calculate_funding_rate(
    total_long: i128,
    total_short: i128,
    base_rate_bps: u32,
) -> Result<i128, Error> {
    if total_long == 0 && total_short == 0 {
        return Ok(0);
    }

    let total = total_long.checked_add(total_short).ok_or(Error::Overflow)?;
    if total == 0 {
        return Ok(0);
    }

    let imbalance = total_long.checked_sub(total_short).ok_or(Error::Overflow)?;
    let base_rate = base_rate_bps as i128;

    imbalance
        .checked_mul(base_rate)
        .and_then(|v| v.checked_div(total))
        .ok_or(Error::Overflow)
}

/// Calculate funding payment for a position
pub fn calculate_funding_payment(
    position_size: i128,
    funding_rate: i128,
    hours_elapsed: u64,
) -> Result<i128, Error> {
    position_size
        .checked_mul(funding_rate)
        .and_then(|v| v.checked_mul(hours_elapsed as i128))
        .and_then(|v| v.checked_div(BASIS_POINTS as i128))
        .ok_or(Error::Overflow)
}

/// Check if position should be liquidated
pub fn should_liquidate(
    collateral: i128,
    unrealized_pnl: i128,
    funding_accumulated: i128,
    maintenance_margin: i128,
) -> bool {
    let equity = match collateral
        .checked_add(unrealized_pnl)
        .and_then(|v| v.checked_sub(funding_accumulated))
    {
        Some(v) => v,
        None => return true, // Overflow means liquidation
    };

    equity < maintenance_margin
}

/// Calculate LP tokens to mint for deposit
pub fn calculate_lp_tokens(
    deposit_amount: i128,
    total_liquidity: i128,
    total_lp_tokens: i128,
) -> Result<i128, Error> {
    if total_lp_tokens == 0 || total_liquidity == 0 {
        // First deposit: 1:1 ratio
        return Ok(deposit_amount);
    }

    deposit_amount
        .checked_mul(total_lp_tokens)
        .and_then(|v| v.checked_div(total_liquidity))
        .ok_or(Error::Overflow)
}

/// Calculate withdrawal amount for LP tokens
pub fn calculate_withdrawal_amount(
    lp_tokens: i128,
    total_liquidity: i128,
    total_lp_tokens: i128,
) -> Result<i128, Error> {
    if total_lp_tokens == 0 {
        return Err(Error::DivisionByZero);
    }

    lp_tokens
        .checked_mul(total_liquidity)
        .and_then(|v| v.checked_div(total_lp_tokens))
        .ok_or(Error::Overflow)
}

/// Calculate utilization rate
pub fn calculate_utilization_rate(
    reserved_liquidity: i128,
    total_liquidity: i128,
) -> Result<u32, Error> {
    if total_liquidity == 0 {
        return Ok(0);
    }

    let rate = reserved_liquidity
        .checked_mul(BASIS_POINTS as i128)
        .and_then(|v| v.checked_div(total_liquidity))
        .ok_or(Error::Overflow)?;

    Ok(rate as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_position_size() {
        let collateral = 100 * PRECISION; // 100 USDC
        let leverage = 10;
        let size = calculate_position_size(collateral, leverage).unwrap();
        assert_eq!(size, 1000 * PRECISION); // 1000 USDC
    }

    #[test]
    fn test_calculate_liquidation_price_long() {
        let entry_price = 1000 * PRECISION;
        let leverage = 10;
        let maintenance_margin_bps = 100; // 1%

        let liq_price =
            calculate_liquidation_price(entry_price, leverage, Direction::Long, maintenance_margin_bps)
                .unwrap();

        // Expected: 1000 * (1 - 0.1 + 0.01) = 1000 * 0.91 = 910
        assert_eq!(liq_price, 910 * PRECISION);
    }

    #[test]
    fn test_calculate_pnl_long_profit() {
        let position_size = 1000 * PRECISION;
        let entry_price = 1000 * PRECISION;
        let exit_price = 1100 * PRECISION; // 10% gain

        let pnl = calculate_pnl(position_size, entry_price, exit_price, Direction::Long).unwrap();

        // Expected: 1000 * (1100 - 1000) / 1000 = 100
        assert_eq!(pnl, 100 * PRECISION);
    }

    #[test]
    fn test_calculate_fee() {
        let amount = 1000 * PRECISION;
        let fee_bps = 10; // 0.1%

        let fee = calculate_fee(amount, fee_bps).unwrap();

        // Expected: 1000 * 0.001 = 1
        assert_eq!(fee, 1 * PRECISION);
    }

    #[test]
    fn test_calculate_lp_tokens_first_deposit() {
        let deposit = 1000 * PRECISION;
        let lp_tokens = calculate_lp_tokens(deposit, 0, 0).unwrap();
        assert_eq!(lp_tokens, deposit);
    }

    #[test]
    fn test_calculate_utilization_rate() {
        let reserved = 500 * PRECISION;
        let total = 1000 * PRECISION;
        let rate = calculate_utilization_rate(reserved, total).unwrap();
        assert_eq!(rate, 5000); // 50%
    }
}
