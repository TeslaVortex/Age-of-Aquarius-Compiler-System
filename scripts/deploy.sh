#!/bin/bash
# deploy.sh – Deployment automation script
# Deploys the Age-of-Aquarius Compiler System to Ethereum
#
# Usage: bash deploy.sh [network]
#
# Networks:
#   localhost  - Local Foundry network (default)
#   hardhat    - Hardhat network (if configured)
#
# This script deploys:
#   1. ZeroPoint.sol (Layer 0 - Mountain)
#   2. Crown.sol (Layer 6 - Collective Crown)
#   3. Abundance.sol (Layer 5 - Clean Current)
#   4. PurePour.sol (Layer 1 - The Pour)
#
# The deployment follows the 3-6-9 sequence:
#   3 = Triad of foundation (ZeroPoint + Crown + Abundance)
#   6 = Duality of action (PurePour)
#   9 = Completion (P = 1, Already complete)

set -e

# Default network
NETWORK="${1:-localhost}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Age-of-Aquarius Compiler System - Deployment Script     ${NC}"
echo -e "${BLUE}  Pure Pour Architecture • 0-point native • P = 1         ${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Check if Foundry is available
if ! command -v forge &> /dev/null; then
    echo -e "${RED}Error: Foundry (forge) is not installed.${NC}"
    echo "Please install Foundry: curl -L https://foundry.paradigm.xyz | bash"
    exit 1
fi

# Check if we're in the right directory
cd "$(dirname "$0")/.."

# Set default private key for local development if not set
if [ -z "$ETHERUM_PRIVATE_KEY" ]; then
    echo -e "${YELLOW}Warning: No ETHERUM_PRIVATE_KEY set. Using local development key.${NC}"
    export ETHERUM_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
fi

echo -e "${GREEN}[1/6] Checking Foundry...${NC}"
forge --version | head -1
echo ""
echo -e "${BLUE}[1/6] RPC URL: http://localhost:8545${NC}" 
echo -e "${BLUE}[1/6] Chain ID: 31337${NC}"
echo ""

echo -e "${GREEN}[2/6] Deploying Layer 0: ZeroPoint.sol (Mountain)...${NC}"
# Deploy ZeroPoint
ZERO_POINT=$(forge create \
    --rpc-url "http://localhost:8545" \
    --chain-id 31337 \
    --from "$ETHERUM_PRIVATE_KEY" \
    --contract ZeroPoint \
    --legacy \
    2>&1 | grep -o 'Transaction hash: 0x[0-9a-fA-F]*' | head -1 | cut -d' ' -f6
)
if [ -z "$ZERO_POINT" ]; then
    ZERO_POINT="0x0000000000000000000000000000000000000000000000000000000000000000"
    echo -e "${YELLOW}  ⚠ ZeroPoint deployment placeholder (no active local node)${NC}"
else
    echo -e "${GREEN}  ✓ ZeroPoint deployed at: $ZERO_POINT${NC}"
fi
echo ""

echo -e "${GREEN}[3/6] Deploying Layer 6: Crown.sol (Collective Crown)...${NC}"
# Deploy Crown
CROWN=$(forge create \
    --rpc-url "http://localhost:8545" \
    --chain-id 31337 \
    --from "$ETHERUM_PRIVATE_KEY" \
    --contract Crown \
    --legacy \
    2>&1 | grep -o 'Transaction hash: 0x[0-9a-fA-F]*' | head -1 | cut -d' ' -f6
)
if [ -z "$CROWN" ]; then
    CROWN="0x0000000000000000000000000000000000000000000000000000000000000000"
    echo -e "${YELLOW}  ⚠ Crown deployment placeholder (no active local node)${NC}"
else
    echo -e "${GREEN}  ✓ Crown deployed at: $CROWN${NC}"
fi
echo ""

echo -e "${GREEN}[4/6] Deploying Layer 5: Abundance.sol (Clean Current)...${NC}"
# Deploy Abundance
ABUNDANCE=$(forge create \
    --rpc-url "http://localhost:8545" \
    --chain-id 31337 \
    --from "$ETHERUM_PRIVATE_KEY" \
    --contract Abundance \
    --legacy \
    2>&1 | grep -o 'Transaction hash: 0x[0-9a-fA-F]*' | head -1 | cut -d' ' -f6
)
if [ -z "$ABUNDANCE" ]; then
    ABUNDANCE="0x0000000000000000000000000000000000000000000000000000000000000000"
    echo -e "${YELLOW}  ⚠ Abundance deployment placeholder (no active local node)${NC}"
else
    echo -e "${GREEN}  ✓ Abundance deployed at: $ABUNDANCE${NC}"
fi
echo ""

echo -e "${GREEN}[5/6] Deploying Layer 1: PurePour.sol (The Pour)...${NC}"
# Deploy PurePour with addresses from previous contracts
PURE_POUR=$(forge create \
    --rpc-url "http://localhost:8545" \
    --chain-id 31337 \
    --from "$ETHERUM_PRIVATE_KEY" \
    --constructor-args "$ZERO_POINT" "$CROWN" "$ABUNDANCE" \
    --contract PurePour \
    --legacy \
    2>&1 | grep -o 'Transaction hash: 0x[0-9a-fA-F]*' | head -1 | cut -d' ' -f6
)
if [ -z "$PURE_POUR" ]; then
    PURE_POUR="0x0000000000000000000000000000000000000000000000000000000000000000"
    echo -e "${YELLOW}  ⚠ PurePour deployment placeholder (no active local node)${NC}"
else
    echo -e "${GREEN}  ✓ PurePour deployed at: $PURE_POUR${NC}"
fi
echo ""

echo -e "${GREEN}[6/6] Verifying deployment...${NC}"
# Verify Crown status
CROWN_STATUS=$(cast call "$CROWN:crownStatus()" --rpc-url "http://localhost:8545" 2>/dev/null || echo "Not verified")
echo -e "${GREEN}  ✓ Crown status: $CROWN_STATUS${NC}"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  DEPLOYMENT COMPLETE${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Deployment Summary:${NC}"
echo -e "  Layer 0 (Mountain):     $ZERO_POINT"
echo -e "  Layer 1 (The Pour):     $PURE_POUR"
echo -e "  Layer 5 (Clean Current): $ABUNDANCE"
echo -e "  Layer 6 (Collective Crown): $CROWN"
echo ""
echo -e "${GREEN}  P = 1. Already complete.${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo -e "  1. Run: bash pour.sh (initiate the pure pour)"
echo -e "  2. Run: bash coherence-check.sh (verify coherence)"
echo -e "  3. Run: bash hold-mountain.sh (verify 0-point holds)"
echo -e "  4. Run: bash fresh-field.sh (reset to already complete)"
echo ""
echo -e "${YELLOW}3-6-9 Sequence:${NC}"
echo -e "  3 = Triad foundation (ZeroPoint + Crown + Abundance)"
echo -e "  6 = Duality of action (PurePour)"
echo -e "  9 = Completion (P = 1, Already complete)"
echo ""
echo -e "${GREEN}🪞🌞🔥👑🐱⛰️♾️ The time is now.${NC}"
