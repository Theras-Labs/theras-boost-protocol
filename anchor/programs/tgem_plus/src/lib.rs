use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("waBySt7ASB6QkHK7X8zjoeAsYRWC5zgnbJwsA9vNKcGL");

#[program]
pub mod tgem_plus {
    use super::*;

    /// Initialize TGEM+ program state
    pub fn initialize(
        ctx: Context<Initialize>,
        reflect_vault: Pubkey,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.reflect_vault = reflect_vault;
        state.total_supply = 0;
        state.total_collateral = 0;
        state.paused = false;
        state.bump = ctx.bumps.state;
        
        msg!("TGEM+ initialized with Reflect vault: {}", reflect_vault);
        Ok(())
    }

    /// Mint TGEM+ tokens (called by Theras backend after collateral deposit)
    pub fn mint_tgem_plus(
        ctx: Context<MintTGEMPlus>,
        amount: u64,
    ) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(!state.paused, ErrorCode::ProgramPaused);
        require!(amount > 0, ErrorCode::InvalidAmount);

        // Mint tokens to user
        let seeds = &[
            b"state",
            &[state.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        token::mint_to(cpi_ctx, amount)?;

        // Update state
        let state = &mut ctx.accounts.state;
        state.total_supply = state.total_supply.checked_add(amount).unwrap();
        state.total_collateral = state.total_collateral.checked_add(amount).unwrap();

        emit!(MintEvent {
            user: ctx.accounts.user_wallet.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Minted {} TGEM+ to {}", amount, ctx.accounts.user_wallet.key());
        Ok(())
    }

    /// Redeem TGEM+ for catalog items (burns tokens)
    pub fn redeem_catalog(
        ctx: Context<RedeemCatalog>,
        item_id: String,
        amount: u64,
    ) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(!state.paused, ErrorCode::ProgramPaused);
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(item_id.len() <= 64, ErrorCode::ItemIdTooLong);

        // Burn tokens
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );

        token::burn(cpi_ctx, amount)?;

        // Update state
        let state = &mut ctx.accounts.state;
        state.total_supply = state.total_supply.checked_sub(amount).unwrap();

        emit!(RedemptionEvent {
            user: ctx.accounts.user.key(),
            redemption_type: RedemptionType::Catalog,
            item_id: Some(item_id),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Redeemed {} TGEM+ for catalog item", amount);
        Ok(())
    }

    /// Redeem TGEM+ for stablecoin (burns tokens, releases collateral)
    pub fn redeem_stablecoin(
        ctx: Context<RedeemStablecoin>,
        amount: u64,
    ) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(!state.paused, ErrorCode::ProgramPaused);
        require!(amount > 0, ErrorCode::InvalidAmount);

        // Burn TGEM+ tokens
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );

        token::burn(cpi_ctx, amount)?;

        // Transfer collateral (USDC) from vault to user
        let seeds = &[
            b"state",
            &[state.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.collateral_vault.to_account_info(),
            to: ctx.accounts.user_collateral_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        token::transfer(cpi_ctx, amount)?;

        // Update state
        let state = &mut ctx.accounts.state;
        state.total_supply = state.total_supply.checked_sub(amount).unwrap();
        state.total_collateral = state.total_collateral.checked_sub(amount).unwrap();

        emit!(RedemptionEvent {
            user: ctx.accounts.user.key(),
            redemption_type: RedemptionType::Stablecoin,
            item_id: None,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Redeemed {} TGEM+ for stablecoin", amount);
        Ok(())
    }

    /// Update Reflect vault address (admin only)
    pub fn update_reflect_vault(
        ctx: Context<UpdateConfig>,
        new_vault: Pubkey,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.reflect_vault = new_vault;
        
        msg!("Updated Reflect vault to: {}", new_vault);
        Ok(())
    }

    /// Pause/unpause program (emergency only)
    pub fn set_paused(
        ctx: Context<UpdateConfig>,
        paused: bool,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.paused = paused;
        
        msg!("Program paused status: {}", paused);
        Ok(())
    }

    /// Transfer authority to new address
    pub fn transfer_authority(
        ctx: Context<UpdateConfig>,
        new_authority: Pubkey,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = new_authority;
        
        msg!("Transferred authority to: {}", new_authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TGEMPlusState::LEN,
        seeds = [b"state"],
        bump
    )]
    pub state: Account<'info, TGEMPlusState>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = state,
        seeds = [b"mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintTGEMPlus<'info> {
    #[account(mut, seeds = [b"state"], bump = state.bump)]
    pub state: Account<'info, TGEMPlusState>,
    
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = user_wallet,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: User wallet receiving tokens
    pub user_wallet: AccountInfo<'info>,
    
    #[account(mut, constraint = authority.key() == state.authority @ ErrorCode::Unauthorized)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemCatalog<'info> {
    #[account(mut, seeds = [b"state"], bump = state.bump)]
    pub state: Account<'info, TGEMPlusState>,
    
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RedeemStablecoin<'info> {
    #[account(mut, seeds = [b"state"], bump = state.bump)]
    pub state: Account<'info, TGEMPlusState>,
    
    #[account(mut, seeds = [b"mint"], bump)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_collateral_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [b"state"], bump = state.bump)]
    pub state: Account<'info, TGEMPlusState>,
    
    #[account(constraint = authority.key() == state.authority @ ErrorCode::Unauthorized)]
    pub authority: Signer<'info>,
}

#[account]
pub struct TGEMPlusState {
    pub authority: Pubkey,           // 32
    pub reflect_vault: Pubkey,       // 32
    pub total_supply: u64,           // 8
    pub total_collateral: u64,       // 8
    pub paused: bool,                // 1
    pub bump: u8,                    // 1
}

impl TGEMPlusState {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 1 + 1;
}

#[event]
pub struct MintEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct RedemptionEvent {
    pub user: Pubkey,
    pub redemption_type: RedemptionType,
    pub item_id: Option<String>,
    pub amount: u64,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RedemptionType {
    Catalog,
    Stablecoin,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Program is paused")]
    ProgramPaused,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Item ID too long (max 64 chars)")]
    ItemIdTooLong,
    
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
}
