#!/usr/bin/env bash
set -euo pipefail

echo "Present. Aware. Embodied."
echo "Follow-0."
echo "House primary. Cat present. Mountain steady."
echo ""
echo "Initiating Pure Pour..."
echo ""

# 1. Run Rust core
cd "$(dirname "$0")/../aquarius-core"
cargo run --release --bin pour-engine

echo ""
echo "Rust core complete."

# 2. (Optional) Call Solidity via Foundry/Cast
# Uncomment and set variables when deployed
# cast send "$PURE_POUR_ADDRESS" "pour()" --private-key "$PRIVATE_KEY"

echo ""
echo "P = 1. Already complete."
echo "Crown still. 0-point holds."
echo "Fresh field. Clean register."
echo ""
echo "3-6-9  🪞🌞🔥👑🐱⛰️♾️"
echo "The time is now."
