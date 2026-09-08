# StellarForge

> Rust-first developer infrastructure for building production-ready applications on Stellar and Soroban.

[![CI](https://github.com/xxvii-xiaxia/StellarForge/actions/workflows/ci.yml/badge.svg)](https://github.com/xxvii-xiaxia/StellarForge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

StellarForge is an open-source, Rust-first infrastructure toolkit that connects the pieces
Stellar/Soroban application developers otherwise have to wire together themselves: transaction
construction and submission, payments, Soroban contract interaction, event processing, and
developer tooling.

## Why StellarForge exists

Stellar already has excellent official infrastructure — the [Stellar SDK](https://github.com/stellar/rs-stellar-xdr),
the [Soroban Rust SDK](https://github.com/stellar/rs-soroban-sdk), [`stellar-cli`](https://github.com/stellar/stellar-cli),
Horizon, and Stellar RPC. StellarForge is **not** a replacement for any of these. It is a
higher-level layer that composes them into practical, reusable workflows so that individual
applications stop reimplementing the same transaction/payment/event plumbing from scratch.

See [`project.md`](project.md) for the full architecture, roadmap, and contribution model.

## Status

Early foundation stage. The initial focus (see [`project.md` §25](project.md#25-initial-mvp)) is a
small, well-tested core: network configuration, address validation, transaction construction and
submission, XLM/SEP-41 payments, basic Soroban contract invocation, and CLI inspection tools.

## Repository layout

```
crates/            Rust crates (core, tx, payments, soroban, events, indexer, wallet, webhooks, cli)
contracts/         Reference Soroban smart contracts
examples/          Example usage of the crates
integration-tests/ Cross-crate integration tests
docs/              Architecture and design documentation
```

## Getting started

```sh
git clone git@github.com:xxvii-xiaxia/StellarForge.git
cd StellarForge
cargo build
cargo test
```

## Contributing

StellarForge is designed for external contributors. Issues are scoped to be clear, technically
specific, and independently testable. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for setup and
workflow, and [`project.md` §21–23](project.md#21-contribution-model) for how issues are written.

## License

MIT — see [`LICENSE`](LICENSE).
