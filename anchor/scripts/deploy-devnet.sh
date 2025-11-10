#!/bin/bash

# Theras Protocol Devnet Deployment Script
# This script deploys both theras_protocol and tgem_plus to Solana devnet

set -e

echo "🚀 Theras Protocol Devnet Deployment"
echo "===================================="
echo ""

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install it first."
    echo "   sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
    exit 1
fi

# Check if Anchor CLI is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Please install it first."
    echo "   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force"
    exit 1
fi

# Check if keypairs exist, if not generate them
if [ ! -f "target/deploy/theras_protocol-keypair.json" ] || [ ! -f "target/deploy/tgem_plus-keypair.json" ]; then
    echo "⚠️  Program keypairs not found. Generating..."
    ./scripts/generate-keypairs.sh
    echo ""
    echo "📝 Updating program IDs in source files..."
    ./scripts/update-program-ids.sh
    echo ""
fi

# Set cluster to devnet
echo "📡 Setting cluster to devnet..."
solana config set --url https://api.devnet.solana.com

# Check wallet balance
BALANCE=$(solana balance | awk '{print $1}')
echo "💰 Current balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 2" | bc -l) )); then
    echo "⚠️  Low balance detected. Requesting airdrop..."
    solana airdrop 2 || echo "⚠️  Airdrop failed. You may need to use a faucet."
fi

# Build programs
echo ""
echo "🔨 Building programs..."
anchor build

# Get program IDs
THERAS_PROGRAM_ID=$(solana address -k target/deploy/theras_protocol-keypair.json)
TGEM_PLUS_PROGRAM_ID=$(solana address -k target/deploy/tgem_plus-keypair.json)

echo ""
echo "📋 Program IDs:"
echo "  Theras Protocol: $THERAS_PROGRAM_ID"
echo "  TGEM+:           $TGEM_PLUS_PROGRAM_ID"

# Check if program IDs match Anchor.toml
EXPECTED_THERAS="Thrs1111111111111111111111111111111111111111"
EXPECTED_TGEM="TGEMp1us111111111111111111111111111111111111"

if [ "$THERAS_PROGRAM_ID" != "$EXPECTED_THERAS" ]; then
    echo ""
    echo "⚠️  WARNING: Theras Protocol ID mismatch!"
    echo "  Expected: $EXPECTED_THERAS"
    echo "  Got:      $THERAS_PROGRAM_ID"
    echo ""
    echo "You need to either:"
    echo "  1. Update declare_id! in programs/theras_protocol/src/lib.rs"
    echo "  2. Update Anchor.toml with the new program ID"
    echo "  3. Use the existing keypair if you have it"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

if [ "$TGEM_PLUS_PROGRAM_ID" != "$EXPECTED_TGEM" ]; then
    echo ""
    echo "⚠️  WARNING: TGEM+ ID mismatch!"
    echo "  Expected: $EXPECTED_TGEM"
    echo "  Got:      $TGEM_PLUS_PROGRAM_ID"
    echo ""
    echo "You need to either:"
    echo "  1. Update declare_id! in programs/tgem_plus/src/lib.rs"
    echo "  2. Update Anchor.toml with the new program ID"
    echo "  3. Use the existing keypair if you have it"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Deploy programs
echo ""
echo "🚀 Deploying to devnet..."
echo ""

anchor deploy --provider.cluster devnet

# Verify deployment
echo ""
echo "✅ Verifying deployment..."
echo ""

echo "Theras Protocol:"
solana program show $THERAS_PROGRAM_ID --url devnet

echo ""
echo "TGEM+:"
solana program show $TGEM_PLUS_PROGRAM_ID --url devnet

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "  1. Initialize TGEM+ program: anchor run initialize-tgem-plus"
echo "  2. Create a test project: anchor run create-project"
echo "  3. Update your frontend configuration with the program IDs"
echo "  4. Start testing on devnet!"
echo ""
echo "📊 Monitor logs:"
echo "  solana logs $THERAS_PROGRAM_ID --url devnet"
echo "  solana logs $TGEM_PLUS_PROGRAM_ID --url devnet"
echo ""
