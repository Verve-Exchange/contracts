/// Decimal precision for prices and amounts (7 decimals)
/// Example: 1.0000000 = 10_000_000
pub const PRECISION: i128 = 10_000_000;

/// Basis points precision (10000 = 100%)
pub const BASIS_POINTS: u32 = 10_000;

/// Maximum contract name length
pub const MAX_NAME_LENGTH: u32 = 32;

/// Minimum collateral amount (10 USDC)
pub const MIN_COLLATERAL: i128 = 10 * PRECISION;

/// Maximum leverage for commodities
pub const MAX_LEVERAGE_COMMODITY: u32 = 10;

/// Maximum leverage for forex
pub const MAX_LEVERAGE_FOREX: u32 = 20;

/// Default maintenance margin (1%)
pub const DEFAULT_MAINTENANCE_MARGIN_BPS: u32 = 100;

/// Default liquidation fee (5%)
pub const DEFAULT_LIQUIDATION_FEE_BPS: u32 = 500;

/// Default trading fee (0.1%)
pub const DEFAULT_TRADING_FEE_BPS: u32 = 10;

/// Maximum price staleness (60 seconds)
pub const MAX_PRICE_STALENESS: u64 = 60;

/// Maximum oracle deviation (1%)
pub const MAX_ORACLE_DEVIATION_BPS: u32 = 100;

/// Funding rate interval (8 hours in seconds)
pub const FUNDING_INTERVAL: u64 = 8 * 3600;

/// Storage TTL extension (30 days in ledgers, ~5s per ledger)
pub const STORAGE_TTL_EXTENSION: u32 = 518_400;

/// Instance storage lifetime (30 days)
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 518_400;

/// Persistent storage lifetime (30 days)
pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 518_400;
