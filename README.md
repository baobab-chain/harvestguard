# HarvestGuard

**Parametric crop micro-insurance on Soroban — rainfall crosses a threshold, payout happens automatically.**

Part of [Baobab Chain Labs](https://github.com/baobab-chain) — built on [Stellar](https://stellar.org).

---

## The problem

Smallholder farmers across Nigeria, the Sahel, and other tropical
agrarian economies can't get insured against drought or flood, because
traditional insurance needs a claims adjuster to visit and assess damage
— and that visit costs more than a small policy is worth. So the
farmers who most need protection against a bad rainy season are exactly
the ones the insurance industry can't profitably serve.

**Parametric insurance** sidesteps this: instead of assessing damage
after the fact, the policy pays out automatically the moment an
objective, verifiable condition is met (e.g. "less than 20mm of rain
fell in this 30-day window"). No adjuster, no claim, no dispute over
damage — just a threshold and a trigger.

## What this is

Two contracts working together:

1. **`weather-oracle`** — a small, purpose-built oracle contract that
   accepts rainfall readings from multiple registered data providers per
   location, and returns the **median** reading when queried (so no
   single provider can unilaterally trigger a payout with a bad number).
2. **`harvestguard`** — the insurance contract. A farmer buys a policy
   tied to a location and a rainfall threshold; an insurer/liquidity
   provider funds the payout pool; anyone can call `check_and_trigger_payout`
   once the oracle shows the threshold condition is met, and the payout
   fires automatically.

A Python service (`python/oracle_feeder/`) fetches real rainfall data
from a public weather API and submits it to `weather-oracle` — this is
the "off-chain data provider" piece every oracle network needs.

## Why not just use Reflector?

We checked. [Reflector](https://reflector.network) is Stellar's
established oracle network, but it's specifically an **asset price**
oracle (SEP-40 interface: CEX/DEX rates, FX rates) — it doesn't carry
rainfall or weather data. So `weather-oracle` is a small custom oracle
contract, deliberately built to follow the same SEP-40-style
conventions (a `lastprice`-equivalent read function, provider-submitted
values) so it's a familiar pattern for anyone who's worked with Reflector,
extendable to a real decentralized provider network later. See
[`docs/adr/001-oracle-design.md`](docs/adr/001-oracle-design.md) for the
full reasoning.

## Why Stellar / Soroban

- **Payout enforced by code**, not a claims department's judgment call
- **Low fees** make small-ticket policies (a few dollars of premium) economically viable to service
- **USDC settlement** means the payout reaches a farmer's wallet in seconds once triggered, no waiting on a claims process

## Status

Early-stage / MVP skeleton. Not audited. See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and both ADRs in
[`docs/adr/`](docs/adr/) for the current design and known gaps — in
particular, **weather data providers are currently open-registration**
(same class of gap as the agent/issuer/warehouse-operator trust
questions in the other Baobab Labs repos). See [`ISSUES.md`](ISSUES.md).

## Repo layout

- **`contracts/weather-oracle/`** — the oracle contract (Rust)
- **`contracts/harvestguard/`** — the insurance contract (Rust), calls into `weather-oracle`
- **`python/oracle_feeder/`** — Python service that fetches real rainfall data and submits it on-chain
- **`api/`** — a NestJS REST layer over both contracts
- **`web/`** — static landing page explaining the protocol

## Getting started

**Contracts** (workspace — builds both at once):
```bash
cargo build --workspace --target wasm32-unknown-unknown --release
cargo test --workspace
```

**Python oracle feeder:**
```bash
cd python/oracle_feeder
pip install -r requirements.txt --break-system-packages
cp .env.example .env   # fill in PROVIDER_SECRET and CONTRACT_ID
python main.py
```

**API:**
```bash
cd api
npm install
cp .env.example .env
npm run start:dev
```

**Landing page:**
```bash
cd web
python3 -m http.server 8084
```

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for how to contribute.

## License

MIT — see [`LICENSE`](LICENSE).
