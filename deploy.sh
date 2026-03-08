#!/bin/bash

# Deployment script for all contracts
# Usage: ./deploy.sh <network> <key-name>
# Example: ./deploy.sh testnet alice

NETWORK=${1:-testnet}
KEY=${2:-alice}

echo "Deploying contracts to $NETWORK using key $KEY..."
echo ""

# Deploy Oracle Manager
echo "Deploying Oracle Manager..."
ORACLE_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/oracle_manager.wasm \
  --network $NETWORK \
  --source $KEY)
echo "Oracle Manager: $ORACLE_ID"
echo ""

# Deploy Faucet
echo "Deploying Faucet..."
FAUCET_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/faucet.wasm \
  --network $NETWORK \
  --source $KEY)
echo "Faucet: $FAUCET_ID"
echo ""

# Deploy Vault
echo "Deploying Vault..."
VAULT_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/vault.wasm \
  --network $NETWORK \
  --source $KEY)
echo "Vault: $VAULT_ID"
echo ""

# Deploy Trading Core
echo "Deploying Trading Core..."
TRADING_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/trading_core.wasm \
  --network $NETWORK \
  --source $KEY)
echo "Trading Core: $TRADING_ID"
echo ""

# Deploy Liquidity Manager
echo "Deploying Liquidity Manager..."
LIQUIDITY_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/liquidity_manager.wasm \
  --network $NETWORK \
  --source $KEY)
echo "Liquidity Manager: $LIQUIDITY_ID"
echo ""

# Deploy Risk Manager
echo "Deploying Risk Manager..."
RISK_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/risk_manager.wasm \
  --network $NETWORK \
  --source $KEY)
echo "Risk Manager: $RISK_ID"
echo ""

# Save contract IDs to file
echo "Saving contract IDs to deployed-contracts.txt..."
cat > deployed-contracts.txt <<EOF
# Deployed Contracts on $NETWORK
# Deployed at: $(date)

ORACLE_MANAGER=$ORACLE_ID
FAUCET=$FAUCET_ID
VAULT=$VAULT_ID
TRADING_CORE=$TRADING_ID
LIQUIDITY_MANAGER=$LIQUIDITY_ID
RISK_MANAGER=$RISK_ID
EOF

echo ""
echo "✅ All contracts deployed successfully!"
echo "Contract IDs saved to deployed-contracts.txt"
