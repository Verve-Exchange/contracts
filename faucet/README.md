# Faucet Contract

A testnet token faucet that can either mint tokens directly or dispense from a pre-funded balance.

## Features

- **Mint Mode**: Directly mint tokens to users (for Stellar Asset Contracts with admin rights)
- **Transfer Mode**: Dispense from pre-funded faucet balance
- Configurable claim amounts and cooldowns
- Daily claim limits per user
- Admin refill functionality (transfer mode)
- Pause/unpause mechanism

## Functions

### Initialize
```rust
initialize(
    admin: Address,
    usdc_token: Address,
    amount_per_claim: i128,
    cooldown_secs: u64,
    max_claims_per_day: u32,
    daily_limit: i128,
    can_mint: bool
)
```

**Parameters:**
- `admin` - Admin address for management
- `usdc_token` - Token contract address (Stellar Asset Contract)
- `amount_per_claim` - Amount dispensed per claim (7 decimals, e.g., 10_000_000 = 1 USDC)
- `cooldown_secs` - Seconds between claims (e.g., 3600 = 1 hour)
- `max_claims_per_day` - Maximum claims per user per day
- `daily_limit` - Maximum total amount per user per day
- `can_mint` - If true, faucet mints tokens; if false, transfers from balance

### Claim USDC
```rust
claim_usdc(user: Address) -> i128
```
Request tokens from the faucet. Returns amount claimed.

### Refill (Transfer Mode Only)
```rust
refill_usdc(amount: i128)
```
Admin refills faucet balance (only needed in transfer mode).

### View Functions
- `get_claim_info(user)` - Get user's claim history
- `get_next_claim_time(user)` - When user can claim next
- `get_faucet_balance()` - Current faucet balance (transfer mode)
- `can_mint()` - Check if faucet is in mint mode
- `get_usdc_token()` - Get token contract address
- `get_admin()` - Get admin address

### Admin Functions
- `pause()` - Pause claims
- `unpause()` - Resume claims

## Usage Examples

### Mint Mode (Recommended for Testnet)

Initialize with minting enabled:
```bash
stellar contract invoke \
  --id <FAUCET_ID> \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --amount_per_claim 10000000 \
  --cooldown_secs 3600 \
  --max_claims_per_day 10 \
  --daily_limit 100000000 \
  --can_mint true
```

Users claim tokens (minted directly):
```bash
stellar contract invoke \
  --id <FAUCET_ID> \
  --source user \
  --network testnet \
  -- claim_usdc \
  --user <USER_ADDRESS>
```

### Transfer Mode

Initialize without minting:
```bash
stellar contract invoke \
  --id <FAUCET_ID> \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --amount_per_claim 10000000 \
  --cooldown_secs 3600 \
  --max_claims_per_day 10 \
  --daily_limit 100000000 \
  --can_mint false
```

Admin refills faucet:
```bash
stellar contract invoke \
  --id <FAUCET_ID> \
  --source alice \
  --network testnet \
  -- refill_usdc \
  --amount 1000000000
```

## Configuration Examples

### Generous Testnet Faucet
- Amount per claim: 100 USDC (100_0000000)
- Cooldown: 1 hour (3600 seconds)
- Max claims per day: 10
- Daily limit: 1000 USDC (1000_0000000)
- Mint mode: true

### Conservative Faucet
- Amount per claim: 10 USDC (10_0000000)
- Cooldown: 24 hours (86400 seconds)
- Max claims per day: 1
- Daily limit: 10 USDC (10_0000000)
- Mint mode: false (requires refilling)

## Testing

```bash
cargo test --package faucet
```

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deployment

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/faucet.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

## Notes

- **Mint Mode**: Requires the faucet contract to be set as admin of the Stellar Asset Contract
- **Transfer Mode**: Requires admin to periodically refill the faucet balance
- Daily limits reset at midnight UTC
- Cooldown prevents spam claims
