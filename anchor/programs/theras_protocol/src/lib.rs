use anchor_lang::prelude::*;

declare_id!("Thrs1111111111111111111111111111111111111111");

#[program]
pub mod theras_protocol {
    use super::*;

    pub fn initialize_project(ctx: Context<InitializeProject>, project_key: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.authority = *ctx.accounts.authority.key;
        project.project_key = project_key;
        project.bump = *ctx.bumps.get("project").unwrap();
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.project = ctx.accounts.project.key();
        user.wallet = *ctx.accounts.user_wallet.key;
        user.daily_logins = 0;
        user.quests = 0;
        user.bump = *ctx.bumps.get("user").unwrap();
        Ok(())
    }

    pub fn record_daily_login(ctx: Context<Record>) -> Result<()> {
        require!(ctx.accounts.user.project == ctx.accounts.project.key(), ErrorCode::InvalidProject);
        require!(ctx.accounts.user.wallet == *ctx.accounts.user_wallet.key, ErrorCode::InvalidUser);
        let user = &mut ctx.accounts.user;
        user.daily_logins = user.daily_logins.saturating_add(1);
        Ok(())
    }

    pub fn record_quest(ctx: Context<Record>) -> Result<()> {
        require!(ctx.accounts.user.project == ctx.accounts.project.key(), ErrorCode::InvalidProject);
        require!(ctx.accounts.user.wallet == *ctx.accounts.user_wallet.key, ErrorCode::InvalidUser);
        let user = &mut ctx.accounts.user;
        user.quests = user.quests.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(project_key: String)]
pub struct InitializeProject<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 64 + 1,
        seeds = [b"project", project_key.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(
        init,
        payer = user_wallet,
        space = 8 + 32 + 32 + 8 + 8 + 1,
        seeds = [b"user", project.key().as_ref(), user_wallet.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Record<'info> {
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub user_wallet: Signer<'info>,
}

#[account]
pub struct Project {
    pub authority: Pubkey,
    pub project_key: String,
    pub bump: u8,
}

#[account]
pub struct User {
    pub project: Pubkey,
    pub wallet: Pubkey,
    pub daily_logins: u64,
    pub quests: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid project for user")] 
    InvalidProject,
    #[msg("Invalid user signer")] 
    InvalidUser,
}
