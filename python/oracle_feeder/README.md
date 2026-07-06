# Oracle Feeder (Python)

Fetches real rainfall data from [Open-Meteo](https://open-meteo.com)
(free, no API key) and submits it to the `weather-oracle` Soroban
contract as a registered data provider.

## Setup

```bash
cd python/oracle_feeder
pip install -r requirements.txt --break-system-packages
cp .env.example .env
```

Fill in `.env`:
- `CONTRACT_ID` — the deployed `weather-oracle` contract ID
- `PROVIDER_SECRET` — a testnet secret key for an address that's been
  registered via `register_provider` (an admin has to do this first — see
  the contract's `docs/adr/002-provider-trust.md`)

## Configure locations

Edit `locations.json` to add/remove locations. Each entry needs a
`location_symbol` (must be ≤ 9 characters — Soroban `Symbol` limits
apply, matching whatever symbol you use when creating a HarvestGuard
policy for that location), a human-readable `name`, and `lat`/`lon`.

## Run

```bash
python main.py
```

This fetches and submits one reading per configured location, then
exits. **Run this periodically** — via cron, a systemd timer, or a
scheduled GitHub Action — rather than as a long-running process. A
rainfall reading every few hours is more than enough freshness for this
use case.

Example crontab entry (every 6 hours):
```
0 */6 * * * cd /path/to/harvestguard/python/oracle_feeder && /usr/bin/python3 main.py >> feeder.log 2>&1
```

## Known limitations

- No retry logic on transient failures yet (see `ISSUES.md` #6 in the repo root)
- Registering as a provider must currently be done separately by an admin — this script only submits readings, it doesn't register itself
