# Deployed Contracts

All contracts deployed to Stellar Testnet.

## Contract Addresses

### Oracle Manager
**Contract ID:** `CDVSYGKM5SFUTJGB5HKBM555YYYSNP26MMQGM5TGJ5MVZ2K5VF7EUH5H`

Manages price feeds from external oracles (Pyth Network).

**Explorer:** https://stellar.expert/explorer/testnet/contract/CDVSYGKM5SFUTJGB5HKBM555YYYSNP26MMQGM5TGJ5MVZ2K5VF7EUH5H

### Vault
**Contract ID:** `(pending - deployment in progress)`

Generic liquidity pool for managing deposits, withdrawals, and LP tokens.

### Faucet
**Contract ID:** `CAL7MUOHBVNFJVFFUNJOOA63T2SB4XLWCRIE2VWXQEJ4HX4GBAJQMBQH`

Testnet USDC faucet for developers and testers.

**Explorer:** https://stellar.expert/explorer/testnet/contract/CAL7MUOHBVNFJVFFUNJOOA63T2SB4XLWCRIE2VWXQEJ4HX4GBAJQMBQH

### Trading Core
**Contract ID:** `CBM5U23LYR4G5OREF4FINKISHQQEDFH7ABODH2WIFREP7DDHZ2AWZPGU`

Core trading engine for opening, closing, and managing leveraged positions.

**Explorer:** https://stellar.expert/explorer/testnet/contract/CBM5U23LYR4G5OREF4FINKISHQQEDFH7ABODH2WIFREP7DDHZ2AWZPGU

### Liquidity Manager
**Contract ID:** `CDMYUM4PNR4ZNGKIRT3ICYBRUBWU5YHGM5L4KBWZC2BEZPCS6YOW7TOD`

Manages liquidity pools with LP token minting/burning and staking.

**Explorer:** https://stellar.expert/explorer/testnet/contract/CDMYUM4PNR4ZNGKIRT3ICYBRUBWU5YHGM5L4KBWZC2BEZPCS6YOW7TOD

### Risk Manager
**Contract ID:** `CDF6CFC33I2ZJFXNRJPR226CU5VT7YIMBOTWWUIAABGWCJ7KIBJSJEJL`

Risk assessment, margin calculations, and liquidation checks.

**Explorer:** https://stellar.expert/explorer/testnet/contract/CDF6CFC33I2ZJFXNRJPR226CU5VT7YIMBOTWWUIAABGWCJ7KIBJSJEJL

---

## Deployment Information

- **Network:** Stellar Testnet
- **Deployed By:** alice
- **Deployment Date:** March 8, 2026

## Next Steps

### 1. Initialize Oracle Manager
```bash
stellar contract invoke \
  --id CDVSYGKM5SFUTJGB5HKBM555YYYSNP26MMQGM5TGJ5MVZ2K5VF7EUH5H \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

### 2. Initialize Faucet
```bash
stellar contract invoke \
  --id CAL7MUOHBVNFJVFFUNJOOA63T2SB4XLWCRIE2VWXQEJ4HX4GBAJQMBQH \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --amount_per_claim 10000000 \
  --cooldown_secs 3600 \
  --max_claims_per_day 10 \
  --daily_limit 100000000
```

### 3. Initialize Vault
```bash
stellar contract invoke \
  --id <VAULT_CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --token <TOKEN_ADDRESS> \
  --deposit_fee_bps 30 \
  --withdraw_fee_bps 30
```

### 4. Initialize Trading Core
```bash
stellar contract invoke \
  --id CBM5U23LYR4G5OREF4FINKISHQQEDFH7ABODH2WIFREP7DDHZ2AWZPGU \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --trading_fee_bps 10 \
  --maintenance_margin_bps 100
```

### 5. Initialize Liquidity Manager
```bash
stellar contract invoke \
  --id CDMYUM4PNR4ZNGKIRT3ICYBRUBWU5YHGM5L4KBWZC2BEZPCS6YOW7TOD \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

### 6. Initialize Risk Manager
```bash
stellar contract invoke \
  --id CDF6CFC33I2ZJFXNRJPR226CU5VT7YIMBOTWWUIAABGWCJ7KIBJSJEJL \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

---

## Testing the Faucet

Once initialized, users can claim USDC from the faucet:

```bash
stellar contract invoke \
  --id CAL7MUOHBVNFJVFFUNJOOA63T2SB4XLWCRIE2VWXQEJ4HX4GBAJQMBQH \
  --source <USER_KEY> \
  --network testnet \
  -- claim_usdc \
  --user <USER_ADDRESS>
```

---

## Contract Interactions

All contracts follow the same pattern for initialization and usage. See individual contract READMEs for detailed function documentation.
