use anchor_lang::prelude::*;

declare_id!("BQm8t9GpXgyX1atwWGaTpYvjeyGZNcwTczJDauSM3f1v");

#[program]
pub mod theras_protocol {
    use super::*;

    /// Initialize a new project with configuration
    pub fn initialize_project(
        ctx: Context<InitializeProject>,
        project_key: String,
        tgem_plus_enabled: bool,
    ) -> Result<()> {
        require!(project_key.len() <= 32, ErrorCode::ProjectKeyTooLong);
        
        let project = &mut ctx.accounts.project;
        project.authority = ctx.accounts.authority.key();
        project.project_key = project_key.clone();
        project.tgem_plus_enabled = tgem_plus_enabled;
        project.total_users = 0;
        project.total_events = 0;
        project.bump = ctx.bumps.project;
        
        emit!(ProjectCreated {
            project: ctx.accounts.project.key(),
            authority: project.authority,
            project_key,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Project initialized: {}", project_key);
        Ok(())
    }

    /// Register a new user for a project
    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.project = ctx.accounts.project.key();
        user.wallet = ctx.accounts.user_wallet.key();
        user.daily_logins = 0;
        user.quests = 0;
        user.referrals = 0;
        user.total_tgem_earned = 0;
        user.last_login = 0;
        user.bump = ctx.bumps.user;
        
        // Update project stats
        let project = &mut ctx.accounts.project;
        project.total_users = project.total_users.saturating_add(1);
        
        emit!(UserRegistered {
            user: ctx.accounts.user_wallet.key(),
            project: ctx.accounts.project.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("User registered: {}", ctx.accounts.user_wallet.key());
        Ok(())
    }

    /// Record daily login event
    pub fn record_daily_login(ctx: Context<RecordEvent>) -> Result<()> {
        require!(
            ctx.accounts.user.project == ctx.accounts.project.key(),
            ErrorCode::InvalidProject
        );
        require!(
            ctx.accounts.user.wallet == ctx.accounts.user_wallet.key(),
            ErrorCode::InvalidUser
        );
        
        let current_time = Clock::get()?.unix_timestamp;
        let user = &mut ctx.accounts.user;
        
        // Check if already logged in today (86400 seconds = 1 day)
        require!(
            current_time - user.last_login >= 86400,
            ErrorCode::AlreadyLoggedInToday
        );
        
        user.daily_logins = user.daily_logins.saturating_add(1);
        user.last_login = current_time;
        
        // Update project stats
        let project = &mut ctx.accounts.project;
        project.total_events = project.total_events.saturating_add(1);
        
        emit!(EventRecorded {
            user: ctx.accounts.user_wallet.key(),
            project: ctx.accounts.project.key(),
            event_type: EventType::DailyLogin,
            count: user.daily_logins,
            timestamp: current_time,
        });
        
        Ok(())
    }

    /// Record quest completion
    pub fn record_quest(
        ctx: Context<RecordEvent>,
        quest_id: String,
    ) -> Result<()> {
        require!(
            ctx.accounts.user.project == ctx.accounts.project.key(),
            ErrorCode::InvalidProject
        );
        require!(
            ctx.accounts.user.wallet == ctx.accounts.user_wallet.key(),
            ErrorCode::InvalidUser
        );
        require!(quest_id.len() <= 64, ErrorCode::QuestIdTooLong);
        
        let user = &mut ctx.accounts.user;
        user.quests = user.quests.saturating_add(1);
        
        // Update project stats
        let project = &mut ctx.accounts.project;
        project.total_events = project.total_events.saturating_add(1);
        
        emit!(EventRecorded {
            user: ctx.accounts.user_wallet.key(),
            project: ctx.accounts.project.key(),
            event_type: EventType::Quest,
            count: user.quests,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Record referral
    pub fn record_referral(
        ctx: Context<RecordEvent>,
        referred_user: Pubkey,
    ) -> Result<()> {
        require!(
            ctx.accounts.user.project == ctx.accounts.project.key(),
            ErrorCode::InvalidProject
        );
        require!(
            ctx.accounts.user.wallet == ctx.accounts.user_wallet.key(),
            ErrorCode::InvalidUser
        );
        
        let user = &mut ctx.accounts.user;
        user.referrals = user.referrals.saturating_add(1);
        
        // Update project stats
        let project = &mut ctx.accounts.project;
        project.total_events = project.total_events.saturating_add(1);
        
        emit!(EventRecorded {
            user: ctx.accounts.user_wallet.key(),
            project: ctx.accounts.project.key(),
            event_type: EventType::Referral,
            count: user.referrals,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Update TGEM earned (called by backend after minting)
    pub fn update_tgem_earned(
        ctx: Context<UpdateTGEM>,
        amount: u64,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.total_tgem_earned = user.total_tgem_earned.saturating_add(amount);
        
        msg!("Updated TGEM earned for user: {} (+{})", ctx.accounts.user_wallet.key(), amount);
        Ok(())
    }

    /// Update project configuration
    pub fn update_project_config(
        ctx: Context<UpdateProject>,
        tgem_plus_enabled: Option<bool>,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        
        if let Some(enabled) = tgem_plus_enabled {
            project.tgem_plus_enabled = enabled;
            msg!("TGEM+ enabled: {}", enabled);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(project_key: String)]
pub struct InitializeProject<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Project::LEN,
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
        space = 8 + User::LEN,
        seeds = [b"user", project.key().as_ref(), user_wallet.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordEvent<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    
    #[account(mut)]
    pub user: Account<'info, User>,
    
    pub user_wallet: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateTGEM<'info> {
    pub project: Account<'info, Project>,
    
    #[account(
        mut,
        constraint = authority.key() == project.authority @ ErrorCode::Unauthorized
    )]
    pub user: Account<'info, User>,
    
    /// CHECK: User wallet for reference
    pub user_wallet: AccountInfo<'info>,
    
    #[account(constraint = authority.key() == project.authority @ ErrorCode::Unauthorized)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateProject<'info> {
    #[account(
        mut,
        constraint = authority.key() == project.authority @ ErrorCode::Unauthorized
    )]
    pub project: Account<'info, Project>,
    
    pub authority: Signer<'info>,
}

#[account]
pub struct Project {
    pub authority: Pubkey,        // 32
    pub project_key: String,      // 4 + 32 = 36
    pub tgem_plus_enabled: bool,  // 1
    pub total_users: u64,         // 8
    pub total_events: u64,        // 8
    pub bump: u8,                 // 1
}

impl Project {
    pub const LEN: usize = 32 + 36 + 1 + 8 + 8 + 1;
}

#[account]
pub struct User {
    pub project: Pubkey,          // 32
    pub wallet: Pubkey,           // 32
    pub daily_logins: u64,        // 8
    pub quests: u64,              // 8
    pub referrals: u64,           // 8
    pub total_tgem_earned: u64,   // 8
    pub last_login: i64,          // 8
    pub bump: u8,                 // 1
}

impl User {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1;
}

#[event]
pub struct ProjectCreated {
    pub project: Pubkey,
    pub authority: Pubkey,
    pub project_key: String,
    pub timestamp: i64,
}

#[event]
pub struct UserRegistered {
    pub user: Pubkey,
    pub project: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EventRecorded {
    pub user: Pubkey,
    pub project: Pubkey,
    pub event_type: EventType,
    pub count: u64,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventType {
    DailyLogin,
    Quest,
    Referral,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid project for user")]
    InvalidProject,
    
    #[msg("Invalid user signer")]
    InvalidUser,
    
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Project key too long (max 32 chars)")]
    ProjectKeyTooLong,
    
    #[msg("Quest ID too long (max 64 chars)")]
    QuestIdTooLong,
    
    #[msg("Already logged in today")]
    AlreadyLoggedInToday,
}
