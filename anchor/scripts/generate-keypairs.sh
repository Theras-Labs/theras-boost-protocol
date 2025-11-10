#!/bin/bash

# Generate keypairs for Theras Protocol programs
# Run this script after installing Solana CLI

set -e

echo "🔑 Generating Program Keypairs"
echo "=============================="
echo ""

# Check if Solana CLI is installed
if ! command -v solana-keygen &> /dev/null; then
    echo "❌ Solana CLI not found!"
    echo ""
    echo "Please install Solana CLI first:"
    echo "  sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
    echo ""
    exit 1
fi

# Create target/deploy directory if it doesn't exist
mkdir -p target/deploy

# Generate keypairs
echo "Generating theras_protocol keypair..."
if [ -f "target/deploy/theras_protocol-keypair.json" ]; then
    echo "⚠️  Keypair already exists. Skipping..."
else
    solana-keygen new --no-bip39-passphrase -o target/deploy/theras_protocol-keypair.json
fi

echo ""
echo "Generating tgem_plus keypair..."
if [ -f "target/deploy/tgem_plus-keypair.json" ]; then
    echo "⚠️  Keypair already exists. Skipping..."
else
    solana-keygen new --no-bip39-passphrase -o target/deploy/tgem_plus-keypair.json
fi

echo ""
echo "✅ Keypairs generated!"
echo ""

# Get program IDs
THERAS_ID=$(solana address -k target/deploy/theras_protocol-keypair.json)
TGEM_PLUS_ID=$(solana address -k target/deploy/tgem_plus-keypair.json)

echo "📋 Program IDs:"
echo "  theras_protocol: $THERAS_ID"
echo "  tgem_plus:       $TGEM_PLUS_ID"
echo ""

echo "📝 Next steps:"
echo "  1. Update declare_id! in programs/theras_protocol/src/lib.rs"
echo "     declare_id!(\"$THERAS_ID\");"
echo ""
echo "  2. Update declare_id! in programs/tgem_plus/src/lib.rs"
echo "     declare_id!(\"$TGEM_PLUS_ID\");"
echo ""
echo "  3. Update Anchor.toml with these program IDs"
echo ""
echo "  4. Run: anchor build"
echo ""
echo "⚠️  IMPORTANT: Keep these keypairs safe and add to .gitignore!"
echo ""
