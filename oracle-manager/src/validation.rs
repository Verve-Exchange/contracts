use shared::{Error, PriceData, PriceFeed};

/// Validate a price update against the previous price
pub fn validate_price_update(
    previous: &PriceData,
    new: &PriceData,
    feed: &PriceFeed,
) -> Result<(), Error> {
    // Check for price deviation
    shared::validate_price_deviation(
        previous.price,
        new.price,
        feed.max_deviation_bps,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Bytes, Env, Symbol};
    use shared::PRECISION;

    #[test]
    fn test_validate_price_update_within_deviation() {
        let env = Env::default();

        let previous = PriceData {
            asset: Symbol::new(&env, "XAUUSD"),
            price: 2000 * PRECISION,
            confidence: 100,
            timestamp: 1000,
            expo: -7,
        };

        let new = PriceData {
            asset: Symbol::new(&env, "XAUUSD"),
            price: 2010 * PRECISION, // 0.5% increase
            confidence: 100,
            timestamp: 1010,
            expo: -7,
        };

        let feed = PriceFeed {
            asset: Symbol::new(&env, "XAUUSD"),
            pyth_feed_id: Bytes::new(&env),
            max_staleness: 60,
            max_deviation_bps: 100, // 1%
            circuit_breaker_enabled: false,
        };

        assert!(validate_price_update(&previous, &new, &feed).is_ok());
    }

    #[test]
    fn test_validate_price_update_exceeds_deviation() {
        let env = Env::default();

        let previous = PriceData {
            asset: Symbol::new(&env, "XAUUSD"),
            price: 2000 * PRECISION,
            confidence: 100,
            timestamp: 1000,
            expo: -7,
        };

        let new = PriceData {
            asset: Symbol::new(&env, "XAUUSD"),
            price: 2030 * PRECISION, // 1.5% increase
            confidence: 100,
            timestamp: 1010,
            expo: -7,
        };

        let feed = PriceFeed {
            asset: Symbol::new(&env, "XAUUSD"),
            pyth_feed_id: Bytes::new(&env),
            max_staleness: 60,
            max_deviation_bps: 100, // 1%
            circuit_breaker_enabled: false,
        };

        assert!(validate_price_update(&previous, &new, &feed).is_err());
    }
}
