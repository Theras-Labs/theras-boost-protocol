# Devnet Deployment Guide

## Prerequisites

1. **Solana CLI** installed and configured
2. **Anchor CLI** (v0.30.1+) installed
3. **Devnet wallet** with SOL for deployment fees

## Setup Devnet Wallet

```bash
# Create a new keypair for devnet (if needed)
solana-keygen new --outfile ~/.config/solana/devnet.json

# Set Solana CLI to use devnet
solana config set --url https://api.devnet.solana.com

# Set your wallet
solana config set --keypair ~/.config/solana/devnet.json

# Airdrop SOL for deployment (may need to run multiple times)
solana airdrop 2

# Check balance
solana balance
```

## Build the Program

```bash
cd anchor

# Build the program
anchor build

# Get the program ID
solana address -k target/deploy/theras_protocol-keypair.json
```

## Update Program ID

If the generated program ID differs from `Thrs1111111111111111111111111111111111111111`:

1. Update `declare_id!` in `programs/theras_protocol/src/lib.rs`
2. Update program ID in `Anchor.toml` under `[programs.devnet]`
3. Rebuild: `anchor build`

## Deploy to Devnet

```bash
# Make sure you're in the anchor directory
cd anchor

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Verify deployment
solana program show <PROGRAM_ID> --url devnet
```

## Test on Devnet

```bash
# Update Anchor.toml provider cluster to devnet
# Then run tests
anchor test --provider.cluster devnet --skip-local-validator
```

## Verify Program

```bash
# Check program account
solana program show Thrs1111111111111111111111111111111111111111 --url devnet

# Check program logs
solana logs Thrs1111111111111111111111111111111111111111 --url devnet
```

## Initialize a Project on Devnet

Using Anchor CLI:

```bash
anchor run initialize-project --provider.cluster devnet
```

Or using Solana CLI with a custom script:

```bash
# Create a transaction to initialize a project
solana program call <PROGRAM_ID> initialize_project \
  --args '{"project_key": "YOUR_PROJECT_KEY"}' \
  --url devnet
```

## Upgrade Program

```bash
# Build new version
anchor build

# Upgrade on devnet
anchor upgrade target/deploy/theras_protocol.so \
  --program-id Thrs1111111111111111111111111111111111111111 \
  --provider.cluster devnet
```

## Monitor Program

```bash
# Watch program logs in real-time
solana logs Thrs1111111111111111111111111111111111111111 --url devnet

# Check program data size
solana program show Thrs1111111111111111111111111111111111111111 --url devnet
```

## Troubleshooting

### Insufficient Funds

```bash
# Request more SOL
solana airdrop 2 --url devnet

# Or use a devnet faucet: https://faucet.solana.com/
```

### Program Already Deployed

If the program ID is already taken, you'll need to:
1. Generate a new keypair: `solana-keygen new -o target/deploy/theras_protocol-keypair.json`
2. Update the program ID in code and config
3. Rebuild and deploy

### Deployment Failed

```bash
# Check your balance
solana balance --url devnet

# Verify your wallet is set correctly
solana config get

# Check program size (max 10MB for devnet)
ls -lh target/deploy/theras_protocol.so
```

## Next Steps

After successful deployment:

1. Update frontend SDK configuration to point to devnet
2. Test all program instructions (initialize_project, register_user, record_daily_login, record_quest)
3. Monitor program usage and performance
4. Prepare for mainnet deployment when ready

## Security Notes

- Never commit your devnet keypair to version control
- Use environment variables for sensitive configuration
- Test thoroughly on devnet before mainnet deployment
- Consider using a multisig for program authority on mainnet
