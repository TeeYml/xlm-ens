# xlm-ns

`xlm-ns` is a Rust workspace for a Stellar name service where names like
`timmy.xlm` can resolve to accounts, applications, and cross-chain targets.

## Workspace layout

- `contracts/`: smart contract crates for registry, resolver, registrar, auctions,
  subdomains, NFT ownership, and bridge integrations.
- `packages/xlm-ns-common`: shared validation rules, constants, and domain types.
- `packages/xlm-ns-sdk`: client-side Rust SDK for wallets and dapps.
- `cli/`: operator CLI for register, resolve, renew, transfer, and auction flows.
- `tests/`: cross-crate integration fixtures and scenarios.
- `scripts/`: deploy, invoke, and local setup helpers.

## Status

This repository is currently scaffolded as a modular workspace with placeholder
logic and APIs so features can be implemented incrementally.
