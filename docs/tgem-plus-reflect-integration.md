# TGEM+ Integration with Reflect.money

## Executive Summary

**TGEM+** is the next evolution of Theras Gem (TGEM), integrating Reflect.money's capital-efficient stablecoin technology to create a **yield-bearing loyalty token**. Users earn passive yield simply by holding TGEM+, combining traditional loyalty rewards with DeFi yield generation.

## What is Reflect.money?

Reflect.money provides **capital-efficient stablecoins** that:
- **Non-custodial**: All collateral held in program accounts, no human access
- **Permissionless**: Create and redeem at will, no restrictions
- **Yield-bearing**: Earn interest just by holding
- **Programmable**: Built on Solana for composability
- **Liquid**: Freely transferable and tradeable

### Key Properties

1. **Price Appreciation Model**: Token value increases over time via rebasing or share-based accounting
2. **Delta-Neutral Strategies**: Yield from cross-margin rate farming and funding rate capture
3. **Verifiable Insurance**: On-chain risk management and collateral verification
4. **Decentralized Strategy Execution**: Automated DeFi strategies without centralized control

## TGEM vs TGEM+ Comparison

| Feature | TGEM (Current) | TGEM+ (Reflect-Powered) |
|---------|----------------|-------------------------|
| **Type** | Off-chain ledger token | On-chain SPL token |
| **Yield** | None | Passive yield from DeFi strategies |
| **Transferability** | Limited (within ecosystem) | Fully transferable |
| **Collateral** | None (points system) | Backed by stablecoins (USDC) |
| **Redemption** | Catalog items only | Catalog items + 1:1 stablecoin redemption |
| **Composability** | Closed ecosystem | DeFi composable |
| **Value Accrual** | Fixed 1:1 | Appreciates over time |

## Architecture Overview

### High-Level Flow

```
User Actions (Login, Quest, Purchase)
    ↓
Theras Rules Engine (Compute Rewards)
    ↓
TGEM+ Minting (via Reflect Protocol)
    ↓
User Wallet (SPL Token Balance)
    ↓
Automatic Yield Accrual (Price Appreciation)
    ↓
Redemption Options:
  - Catalog Items (Theras)
  - Stablecoin (Reflect)
  - DeFi Protocols (Composable)
```

### Components

1. **Theras Protocol Backend**
   - Event tracking and validation
   - Loyalty rules engine
   - Reward computation
   - Integration with Reflect API

2. **Reflect Protocol Integration**
   - TGEM+ token minting/burning
   - Collateral management (USDC)
   - Yield strategy execution
   - Price oracle and rebalancing

3. **On-Chain Program (Solana)**
   - TGEM+ SPL token
   - User account management
   - Redemption logic
   - Catalog integration

4. **Frontend SDK**
   - Wallet connection
   - Balance display (with yield)
   - Transaction signing
   - Redemption interface

## Technical Implementation

### Phase 1: Reflect Integration Setup

#### 1.1 Reflect API Integration

```typescript
// packages/core-sdk/src/reflect-client.ts
import { Connection, PublicKey } from '@solana/web3.js';

export class ReflectClient {
  private connection: Connection;
  private reflectProgramId: PublicKey;
  
  constructor(cluster: 'devnet' | 'mainnet') {
    this.connection = new Connection(
      cluster === 'mainnet' 
        ? 'https://api.mainnet-beta.solana.com'
        : 'https://api.devnet.solana.com'
    );
    this.reflectProgramId = new PublicKey('REFLECT_PROGRAM_ID');
  }
  
  async mintTGEMPlus(
    userWallet: PublicKey,
    amount: number,
    collateralSource: PublicKey
  ): Promise<string> {
    // Mint TGEM+ tokens backed by USDC collateral
    // Returns transaction signature
  }
  
  async redeemTGEMPlus(
    userWallet: PublicKey,
    amount: number,
    destination: PublicKey
  ): Promise<string> {
    // Redeem TGEM+ for underlying USDC
    // Returns transaction signature
  }
  
  async getTGEMPlusBalance(wallet: PublicKey): Promise<{
    balance: number;
    yieldAccrued: number;
    currentPrice: number;
  }> {
    // Get user's TGEM+ balance with yield information
  }
  
  async getCurrentYieldRate(): Promise<number> {
    // Get current APY from Reflect strategies
  }
}
```

#### 1.2 TGEM+ Token Program

