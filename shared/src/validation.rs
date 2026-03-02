use crate::{constants::*, errors::Error};

/// Validate leverage is within acceptable range
pub fn validate_leverage(leverage: u32, max_leverage: u32) -> Result<(), Error> {
    if leverage == 0 || leverage > max_leverage {
        return Err(Error::InvalidLeverage);
    }
    Ok(())
}

/// Validate collateral meets minimum requirement
pub fn validate_collateral(collateral: i128) -> Result<(), Error> {
    if collateral < MIN_COLLATERAL {
        return Err(Error::InsufficientCollateral);
    }
    Ok(())
}

/// Validate amount is positive
pub fn validate_positive_amount(amount: i128) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    Ok(())
}

/// Validate price is positive and reasonable
pub fn validate_price(price: i128) -> Result<(), Error> {
    if price <= 0 {
        return Err(Error::InvalidPrice);
    }
    Ok(())
}

/// Validate price staleness
pub fn validate_price_freshness(timestamp: u64, current_time: u64, max_staleness: u64) -> Result<(), Error> {
    if current_time < timestamp {
        // Clock skew protection
        return Ok(());
    }

    if current_time - timestamp > max_staleness {
        return Err(Error::PriceStale);
    }

    Ok(())
}

/// Validate price deviation between two sources
pub fn validate_price_deviation(price1: i128, price2: i128, max_deviation_bps: u32) -> Result<(), Error> {
    if price1 == 0 || price2 == 0 {
        return Err(Error::InvalidPrice);
    }

    let diff = if price1 > price2 {
        price1 - price2
    } else {
        price2 - price1
    };

    let avg = (price1 + price2) / 2;
    let deviation_bps = (diff * BASIS_POINTS as i128) / avg;

    if deviation_bps > max_deviation_bps as i128 {
        return Err(Error::PriceDeviationTooHigh);
    }

    Ok(())
}

/// Validate slippage tolerance
pub fn validate_slippage_tolerance(slippage_bps: u32) -> Result<(), Error> {
    if slippage_bps == 0 || slippage_bps > BASIS_POINTS {
        return Err(Error::InvalidSlippageTolerance);
    }
    Ok(())
}

/// Check if slippage is within tolerance
pub fn check_slippage(
    expected_price: i128,
    actual_price: i128,
    slippage_tolerance_bps: u32,
) -> Result<(), Error> {
    if expected_price == 0 {
        return Err(Error::InvalidPrice);
    }

    let diff = if actual_price > expected_price {
        actual_price - expected_price
    } else {
        expected_price - actual_price
    };

    let slippage_bps = (diff * BASIS_POINTS as i128) / expected_price;

    if slippage_bps > slippage_tolerance_bps as i128 {
        return Err(Error::SlippageExceeded);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_leverage() {
        assert!(validate_leverage(5, 10).is_ok());
        assert!(validate_leverage(0, 10).is_err());
        assert!(validate_leverage(11, 10).is_err());
    }

    #[test]
    fn test_validate_collateral() {
        assert!(validate_collateral(100 * PRECISION).is_ok());
        assert!(validate_collateral(5 * PRECISION).is_err());
    }

    #[test]
    fn test_validate_price_deviation() {
        let price1 = 1000 * PRECISION;
        let price2 = 1005 * PRECISION; // 0.5% difference
        assert!(validate_price_deviation(price1, price2, 100).is_ok()); // 1% tolerance

        let price3 = 1020 * PRECISION; // 2% difference
        assert!(validate_price_deviation(price1, price3, 100).is_err()); // 1% tolerance
    }

    #[test]
    fn test_check_slippage() {
        let expected = 1000 * PRECISION;
        let actual = 1005 * PRECISION; // 0.5% slippage
        assert!(check_slippage(expected, actual, 100).is_ok()); // 1% tolerance

        let actual_high = 1020 * PRECISION; // 2% slippage
        assert!(check_slippage(expected, actual_high, 100).is_err()); // 1% tolerance
    }
}
