# Architecture

## Two contracts, one workspace

**`weather-oracle`** — a minimal SEP-40-style oracle. Registered
providers submit rainfall readings per location; the contract returns
the median of recent readings when queried, so no single provider
controls the outcome.

**`harvestguard`** — the insurance contract. Holds a liquidity pool
funded by insurers, sells policies to farmers, and pays out
automatically when `weather-oracle` reports a condition met.

## weather-oracle: data flow

1. `register_provider(admin, provider)` — admin approves a data provider address.
2. `submit_reading(provider, location, value_mm)` — a registered provider
   submits a rainfall reading (millimeters, scaled) for a location.
   Readings are stored per-provider, per-location, with a timestamp.
3. `get_rainfall(location)` — returns the **median** of all providers'
   most recent readings for that location, or `None` if no data exists.
   Using a median (not an average, and not any single provider) means
   one bad or malicious reading can't unilaterally move the result if
   there are at least 3 independent providers.

## harvestguard: policy lifecycle

1. `deposit_liquidity(insurer, token, amount)` — an insurer funds the
   shared payout pool.
2. `create_policy(farmer, token, premium, payout, location, threshold_mm, comparison, expiry_ledger)` —
   farmer pays a premium (added to the pool) and buys a policy: if
   rainfall at `location` is below (drought) or above (flood)
   `threshold_mm` before `expiry_ledger`, the policy pays `payout`.
3. `check_and_trigger_payout(policy_id)` — callable by **anyone** (not
   just the farmer), since the trigger condition is objective and
   verifiable by anyone reading the oracle. Reads `weather-oracle`,
   checks the condition, and if met, transfers `payout` from the pool to
   the farmer and marks the policy triggered.

## Known gaps (help wanted)

- **Provider registration is open-admin-controlled but not
  decentralized.** A single admin currently approves providers for
  `weather-oracle`. See `docs/adr/002-provider-trust.md` — this is the
  top-priority issue.
- **No pool solvency check at policy creation.** `create_policy`
  currently doesn't verify the pool has enough balance to cover the
  policy's payout — it could sell more coverage than it can pay out.
  This is a real risk and a high-priority issue.
- **No premium pricing model.** Premiums are currently set arbitrarily
  by whoever calls `create_policy` — there's no actuarial logic relating
  premium to payout, threshold likelihood, or risk.
- **Median requires enough providers.** With fewer than 3 registered
  providers per location, the "median resists a bad actor" property
  doesn't meaningfully hold. Needs a minimum-provider-count check before
  a location's data is considered valid.
- **No audit.** Do not use this to insure real money until it's had a
  security review and the provider-trust and solvency gaps are closed.

## Why cross-contract calls instead of one big contract

Keeping the oracle and the insurance logic as separate contracts means
other future Baobab Labs repos (or third parties) could reuse
`weather-oracle` for a different product (e.g. a different parametric
product, a dashboard, a research tool) without depending on
`harvestguard`'s insurance-specific logic.
