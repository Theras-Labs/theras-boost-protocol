#!/bin/bash

# Automatically update program IDs in source files and Anchor.toml
# Run this after generating keypairs

set -e

echo "🔄 Updating Program IDs"
echo "======================="
echo ""

# Check if keypairs exist
if [ ! -f "target/deploy/theras_protocol-keypair.json" ]; then
    echo "❌ theras_protocol keypair not found!"
    echo "Run ./scripts/generate-keypairs.sh first"
    exit 1
fi

if [ ! -f "target/deploy/tgem_plus-keypair.json" ]; then
    echo "❌ tgem_plus keypair not found!"
    echo "Run ./scripts/generate-keypairs.sh first"
    exit 1
fi

# Get program IDs
THERAS_ID=$(solana address -k target/deploy/theras_protocol-keypair.json)
TGEM_PLUS_ID=$(solana address -k target/deploy/tgem_plus-keypair.json)

echo "Program IDs:"
echo "  theras_protocol: $THERAS_ID"
echo "  tgem_plus:       $TGEM_PLUS_ID"
echo ""

# Update theras_protocol/src/lib.rs
echo "Updating programs/theras_protocol/src/lib.rs..."
sed -i.bak "s/declare_id!(\".*\");/declare_id!(\"$THERAS_ID\");/" programs/theras_protocol/src/lib.rs
rm programs/theras_protocol/src/lib.rs.bak

# Update tgem_plus/src/lib.rs
echo "Updating programs/tgem_plus/src/lib.rs..."
sed -i.bak "s/declare_id!(\".*\");/declare_id!(\"$TGEM_PLUS_ID\");/" programs/tgem_plus/src/lib.rs
rm programs/tgem_plus/src/lib.rs.bak

# Update Anchor.toml
echo "Updating Anchor.toml..."
sed -i.bak "s/theras_protocol = \".*\"/theras_protocol = \"$THERAS_ID\"/" Anchor.toml
sed -i.bak "s/tgem_plus = \".*\"/tgem_plus = \"$TGEM_PLUS_ID\"/" Anchor.toml
rm Anchor.toml.bak

echo ""
echo "✅ Program IDs updated successfully!"
echo ""
echo "Next steps:"
echo "  1. Review the changes"
echo "  2. Run: anchor build"
echo "  3. Deploy: ./scripts/deploy-devnet.sh"
echo ""
