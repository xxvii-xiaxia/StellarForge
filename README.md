# StellarForge

> Rust-first developer infrastructure for building production-ready applications on Stellar and Soroban.

[![CI](https://github.com/xxvii-xiaxia/StellarForge/actions/workflows/ci.yml/badge.svg)](https://github.com/xxvii-xiaxia/StellarForge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](Cargo.toml)

StellarForge is an open-source, Rust-first infrastructure toolkit that connects the pieces
Stellar/Soroban application developers otherwise have to wire together themselves: transaction
construction and submission, confirmation tracking, payments, and (in progress) Soroban contract
interaction, event processing, and developer tooling.

## Why StellarForge exists

Stellar already has excellent official infrastructure — the [Stellar SDK](https://github.com/stellar/rs-stellar-xdr),
the [Soroban Rust SDK](https://github.com/stellar/rs-soroban-sdk), [`stellar-cli`](https://github.com/stellar/stellar-cli),
Horizon, and Stellar RPC. StellarForge is **not** a replacement for any of these. It is a
higher-level layer that composes them into practical, reusable workflows — transaction
construction, fee/memo/time-bounds handling, submission, confirmation polling, and payments — so
that individual applications stop reimplementing the same plumbing from scratch.

StellarForge never handles private keys or signs transactions. Callers build an unsigned
transaction, sign it with whatever mechanism they already trust, and hand the signed envelope
back to StellarForge for submission. See [`project.md` §18](project.md#18-security) for the full
rationale.

## Status

**Early, actively developed.** The crates below are real, tested, and used by each other — this
is not a scaffold. The current focus (see [`project.md` §25](project.md#25-initial-mvp)) is a
small, well-tested transaction and payments core; Soroban contract tooling, event processing, and
the CLI are next.

| Crate | Status | What it does |
|---|---|---|
| [`stellarforge-core`](crates/stellarforge-core) | ✅ Implemented | Network config (mainnet/testnet/custom), Stellar address (strkey) validation, asset representation, shared error types |
| [`stellarforge-tx`](crates/stellarforge-tx) | ✅ Implemented | Transaction builder (fees, memo, time bounds, operation limits), `Payment` and `InvokeContract` operations, Horizon submission, confirmation polling |
| [`stellarforge-payments`](crates/stellarforge-payments) | ✅ Implemented | XLM payments and SEP-41 token payments built on top of `stellarforge-tx` |
| `stellarforge-soroban` | 🚧 Planned | Contract invocation, simulation, and result/error decoding |
| `stellarforge-events` | 🚧 Planned | Unified event decoding for transactions, payments, and contract events |
| `stellarforge-cli` | 🚧 Planned | `stellarforge tx inspect`, `contract inspect`, `payment verify`, etc. |

All three implemented crates build and pass CI (`fmt`, `clippy -D warnings`, `test`) on every
push and pull request — see the badge above.

## Quickstart

```sh
git clone https://github.com/xxvii-xiaxia/StellarForge.git
cd StellarForge
cargo build --workspace
cargo test --workspace
```

### Building and submitting an XLM payment

```rust
use stellarforge_core::NetworkConfig;
use stellarforge_payments::PaymentRequest;
use stellarforge_tx::{submit_transaction, wait_for_confirmation, ConfirmationOptions};

async fn send_payment() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure the network.
    let config = NetworkConfig::testnet();

    // 2. Build an unsigned payment transaction (source account + next sequence number).
    let request = PaymentRequest::xlm("GDESTINATIONADDR...", "25").with_memo("invoice-1038");
    let tx = stellarforge_payments::build_payment_transaction(
        "GSOURCEACCOUNT...",
        /* sequence_number */ 1234567890,
        /* base_fee_stroops */ 100,
        request,
    )?;

    // 3. Encode `tx` to XDR and sign it with your own key-management solution — StellarForge
    //    does not sign transactions or touch private keys (see `project.md` §18).
    let signed_envelope_xdr: String = sign_with_my_wallet(&tx)?;

    // 4. Submit the signed envelope to Horizon.
    let result = submit_transaction(&config, &signed_envelope_xdr).await?;
    println!("submitted: {} (ledger {})", result.hash, result.ledger);

    // 5. If you obtained a hash from elsewhere (e.g. Soroban RPC's async sendTransaction)
    //    instead of submitting through Horizon directly, wait for it to reach a final state.
    wait_for_confirmation(&config, &result.hash, ConfirmationOptions::default()).await?;
    Ok(())
}
```

Each crate's `lib.rs` documents its own module in more depth; the crates are also independently
usable — you don't have to adopt all of them at once.

## Repository layout

```
crates/
├── stellarforge-core/      network config, address validation, assets, errors
├── stellarforge-tx/        transaction builder, submission, confirmation tracking
└── stellarforge-payments/  XLM and SEP-41 payment helpers built on stellarforge-tx
```

Planned additions (`stellarforge-soroban`, `stellarforge-events`, `stellarforge-cli`, and more)
are tracked in [`project.md`](project.md), which is the full architecture and roadmap document
for the project.

## Testing

Every crate has unit tests covering both the success path and validation/error paths (invalid
addresses, malformed amounts, fee/operation limits, Horizon rejection and timeout handling, etc.).
Network-facing code (`submit_transaction`, `wait_for_confirmation`) is tested against a mocked
Horizon (via [`wiremock`](https://docs.rs/wiremock)) with no live network dependency, so the full
suite runs offline and deterministically in CI.

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Contributing

StellarForge is designed for external contributors. Issues are scoped to be clear, technically
specific, and independently testable — see the backlog in
[`project.md` §22](project.md#22-initial-contributor-backlog) and the issue-design philosophy in
[§23](project.md#23-issue-design-philosophy). See [`CONTRIBUTING.md`](CONTRIBUTING.md) for local
setup, workflow, and PR expectations, and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) for community
guidelines.

## Security

See [`SECURITY.md`](SECURITY.md) for the project's vulnerability reporting process. StellarForge
never handles private keys or signs transactions on a caller's behalf — see
[`project.md` §18](project.md#18-security) for the full security model.

## License

MIT — see [`LICENSE`](LICENSE).
