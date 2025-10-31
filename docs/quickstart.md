# Quickstart

## Install (monorepo)

```sh
bun install
```

## Provider setup

```tsx
import { TherasProvider } from '@theras/ui-sdk';

<TherasProvider
  posthogKey="PH_PUBLIC_KEY"
  sdkApi="http://localhost:8080"
  projectKey="BOLTIS"
  userId="WALLET_OR_USER_ID"
/>
```

## Components

```tsx
import { DailyLogin, QuestBoard, LoyaltyWallet } from '@theras/ui-sdk';

<LoyaltyWallet />
<DailyLogin />
<QuestBoard quests={[{ id: 'q1', title: 'Join Discord' }]} />
```

## Events emitted

- `daily_login` — fired by `DailyLogin`
- `quest_complete` — fired by `QuestBoard` with `{ questId }`

## Wallet endpoint (demo)

- `GET /wallet?projectKey=BOLTIS&userId=<id>` returns `{ tgem, partner }`

## Notes

- PostHog events are sent client-side with minimal network overhead.
- When `sdkApi` is provided, events also POST to `/events` with `idempotencyKey`, `ts`, and `client.ua`.
