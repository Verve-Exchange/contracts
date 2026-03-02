use soroban_sdk::{contracttype, Address, Bytes, Symbol};

/// Trading direction
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Direction {
    Long = 0,
    Short = 1,
}

/// Position status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum PositionStatus {
    Open = 0,
    Closed = 1,
    Liquidated = 2,
}

/// Asset class type
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum AssetClass {
    Crypto = 0,
    Commodity = 1,
    Forex = 2,
}

/// Trading position
#[contracttype]
#[derive(Clone, Debug)]
pub struct Position {
    pub id: u64,
    pub trader: Address,
    pub asset: Symbol,
    pub collateral_token: Address,
    pub collateral_amount: i128,
    pub position_size: i128,
    pub entry_price: i128,
    pub leverage: u32,
    pub direction: Direction,
    pub liquidation_price: i128,
    pub unrealized_pnl: i128,
    pub funding_accumulated: i128,
    pub opened_at: u64,
    pub last_updated: u64,
    pub status: PositionStatus,
}

/// Order type
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum OrderType {
    LimitEntry = 0,
    StopLoss = 1,
    TakeProfit = 2,
}

/// Order status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum OrderStatus {
    Pending = 0,
    Executed = 1,
    Cancelled = 2,
    CancelledSlippage = 3,
}

/// Trigger condition
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum TriggerCondition {
    Above = 0,
    Below = 1,
}

/// Trading order
#[contracttype]
#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub trader: Address,
    pub asset: Symbol,
    pub order_type: OrderType,
    pub direction: Direction,
    pub trigger_price: i128,
    pub size: i128,
    pub collateral: i128,
    pub leverage: u32,
    pub slippage_tolerance: u32,
    pub created_at: u64,
    pub status: OrderStatus,
    pub linked_position_id: u64,
}

/// Price data from oracle
#[contracttype]
#[derive(Clone, Debug)]
pub struct PriceData {
    pub asset: Symbol,
    pub price: i128,
    pub confidence: u32,
    pub timestamp: u64,
    pub expo: i32,
}

/// Oracle source
#[contracttype]
#[derive(Clone, Debug)]
pub struct OracleSource {
    pub name: Symbol,
    pub address: Address,
    pub weight: u32,
    pub is_active: bool,
    pub last_update: u64,
}

/// Price feed configuration
#[contracttype]
#[derive(Clone, Debug)]
pub struct PriceFeed {
    pub asset: Symbol,
    pub pyth_feed_id: Bytes,
    pub max_staleness: u64,
    pub max_deviation_bps: u32,
    pub circuit_breaker_enabled: bool,
}

/// Liquidity pool information
#[contracttype]
#[derive(Clone, Debug)]
pub struct PoolInfo {
    pub token: Address,
    pub total_liquidity: i128,
    pub available_liquidity: i128,
    pub reserved_liquidity: i128,
    pub total_lp_tokens: i128,
    pub accumulated_fees: i128,
    pub utilization_rate: u32,
}

/// LP stake information
#[contracttype]
#[derive(Clone, Debug)]
pub struct LPStake {
    pub user: Address,
    pub staked_amount: i128,
    pub rewards_earned: i128,
    pub stake_timestamp: u64,
    pub lock_period: u64,
    pub boost_multiplier: u32,
}

/// Risk parameters for an asset
#[contracttype]
#[derive(Clone, Debug)]
pub struct RiskParameters {
    pub asset: Symbol,
    pub asset_class: AssetClass,
    pub min_margin_ratio: u32,
    pub maintenance_margin_ratio: u32,
    pub liquidation_threshold: u32,
    pub max_leverage: u32,
    pub max_position_size: i128,
    pub max_open_interest: i128,
}

/// Portfolio risk assessment
#[contracttype]
#[derive(Clone, Debug)]
pub struct PortfolioRisk {
    pub user: Address,
    pub total_exposure: i128,
    pub margin_used: i128,
    pub margin_available: i128,
    pub risk_score: u32,
    pub liquidation_distance: i128,
}

/// Market statistics
#[contracttype]
#[derive(Clone, Debug)]
pub struct MarketStats {
    pub asset: Symbol,
    pub total_long_size: i128,
    pub total_short_size: i128,
    pub open_interest: i128,
    pub funding_rate: i128,
    pub last_funding_time: u64,
}

/// Faucet claim record
#[contracttype]
#[derive(Clone, Debug)]
pub struct ClaimRecord {
    pub user: Address,
    pub token: Address,
    pub amount_claimed: i128,
    pub last_claim_time: u64,
    pub total_claims: u32,
}
