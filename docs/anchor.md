# Anchor Program Overview

Program ID: `Thrs1111111111111111111111111111111111111111`

## Accounts
- `Project` — authority, project_key, bump
- `User` — project, wallet, daily_logins, quests, bump

## Instructions
- `initialize_project(project_key: String)` — creates PDA for project
- `register_user()` — creates `User` PDA
- `record_daily_login()` — increments daily_logins
- `record_quest()` — increments quests

## Suggested extensions
- `ProjectConfig` PDA — earn rates, cooldowns, caps
- `EventReceipt` PDA — on-chain idempotency
- `Referral` PDA — track referrer/referee attribution
- Treasury adapters — partner token escrow + TGEM mint hooks

## Local testing
- Use `anchor build` and `anchor test` against localnet.
- Wire service to submit on-chain instructions on validated events.
