# Theras Monorepo

A minimal scaffold of Theras Protocol components:

- packages/core-sdk — PostHog-based client tracking + API forwarding
- packages/ui-sdk — React provider + drop-in components
- anchor/ — Anchor Solana program for project/user/event counters
- services/distribution — Go demo API for events, wallet, distribution
- docs/ — Developer docs and SVG flows

## Install

- Prereqs: Bun, Go 1.21+, Rust + Anchor (for on-chain), Solana CLI (for localnet).
- From monorepo root:

```sh
bun install
```

## Build SDKs

```sh
# core-sdk
bun run build --cwd packages/core-sdk

# ui-sdk
bun run build --cwd packages/ui-sdk
```

## Run demo API (distribution)

```sh
# serves on http://localhost:8080
# endpoints: POST /events, GET /wallet, POST /distribute
bun x go run services/distribution/main.go
```

## Using in an app

```tsx
import { TherasProvider } from '@theras/ui-sdk';
import { DailyLogin, QuestBoard, LoyaltyWallet } from '@theras/ui-sdk';

<TherasProvider
  posthogKey="PH_PUBLIC_KEY"
  sdkApi="http://localhost:8080"
  projectKey="BOLTIS"
  userId="WALLET_OR_USER_ID"
>
  <LoyaltyWallet />
  <DailyLogin />
  <QuestBoard quests={[{ id: 'q1', title: 'Join Discord' }]} />
</TherasProvider>
```

## Anchor program (local)

- Program ID: `Thrs1111111111111111111111111111111111111111`
- See `anchor/` for:
  - `initialize_project(project_key)`
  - `register_user()`
  - `record_daily_login()`
  - `record_quest()`

## Docs

See `docs/` for quickstart, API, on-chain overview, and SVG flows.
