#!/bin/bash

# Initialize Faucet Contract
# This script initializes the faucet with configuration

set -e

NETWORK=${1:-testnet}
KEY=${2:-alice}

# Contract IDs
FAUCET_ID="CAL7MUOHBVNFJVFFUNJOOA63T2SB4XLWCRIE2VWXQEJ4HX4GBAJQMBQH"
USDC_ISSUER="GCKIUOTK3NWD33ONH7TQERCSLECXLWQMA377HSJR4E2MV7KPQFAQLOLN"

# Get admin address
ADMIN_ADDRESS=$(stellar keys address $KEY)

echo "Initializing Faucet Contract..."
echo "Network: $NETWORK"
echo "Admin: $ADMIN_ADDRESS"
echo "Faucet Contract: $FAUCET_ID"
echo "USDC Issuer: $USDC_ISSUER"
echo ""

# Initialize faucet
# Parameters:
# - admin: Address
# - usdc_token: Address (USDC issuer)
# - amount_per_claim: i128 (10 USDC = 100000000 stroops)
# - cooldown_secs: u64 (3600 = 1 hour)
# - max_claims_per_day: u32 (10 claims)
# - daily_limit: i128 (100 USDC = 1000000000 stroops)
# - can_mint: bool (true for mint mode)

stellar contract invoke \
  --id $FAUCET_ID \
  --source $KEY \
  --network $NETWORK \
  -- initialize \
  --admin $ADMIN_ADDRESS \
  --usdc_token $USDC_ISSUER \
  --amount_per_claim 100000000 \
  --cooldown_secs 3600 \
  --max_claims_per_day 10 \
  --daily_limit 1000000000 \
  --can_mint true

echo ""
echo "✅ Faucet initialized successfully!"
echo ""
echo "Configuration:"
echo "  Amount per claim: 10 USDC"
echo "  Cooldown: 1 hour"
echo "  Max claims per day: 10"
echo "  Daily limit: 100 USDC"
echo "  Mint mode: enabled"