```rust
// anchor/programs/tgem_plus/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("TGEMp1us111111111111111111111111111111111111");

#[program]
pub mod tgem_plus {
    use super::*;
    
    /// Initialize TGEM+ mint with Reflect integration
    pub fn initialize_tgem_plus(
        ctx: Context<InitializeTGEMPlus>,
        reflect_vault: Pubkey,
    ) -> Result<()> {
        let tgem_plus = &mut ctx.accounts.tgem_plus_state;
        tgem_plus.authority = ctx.accounts.authority.key();
        tgem_plus.reflect_vault = reflect_vault;
        tgem_plus.total_supply = 0;
        tgem_plus.bump = *ctx.bumps.get("tgem_plus_state").unwrap();
        Ok(())
    }
    
    /// Mint TGEM+ tokens (called by Theras backend)
    pub fn mint_tgem_plus(
        ctx: Context<MintTGEMPlus>,
        amount: u64,
    ) -> Result<()> {
        // Verify collateral is deposited in Reflect vault
        // Mint TGEM+ tokens to user
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.tgem_plus_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.tgem_plus_state.to_account_info(),
        };
        
        let seeds = &[
            b"tgem_plus_state",
            &[ctx.accounts.tgem_plus_state.bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        
        token::mint_to(cpi_ctx, amount)?;
        
        let tgem_plus = &mut ctx.accounts.tgem_plus_state;
        tgem_plus.total_supply = tgem_plus.total_supply.checked_add(amount).unwrap();
        
        Ok(())
    }
    
    /// Redeem TGEM+ for catalog items
    pub fn redeem_for_catalog(
        ctx: Context<RedeemCatalog>,
        item_id: String,
        amount: u64,
    ) -> Result<()> {
        // Burn TGEM+ tokens
        // Record redemption for off-chain fulfillment
        let cpi_accounts = token::Burn {
            mint: ctx.accounts.tgem_plus_mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        
        token::burn(cpi_ctx, amount)?;
        
        // Emit event for backend processing
        emit!(RedemptionEvent {
            user: ctx.accounts.user.key(),
            item_id,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Redeem TGEM+ for stablecoin via Reflect
    pub fn redeem_for_stablecoin(
        ctx: Context<RedeemStablecoin>,
        amount: u64,
    ) -> Result<()> {
        // Burn TGEM+ tokens
        // Withdraw from Reflect vault to user
        // This enables liquidity exit
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTGEMPlus<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"tgem_plus_state"],
        bump
    )]
    pub tgem_plus_state: Account<'info, TGEMPlusState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTGEMPlus<'info> {
    #[account(mut)]
    pub tgem_plus_state: Account<'info, TGEMPlusState>,
    #[account(mut)]
    pub tgem_plus_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RedeemCatalog<'info> {
    #[account(mut)]
    pub tgem_plus_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RedeemStablecoin<'info> {
    #[account(mut)]
    pub tgem_plus_state: Account<'info, TGEMPlusState>,
    #[account(mut)]
    pub tgem_plus_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reflect_vault: AccountInfo<'info>,
    #[account(mut)]
    pub user_usdc_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TGEMPlusState {
    pub authority: Pubkey,
    pub reflect_vault: Pubkey,
    pub total_supply: u64,
    pub bump: u8,
}

#[event]
pub struct RedemptionEvent {
    pub user: Pubkey,
    pub item_id: String,
    pub amount: u64,
    pub timestamp: i64,
}
```

### Phase 2: Backend Integration

#### 2.1 Reward Distribution Service

