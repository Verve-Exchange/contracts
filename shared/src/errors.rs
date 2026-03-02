use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    // General errors (1-10)
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    Paused = 4,
    InvalidParameter = 5,
    
    // Position errors (11-25)
    PositionNotFound = 11,
    InvalidLeverage = 12,
    InsufficientCollateral = 13,
    PositionTooLarge = 14,
    NotPositionOwner = 15,
    InsufficientMargin = 16,
    PositionAlreadyClosed = 17,
    MaxPositionsReached = 18,
    
    // Oracle errors (26-35)
    PriceStale = 26,
    InvalidPrice = 27,
    OracleUnavailable = 28,
    PriceDeviationTooHigh = 29,
    InvalidPriceFeed = 30,
    
    // Liquidity errors (36-45)
    InsufficientLiquidity = 36,
    InvalidAmount = 37,
    InsufficientBalance = 38,
    PoolNotFound = 39,
    InvalidToken = 40,
    
    // Liquidation errors (46-50)
    NotLiquidatable = 46,
    LiquidationFailed = 47,
    
    // Order errors (51-65)
    OrderNotFound = 51,
    OrderNotPending = 52,
    OrderNotTriggered = 53,
    SlippageExceeded = 54,
    NotOrderOwner = 55,
    InvalidTriggerPrice = 56,
    InvalidSlippageTolerance = 57,
    OrderAlreadyExists = 58,
    InvalidOrderType = 59,
    
    // Risk errors (66-75)
    RiskLimitExceeded = 66,
    VolatilityTooHigh = 67,
    ConcentrationLimitExceeded = 68,
    
    // Math errors (76-80)
    Overflow = 76,
    DivisionByZero = 77,
    NegativeValue = 78,
    
    // Funding errors (81-85)
    FundingIntervalNotElapsed = 81,
    
    // Faucet errors (86-95)
    ClaimLimitExceeded = 86,
    CooldownNotElapsed = 87,
    FaucetDepleted = 88,
}
