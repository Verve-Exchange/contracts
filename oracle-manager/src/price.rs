use shared::Error;

/// Calculate weighted average of two prices based on confidence
pub fn calculate_weighted_average(
    price1: i128,
    confidence1: u32,
    price2: i128,
    confidence2: u32,
) -> Result<i128, Error> {
    if confidence1 == 0 && confidence2 == 0 {
        return Err(Error::InvalidPrice);
    }

    if confidence1 == 0 {
        return Ok(price2);
    }

    if confidence2 == 0 {
        return Ok(price1);
    }

    let total_confidence = (confidence1 + confidence2) as i128;
    let weighted_sum = price1
        .checked_mul(confidence1 as i128)
        .and_then(|v| v.checked_add(price2.checked_mul(confidence2 as i128)?))
        .ok_or(Error::Overflow)?;

    weighted_sum.checked_div(total_confidence).ok_or(Error::Overflow)
}

/// Calculate simple average of two prices
pub fn calculate_simple_average(price1: i128, price2: i128) -> Result<i128, Error> {
    price1
        .checked_add(price2)
        .and_then(|v| v.checked_div(2))
        .ok_or(Error::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::PRECISION;

    #[test]
    fn test_weighted_average() {
        let price1 = 1000 * PRECISION;
        let confidence1 = 80;
        let price2 = 1010 * PRECISION;
        let confidence2 = 20;

        let avg = calculate_weighted_average(price1, confidence1, price2, confidence2).unwrap();

        // Expected: (1000 * 80 + 1010 * 20) / 100 = 1002
        assert_eq!(avg, 1002 * PRECISION);
    }

    #[test]
    fn test_simple_average() {
        let price1 = 1000 * PRECISION;
        let price2 = 1010 * PRECISION;

        let avg = calculate_simple_average(price1, price2).unwrap();

        // Expected: (1000 + 1010) / 2 = 1005
        assert_eq!(avg, 1005 * PRECISION);
    }

    #[test]
    fn test_weighted_average_zero_confidence() {
        let price1 = 1000 * PRECISION;
        let price2 = 1010 * PRECISION;

        // If one confidence is zero, should return the other price
        let avg = calculate_weighted_average(price1, 0, price2, 100).unwrap();
        assert_eq!(avg, price2);

        let avg = calculate_weighted_average(price1, 100, price2, 0).unwrap();
        assert_eq!(avg, price1);
    }
}