```go
// services/distribution/tgem_plus.go
package main

import (
    "context"
    "github.com/gagliardetto/solana-go"
    "github.com/gagliardetto/solana-go/rpc"
)

type TGEMPlusService struct {
    rpcClient     *rpc.Client
    reflectClient *ReflectClient
    programID     solana.PublicKey
}

func NewTGEMPlusService(cluster string) *TGEMPlusService {
    var endpoint string
    if cluster == "mainnet" {
        endpoint = rpc.MainNetBeta_RPC
    } else {
        endpoint = rpc.DevNet_RPC
    }
    
    return &TGEMPlusService{
        rpcClient:     rpc.New(endpoint),
        reflectClient: NewReflectClient(cluster),
        programID:     solana.MustPublicKeyFromBase58("TGEMp1us111111111111111111111111111111111111"),
    }
}

func (s *TGEMPlusService) AwardTGEMPlus(
    ctx context.Context,
    userWallet string,
    amount float64,
    eventType string,
) error {
    // 1. Validate event and compute reward
    reward := s.computeReward(eventType, amount)
    
    // 2. Deposit collateral to Reflect vault
    collateralTx, err := s.reflectClient.DepositCollateral(ctx, reward)
    if err != nil {
        return err
    }
    
    // 3. Mint TGEM+ tokens to user
    mintTx, err := s.mintTGEMPlus(ctx, userWallet, reward)
    if err != nil {
        return err
    }
    
    // 4. Record transaction in database
    s.recordAward(userWallet, reward, eventType, mintTx)
    
    return nil
}

func (s *TGEMPlusService) GetUserBalance(
    ctx context.Context,
    userWallet string,
) (*BalanceInfo, error) {
    // Get TGEM+ balance with yield information
    balance, err := s.reflectClient.GetBalance(ctx, userWallet)
    if err != nil {
        return nil, err
    }
    
    return &BalanceInfo{
        Balance:      balance.Amount,
        YieldAccrued: balance.Yield,
        CurrentAPY:   balance.APY,
        ValueUSD:     balance.Amount * balance.Price,
    }, nil
}

type BalanceInfo struct {
    Balance      float64 `json:"balance"`
    YieldAccrued float64 `json:"yield_accrued"`
    CurrentAPY   float64 `json:"current_apy"`
    ValueUSD     float64 `json:"value_usd"`
}
```

#### 2.2 API Endpoints

```go
// services/distribution/handlers.go

// POST /api/v2/award-tgem-plus
func (h *Handler) AwardTGEMPlus(w http.ResponseWriter, r *http.Request) {
    var req struct {
        UserWallet string  `json:"user_wallet"`
        EventType  string  `json:"event_type"`
        Amount     float64 `json:"amount"`
    }
    
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    
    err := h.tgemPlusService.AwardTGEMPlus(
        r.Context(),
        req.UserWallet,
        req.Amount,
        req.EventType,
    )
    
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }
    
    json.NewEncoder(w).Encode(map[string]string{
        "status": "success",
        "message": "TGEM+ awarded successfully",
    })
}

// GET /api/v2/balance/:wallet
func (h *Handler) GetTGEMPlusBalance(w http.ResponseWriter, r *http.Request) {
    wallet := chi.URLParam(r, "wallet")
    
    balance, err := h.tgemPlusService.GetUserBalance(r.Context(), wallet)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }
    
    json.NewEncoder(w).Encode(balance)
}

// POST /api/v2/redeem-catalog
func (h *Handler) RedeemCatalog(w http.ResponseWriter, r *http.Request) {
    var req struct {
        UserWallet string  `json:"user_wallet"`
        ItemID     string  `json:"item_id"`
        Amount     float64 `json:"amount"`
    }
    
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    
    // Process redemption
    // This will burn TGEM+ and fulfill catalog item
}

// POST /api/v2/redeem-stablecoin
func (h *Handler) RedeemStablecoin(w http.ResponseWriter, r *http.Request) {
    var req struct {
        UserWallet string  `json:"user_wallet"`
        Amount     float64 `json:"amount"`
    }
    
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    
    // Redeem TGEM+ for USDC via Reflect
    // Provides liquidity exit
}
```

### Phase 3: Frontend Integration

#### 3.1 TGEM+ Balance Component

```typescript
// packages/ui-sdk/src/components/TGEMPlusWallet.tsx
import React, { useEffect, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { useTheras } from '../context/TherasContext';

export const TGEMPlusWallet: React.FC = () => {
  const { publicKey } = useWallet();
  const { reflectClient } = useTheras();
  const [balance, setBalance] = useState<{
    balance: number;
    yieldAccrued: number;
    currentAPY: number;
    valueUSD: number;
  } | null>(null);
  
  useEffect(() => {
    if (!publicKey) return;
    
    const fetchBalance = async () => {
      const data = await reflectClient.getTGEMPlusBalance(publicKey);
      setBalance(data);
    };
    
    fetchBalance();
    const interval = setInterval(fetchBalance, 10000); // Update every 10s
    
    return () => clearInterval(interval);
  }, [publicKey, reflectClient]);
  
  if (!balance) return <div>Loading...</div>;
  
  return (
    <div className="tgem-plus-wallet">
      <h3>TGEM+ Balance</h3>
      <div className="balance-display">
        <div className="main-balance">
          {balance.balance.toFixed(2)} TGEM+
        </div>
        <div className="value-usd">
          ≈ ${balance.valueUSD.toFixed(2)} USD
        </div>
      </div>
      
      <div className="yield-info">
        <div className="yield-accrued">
          <span>Yield Earned:</span>
          <span className="highlight">+{balance.yieldAccrued.toFixed(4)} TGEM+</span>
        </div>
        <div className="current-apy">
          <span>Current APY:</span>
          <span className="highlight">{balance.currentAPY.toFixed(2)}%</span>
        </div>
      </div>
      
      <div className="actions">
        <button className="btn-primary">Redeem for Items</button>
        <button className="btn-secondary">Cash Out to USDC</button>
      </div>
    </div>
  );
};
```

