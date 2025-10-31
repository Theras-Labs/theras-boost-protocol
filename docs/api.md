# Distribution API (Demo)

Base URL: your `sdkApi`, e.g. `http://localhost:8080`

## POST /events

Body:
```json
{
  "event": "daily_login",
  "props": { "projectKey": "BOLTIS" },
  "projectKey": "BOLTIS",
  "userId": "WALLET_OR_USER_ID",
  "idempotencyKey": "<uuid>",
  "ts": 1730340000000,
  "client": { "ua": "Mozilla/..." }
}
```

Recommendations:
- Verify `idempotencyKey` server-side to avoid duplicate rewards.
- Add HMAC/Ed25519 signature: `X-Signature` header over the raw body.
- Enforce replay protection with `ts` window, rate limits per user/project.

## GET /wallet

Query: `?projectKey=...&userId=...`

Response:
```json
{ "tgem": 120, "partner": 45 }
```

## POST /distribute

Body:
```json
{ "projectKey": "BOLTIS", "userId": "<id>", "token": "PTKN", "amount": 100 }
```

Notes:
- In production, implement custody/escrow for partner tokens and mint TGEM on-chain.
- Log receipts and update on-chain counters via Anchor as needed.
