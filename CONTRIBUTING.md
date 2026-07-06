# Contributing to HarvestGuard

Thanks for considering contributing. This project is part of the Stellar
Wave Program, so contributions here can count toward Wave rewards — but
we care about code quality first.

## Before you start

1. Check open issues for one tagged `good-first-issue` if this is your first contribution here.
2. Comment on the issue to claim it before starting work.
3. Want to propose something not covered by an existing issue? Open a new issue first to discuss scope.

## This repo has three languages — know which part you're touching

- **Rust** (`contracts/`) — the oracle and insurance contract logic
- **TypeScript** (`api/`) — the REST layer
- **Python** (`python/oracle_feeder/`) — the off-chain data provider service

Issues are labeled by which part they touch — check for `rust`, `typescript`, or `python` labels alongside the complexity label.

## Issue complexity labels

- `complexity: trivial` — docs, small fixes, config, no contract logic changes
- `complexity: medium` — a contract function, a test suite, a well-scoped feature
- `complexity: high` — cross-cutting changes, new modules, anything touching oracle aggregation or payout logic

## Development setup

```bash
git clone https://github.com/baobab-labs/harvestguard.git
cd harvestguard
cargo build --workspace --target wasm32-unknown-unknown --release
cargo test --workspace
```

## Pull requests

- Keep PRs scoped to one issue
- Include or update tests for any contract logic change
- Describe what you tested manually and how
- Oracle aggregation logic and payout-triggering logic need at least one other reviewer's approval before merge — a bug here either locks a farmer out of a legitimate payout or drains the pool incorrectly

## Code of conduct

Be respectful. This project is meant to protect farmers against real
financial risk — treat contributions and reviews with the seriousness
that implies.
