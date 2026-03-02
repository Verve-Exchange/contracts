# Contracts

Exchange core contracts.

## 1) Workspace Overview

| Item               | Value                           |
| ------------------ | ------------------------------- |
| Network deployed   | Testnet                         |
| Leverage policy    | `MAX_LEVERAGE = 10`             |
| Faucet policy      | USDC-only claims (`claim_usdc`) |
| Governance         | Not in active scope             |
| Deploy signer used | `alice`                         |

## 2) Crates and Purpose

| Crate               | Type     | Deployable | Purpose                                                           |
| ------------------- | -------- | ---------- | ----------------------------------------------------------------- |
| `shared`            | Library  | No         | Shared types, errors, constants, math, validation                 |
| `oracle-manager`    | Contract | Yes        | Feed registration, price updates, staleness/deviation checks      |
| `trading-core`      | Contract | Yes        | Positions, orders, execution, liquidation, market stats           |
| `liquidity-manager` | Contract | Yes        | Pool accounting, LP mint/burn accounting, staking state           |
| `risk-manager`      | Contract | Yes        | Risk parameters, margin requirement, liquidation threshold checks |
| `faucet`            | Contract | Yes        | USDC-only faucet with cooldown and daily limits                   |

## 3) Contract Interface Summary

| Contract            | Initialize                                                                                        | Admin Ops                                                                                        | User/Read Ops                                                                                                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `oracle-manager`    | `initialize(admin)`                                                                               | `register_feed`, `set_circuit_breaker`, `update_feed_config`, `set_admin`, `pause`, `unpause`    | `get_price`, `get_prices`, `get_admin`, `is_paused`                                                                                                                                               |
| `trading-core`      | `initialize(admin, trading_fee_bps, maintenance_margin_bps)`                                      | `pause`, `unpause`                                                                               | `open_position`, `close_position`, `place_order`, `cancel_order`, `execute_order`, `liquidate_position`, `get_position`, `get_user_positions`, `get_order`, `get_user_orders`, `get_market_stats` |
| `liquidity-manager` | `initialize(admin)`                                                                               | `create_pool`, `reserve_liquidity`, `release_liquidity`, `settle_trader_pnl`, `pause`, `unpause` | `deposit`, `withdraw`, `stake_lp_tokens`, `unstake_lp_tokens`, `get_pool_info`, `get_user_stake_info`                                                                                             |
| `risk-manager`      | `initialize(admin)`                                                                               | `update_risk_parameters`, `pause`, `unpause`                                                     | `assess_position_risk`, `check_liquidation_threshold`, `calculate_margin_requirement`, `get_max_leverage`, `get_risk_parameters`                                                                  |
| `faucet`            | `initialize(admin, usdc_token, amount_per_claim, cooldown_secs, max_claims_per_day, daily_limit)` | `refill_usdc`, `pause`, `unpause`                                                                | `claim_usdc`, `get_claim_info`, `get_next_claim_time`, `get_usdc_token`                                                                                                                           |

## 4) Global Constraints

| Constraint       | Value                                  | Enforced In                                               |
| ---------------- | -------------------------------------- | --------------------------------------------------------- |
| Maximum leverage | `10`                                   | `shared::MAX_LEVERAGE`, trading and risk validation paths |
| Price validity   | positive, non-stale, bounded deviation | oracle manager + shared validation                        |
| Faucet token     | USDC only                              | faucet storage/config and `claim_usdc` flow               |
| Pause control    | per-contract admin pause               | all deployable contracts                                  |

## 5) Testnet Deployments (2026-03-02)

| Alias               | Contract ID                                                |
| ------------------- | ---------------------------------------------------------- |
| `oracle-manager`    | `CBMKSU33CD34DSTS5DDIYURT5VSSLP7R2N5FUF4A2G6KSAAN7YMD2ZZT` |
| `trading-core`      | `CBD6YMB2VILL47C5B743CNDWZKWNJNJGCRNQYXU4ZKM3XN3LSEKOJPEQ` |
| `liquidity-manager` | `CAGQODXGK6TLSQWHW7TFSMAAH3Z2ZKWDSUX2DVF57VI3GYN7CK7OKTNX` |
| `risk-manager`      | `CDTWOBV2WJCO3KN6OICUTOIPSQGEI7W2LBTGR6WBKLW6RI4VZPHHFAC2` |
| `faucet`            | `CC5A44EOJ4MDGHKO4RNTL3HJQLEW2SWKO6IOPGHN2PTFWJSHKBUSF3LT` |

## 6) Build, Test, Deploy Commands

| Action               | Command                                                                                       |
| -------------------- | --------------------------------------------------------------------------------------------- |
| Build optimized wasm | `stellar contract build --optimize`                                                           |
| Run tests            | `cargo test`                                                                                  |
| Build release        | `cargo build --release`                                                                       |
| Deploy one contract  | `stellar contract deploy --wasm <wasm_path> --source alice --network testnet --alias <alias>` |

## 8) Notes

| Topic                   | Status                                                        |
| ----------------------- | ------------------------------------------------------------- |
| Governance contract     | Removed from active workspace                                 |
| Shared crate deployment | Not deployed (library only)                                   |
| Production posture      | Optimized wasm builds and deterministic contract aliases used |
