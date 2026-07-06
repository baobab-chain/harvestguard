# ADR 001: Build a custom weather oracle, following SEP-40-style conventions

## Status
Accepted

## Context
Reflector is Stellar's established, actively-used oracle network, and
reusing existing infrastructure is usually the right call rather than
building a parallel system. We evaluated it first.

Reflector, however, is specifically a **price** oracle: its public
interface (`lastprice(asset) -> Option<PriceData>`, `decimals()`)
follows the SEP-40 ecosystem standard for asset pricing — CEX/DEX rates,
FX rates, on-chain asset prices. It has no concept of rainfall,
temperature, or any other physical/environmental measurement, and
extending it to carry arbitrary weather data isn't a natural fit for
what the network and its data-provider nodes are built to do.

## Decision
Build `weather-oracle` as a small, purpose-specific contract rather than
forcing weather data through a price-oracle interface. To keep it
familiar and swappable, it mirrors SEP-40's shape where it makes sense:
a simple read function returning a value + timestamp, and a
provider-submission model similar to how Reflector's node network
pushes price updates.

This means `weather-oracle` is **not** a Reflector integration, and
should not be described as one — it's a standalone oracle built in the
same spirit, for a data type Reflector doesn't cover.

## Consequences
- HarvestGuard doesn't inherit Reflector's existing decentralized node
  network or reputation — `weather-oracle`'s trust model has to be built
  from scratch (see ADR 002).
- If Reflector or another established provider adds weather/climate data
  feeds in the future, `harvestguard`'s oracle-consumer code should be
  revisited to potentially call that instead — `weather-oracle` is a
  bridge, not necessarily a permanent architecture.
- The SEP-40-style shape means a contributor familiar with Reflector
  integration patterns should find `weather-oracle`'s interface
  recognizable, even though it's a different contract.
