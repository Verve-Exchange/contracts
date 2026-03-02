# Stellar Contracts Workspace

This workspace contains the production Soroban contracts for the exchange.

## Active Contracts

- oracle-manager
- trading-core
- liquidity-manager
- risk-manager
- faucet

`shared` is a library crate used by the contracts and is not deployed directly.

## Current Rules

- Maximum leverage: `MAX_LEVERAGE = 10`
- Faucet token policy: USDC-only dispensing (`claim_usdc`)
- Governance contract: not included in current scope

## Testnet Deployment (2026-03-02)

- oracle-manager: `CBMKSU33CD34DSTS5DDIYURT5VSSLP7R2N5FUF4A2G6KSAAN7YMD2ZZT`
- trading-core: `CBD6YMB2VILL47C5B743CNDWZKWNJNJGCRNQYXU4ZKM3XN3LSEKOJPEQ`
- liquidity-manager: `CAGQODXGK6TLSQWHW7TFSMAAH3Z2ZKWDSUX2DVF57VI3GYN7CK7OKTNX`
- risk-manager: `CDTWOBV2WJCO3KN6OICUTOIPSQGEI7W2LBTGR6WBKLW6RI4VZPHHFAC2`
- faucet: `CC5A44EOJ4MDGHKO4RNTL3HJQLEW2SWKO6IOPGHN2PTFWJSHKBUSF3LT`

## Build

From this folder:

`stellar contract build --optimize`

