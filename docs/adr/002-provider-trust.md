# ADR 002: Admin-approved providers for now, decentralized network planned as v2

## Status
Accepted (temporary — revisit before any real-money use)

## Context
`weather-oracle` needs data providers to submit rainfall readings.
Fully open submission (anyone can submit anything) would make the median
trivially manipulable by registering many colluding addresses. Fully
centralized (one trusted admin submits all data) defeats the point of an
oracle and reintroduces a single point of failure/trust.

## Decision
For the MVP, a single admin address approves which providers can submit
readings (`register_provider`), and `get_rainfall` takes the median
across whatever providers have reported for a location. This is
explicitly a stepping stone:
- It's better than a single data source, because the median resists one
  bad reading among several honest ones.
- It's not yet decentralized, because the admin controls who counts as
  "a provider" in the first place.

Real decentralization needs one of:
- A staking/bonding model (providers stake collateral, slashed for
  readings that deviate far from the eventual median — similar pattern
  to `remit-last-mile`'s agent bonding idea)
- A permissionless network with reputation built over time
- Integration with an existing decentralized weather-data network, if
  one becomes available on Stellar

## Consequences
- Anyone testing/contributing right now needs the admin to register them
  as a provider — expected and fine for testnet development
- This contract must not be used to trigger real payouts until a
  decentralized provider-trust model exists and there's a
  minimum-provider-count enforced per location (see `docs/ARCHITECTURE.md`
  known gaps)
- Every README/API doc references this limitation explicitly
