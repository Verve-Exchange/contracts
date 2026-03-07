# Vault Contract

A generic liquidity pool contract for managing deposits, withdrawals, and LP tokens.

## Features

- Deposit tokens to receive LP tokens
- Withdraw tokens by burning LP tokens
- Reserve/release liquidity for trading
- Configurable deposit and withdrawal fees
- Admin controls and pause mechanism
- Dynamic LP token pricing

## Functions

### Initialize
```rust
initialize(
    admin: Address,
    token: Address,
    deposit_fee_bps: u32,
    withdraw_fee_bps: u32
)
```

### Deposit
```rust
deposit(user: Address, amount: i128) -> i128
```
Deposit tokens and receive LP tokens.

### Withdraw
```rust
withdraw(user: Address, lp_tokens: i128) -> i128
```
Burn LP tokens and receive tokens back.

### Reserve Liquidity
```rust
reserve_liquidity(amount: i128)
```
Reserve liquidity for trading (admin only).

### Release Liquidity
```rust
release_liquidity(amount: i128)
```
Release reserved liquidity (admin only).

### View Functions
- `get_pool_info()` - Get pool statistics
- `get_lp_balance(user)` - Get LP token balance
- `get_lp_price()` - Get LP token price
- `is_paused()` - Check if paused
- `get_admin()` - Get admin address

### Admin Functions
- `pause()` - Pause deposits/withdrawals
- `unpause()` - Unpause
- `set_admin(new_admin)` - Transfer admin role

## Configuration

- `deposit_fee_bps` - Deposit fee in basis points (max 1000 = 10%)
- `withdraw_fee_bps` - Withdrawal fee in basis points (max 1000 = 10%)

## LP Token Mechanics

- First deposit: 1:1 ratio (1 token = 1 LP token)
- Subsequent deposits: `LP tokens = (deposit × total_lp_tokens) / total_liquidity`
- Withdrawals: `tokens = (lp_tokens × total_liquidity) / total_lp_tokens`

## Testing

```bash
cargo test
```

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deployment

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/vault.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```