## Migration Strategy

### From TGEM to TGEM+

#### Option 1: Snapshot & Airdrop
1. Take snapshot of all TGEM balances at cutoff date
2. Airdrop equivalent TGEM+ to all holders
3. Deprecate old TGEM system
4. Provide grace period for catalog redemptions

#### Option 2: Gradual Migration
1. Run TGEM and TGEM+ in parallel
2. Allow users to convert TGEM → TGEM+ at 1:1 ratio
3. New rewards issued in TGEM+ only
4. Phase out TGEM over 6 months

#### Option 3: Hybrid Approach
1. Existing TGEM balances remain for catalog redemption
2. New rewards issued as TGEM+ only
3. Users can choose to convert old TGEM to TGEM+
4. Both systems coexist indefinitely

**Recommended: Option 2 (Gradual Migration)** - Provides flexibility and user choice.

## Economic Model

### Collateralization

- **1:1 USDC backing**: Each TGEM+ is backed by $1 USDC in Reflect vault
- **Over-collateralization buffer**: 10% reserve for yield volatility
- **Treasury management**: Theras deposits USDC to mint TGEM+ for rewards

### Yield Distribution

- **Base yield**: From Reflect's DeFi strategies (5-15% APY)
- **Bonus yield**: Additional rewards for specific user actions
- **Compounding**: Yield automatically compounds in token price

### Redemption Economics

- **Catalog items**: Priced in TGEM+ (includes yield appreciation)
- **Stablecoin exit**: 1:1 redemption to USDC (minus small fee)
- **No lockup**: Users can redeem anytime (liquidity)

## Risk Management

### Smart Contract Risks
- Audit TGEM+ program before mainnet
- Use Reflect's audited contracts
- Implement emergency pause mechanism
- Multi-sig for program authority

### Economic Risks
- Monitor Reflect yield strategies
- Maintain collateralization ratio
- Set redemption limits if needed
- Insurance fund for black swan events

### Operational Risks
- Redundant RPC endpoints
- Transaction retry logic
- Balance reconciliation system
- User support for failed transactions

## Roadmap

### Phase 1: Development (Weeks 1-4)
- [ ] Integrate Reflect SDK
- [ ] Build TGEM+ on-chain program
- [ ] Update backend services
- [ ] Create frontend components

### Phase 2: Testing (Weeks 5-6)
- [ ] Deploy to devnet
- [ ] Internal testing
- [ ] Security audit
- [ ] Beta user testing

### Phase 3: Migration (Weeks 7-8)
- [ ] Mainnet deployment
- [ ] TGEM → TGEM+ conversion tool
- [ ] User communication & education
- [ ] Monitor and support

### Phase 4: Optimization (Weeks 9-12)
- [ ] Yield optimization
- [ ] Additional DeFi integrations
- [ ] Advanced features (staking, governance)
- [ ] Analytics and reporting

## Success Metrics

- **Adoption**: % of users migrated to TGEM+
- **TVL**: Total value locked in Reflect vault
- **Yield**: Average APY delivered to users
- **Retention**: User retention after migration
- **Redemptions**: Catalog vs stablecoin redemption ratio

## Conclusion

TGEM+ represents a significant evolution of the Theras loyalty ecosystem, combining traditional loyalty rewards with DeFi yield generation. By integrating Reflect.money's capital-efficient stablecoin technology, users can earn passive income simply by holding their loyalty tokens, creating a more valuable and engaging experience.

The migration from TGEM to TGEM+ should be gradual and user-friendly, with clear communication and support throughout the process. With proper implementation and risk management, TGEM+ can become a leading example of loyalty tokens in the Web3 era.
