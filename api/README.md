# HarvestGuard API

A NestJS REST layer over both the `harvestguard` insurance contract and
the `weather-oracle` contract. Note this API talks to **two** contract
IDs — see `.env.example`.

## ⚠️ Before you use this for anything real

Testnet demo scaffold. Neither contract has been audited; the pool has
no solvency check (see `docs/ARCHITECTURE.md` known gaps in the repo
root); and every write endpoint here signs with a shared service key
rather than the actual farmer's/insurer's/admin's own wallet. Tracked in
`ISSUES.md`.

## Setup

```bash
cd api
npm install
cp .env.example .env
# fill in HARVESTGUARD_CONTRACT_ID and ORACLE_CONTRACT_ID after deploying both contracts
npm run start:dev
```

## Endpoints

| Method | Path | Description |
|---|---|---|
| `POST` | `/oracle/providers` | Admin registers a weather data provider. Body: `{ adminAddress, providerAddress }` |
| `GET` | `/oracle/:location` | Get current median rainfall for a location |
| `POST` | `/policies/liquidity` | Insurer deposits into the payout pool. Body: `{ insurerAddress, tokenContractId, amount }` |
| `POST` | `/policies` | Farmer buys a policy. Body: `{ farmerAddress, tokenContractId, premium, payout, location, thresholdMm, comparison, expiryLedger }` — `comparison` is `"Below"` (drought) or `"Above"` (flood) |
| `GET` | `/policies/:id` | Fetch a policy's state |
| `POST` | `/policies/:id/check-and-trigger` | Anyone can call this — checks the oracle and fires payout if the condition is met |

## Generating a testnet service account

```bash
node -e "console.log(require('@stellar/stellar-sdk').Keypair.random().secret())"
```

Fund it via [Friendbot](https://friendbot.stellar.org) before use.
