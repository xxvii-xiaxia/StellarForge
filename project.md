# StellarForge

> Rust-first developer infrastructure for building production-ready applications on Stellar and Soroban.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Problem](#2-problem)
3. [Vision](#3-vision)
4. [Project Goals](#4-project-goals)
5. [Non-Goals](#5-non-goals)
6. [Why Rust?](#6-why-rust)
7. [Architecture](#7-architecture)
8. [Repository Structure](#8-repository-structure)
9. [Core Components](#9-core-components)
10. [Transaction Engine](#10-transaction-engine)
11. [Payment Module](#11-payment-module)
12. [Soroban Module](#12-soroban-module)
13. [Event Engine](#13-event-engine)
14. [Indexer](#14-indexer)
15. [Webhook Engine](#15-webhook-engine)
16. [Rust CLI](#16-rust-cli)
17. [Smart Contracts](#17-smart-contracts)
18. [Security](#18-security)
19. [Testing Strategy](#19-testing-strategy)
20. [Developer Experience](#20-developer-experience)
21. [Contribution Model](#21-contribution-model)
22. [Initial Contributor Backlog](#22-initial-contributor-backlog)
23. [Issue Design Philosophy](#23-issue-design-philosophy)
24. [Drips Wave Strategy](#24-drips-wave-strategy)
25. [Initial MVP](#25-initial-mvp)
26. [MVP Success Criteria](#26-mvp-success-criteria)
27. [Long-Term Roadmap](#27-long-term-roadmap)
28. [Project Principles](#28-project-principles)
29. [What Makes StellarForge Different](#29-what-makes-stellarforge-different)
30. [Definition of Done](#30-definition-of-done)
31. [Final Project Statement](#31-final-project-statement)

---

## 1. Project Overview

StellarForge is an open-source, Rust-first infrastructure toolkit designed to simplify the development of applications on the **Stellar network** and **Soroban smart-contract platform**.

The project provides reusable Rust libraries, command-line tooling, transaction infrastructure, Soroban utilities, event processing, and developer-focused services that remove repetitive infrastructure work from Stellar application developers.

StellarForge is **not** intended to replace the official Stellar SDK, Soroban SDK, Stellar CLI, Horizon, or Stellar RPC. Instead, it builds on top of the existing Stellar ecosystem and provides higher-level infrastructure that connects these components into practical workflows.

> **Core principle:** Developers should be able to focus on application logic instead of repeatedly rebuilding Stellar infrastructure.

---

## 2. Problem

Building a production Stellar or Soroban application often requires developers to implement and maintain several independent pieces of infrastructure:

- Transaction construction, simulation, submission, and confirmation tracking
- Retry handling and error decoding
- XDR handling and Soroban contract interaction
- Event decoding and token operations
- Payment verification and wallet integration
- Transaction monitoring, event indexing, and webhook delivery
- Local/testnet development tooling

The individual components already exist across the Stellar ecosystem. The problem is that application developers frequently have to connect them themselves and rebuild similar infrastructure repeatedly.

**StellarForge aims to provide a common, composable foundation.**

---

## 3. Vision

StellarForge aims to become a reliable open-source infrastructure layer for Stellar developers.

```
         Stellar / Soroban
                │
                ▼
        ┌───────────────┐
        │  StellarForge │
        └───────┬───────┘
                │
┌───────────────┼────────────────┐
│               │                │
▼               ▼                ▼
Transactions   Soroban         Events
│               Tools            │
▼               ▼                ▼
Payments    Contracts         Indexing
│               │                │
└───────────────┼────────────────┘
                ▼
         Developer APIs
                │
                ▼
     Stellar Applications
```

---

## 4. Project Goals

### Primary Goals

- Provide production-quality Rust infrastructure for Stellar applications
- Make common Stellar transaction workflows significantly easier
- Provide reusable Soroban development utilities
- Provide reliable transaction monitoring
- Provide event decoding and processing infrastructure
- Provide a Rust CLI for common developer workflows
- Provide strong automated testing
- Make the project easy for external developers to contribute to
- Maintain clear, well-scoped issues suitable for open-source contribution
- Become a useful foundation for other Stellar projects

### Secondary Goals

The project should eventually provide:

- TypeScript bindings/SDK
- REST API and webhooks
- Event indexing
- Local development utilities
- Contract development utilities
- Payment infrastructure
- Developer observability tools

---

## 5. Non-Goals

StellarForge will **not** attempt to:

- Replace the official Stellar SDK, Soroban Rust SDK, or Stellar CLI
- Become a general-purpose blockchain framework
- Build a complete wallet application, centralized exchange, or consumer-facing payment application
- Build a proprietary hosted infrastructure service
- Hide Stellar concepts behind an unnecessarily complicated abstraction

The project should remain focused on **open-source developer infrastructure**.

---

## 6. Why Rust?

Rust is a first-class requirement of this project — not merely for smart contracts, but for the entire core infrastructure. The project requires:

- Strong type safety and high performance
- Predictable resource usage and reliable concurrency
- Excellent binary/WASM support
- Strong tooling for systems infrastructure
- Native alignment with Soroban smart-contract development

Stellar's Soroban contracts are written in Rust, and Stellar maintains an official Rust SDK for Soroban. StellarForge follows that foundation end to end.

---

## 7. Architecture

### High-Level Architecture

```
                     Stellar Network
                            │
            ┌───────────────┼───────────────┐
            │               │               │
         Horizon           RPC           Ledger
            │               │               │
            └───────────────┼───────────────┘
                            │
                            ▼
                   StellarForge Core
                            │
    ┌───────────────────────┼────────────────────────┐
    │                       │                        │
    ▼                       ▼                        ▼
Transaction Engine    Soroban Engine          Event Engine
    │                       │                        │
    ▼                       ▼                        ▼
Payment Module        Contract Tools            Indexer
    │                       │                        │
    └───────────────────────┼────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
          Rust CLI                  Developer API
              │                           │
              └─────────────┬─────────────┘
                            ▼
                     Stellar Applications
```

---

## 8. Repository Structure

```
stellarforge/
│
├── crates/
│   ├── stellarforge-core/
│   ├── stellarforge-tx/
│   ├── stellarforge-payments/
│   ├── stellarforge-soroban/
│   ├── stellarforge-events/
│   ├── stellarforge-indexer/
│   ├── stellarforge-wallet/
│   ├── stellarforge-webhooks/
│   └── stellarforge-cli/
│
├── contracts/
│   ├── payment/
│   ├── escrow/
│   └── subscription/
│
├── examples/
│   ├── payment/
│   ├── soroban/
│   ├── events/
│   └── transactions/
│
├── integration-tests/
├── docs/
├── scripts/
│
├── .github/
│   ├── workflows/
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
│
├── Cargo.toml
├── Cargo.lock
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── LICENSE
└── README.md
```

---

## 9. Core Components

### 9.1 StellarForge Core

The core crate provides shared primitives:

- Network configuration (mainnet/testnet/custom)
- RPC and Horizon configuration
- Common error types
- Address validation and asset representation
- Configuration management, logging, and retry configuration

```rust
let config = StellarConfig::testnet();
let client = StellarForge::new(config)?;
```

---

## 10. Transaction Engine

One of the project's primary components. Provides:

- Transaction construction, preparation, and fee configuration
- Sequence management, simulation, and submission
- Confirmation tracking, retry handling, and error decoding
- Transaction status and inspection

```rust
let payment = Payment::builder()
    .from(source)
    .to(destination)
    .asset(usdc)
    .amount("25")
    .build()?;

let result = forge
    .transactions()
    .submit(payment)
    .await?;

println!("Transaction: {}", result.hash());
```

**Confirmation tracking:**

```rust
let result = forge
    .transactions()
    .wait_for_confirmation(tx_hash)
    .await?;
```

---

## 11. Payment Module

Provides reusable payment infrastructure:

- XLM and SEP-41 token payments
- Payment validation, memo support, and transaction verification
- Payment status and event generation

```rust
let payment = forge
    .payments()
    .send(PaymentRequest {
        asset: Asset::Usdc,
        amount: "25".into(),
        destination,
        memo: Some("invoice-1038".into()),
    })
    .await?;
```

---

## 12. Soroban Module

Higher-level utilities for interacting with smart contracts:

- Contract address validation and invocation
- Simulation, argument encoding, and result decoding
- Contract error decoding, event extraction, and metadata inspection
- Testnet deployment utilities

```rust
let result = forge
    .soroban()
    .contract(contract_id)
    .method("transfer")
    .args(args)
    .simulate()
    .await?;
```

> The project builds on the official Soroban Rust ecosystem rather than recreating the underlying SDK.

---

## 13. Event Engine

A unified event-processing interface supporting:

- Transaction, payment, contract, and Soroban events
- Token transfers and custom application events
- Both real-time and historical processing

```rust
forge.events()
    .watch(contract_id)
    .on_event(|event| {
        println!("{:?}", event);
    });
```

---

## 14. Indexer

Optional infrastructure for projects that need structured Stellar/Soroban data.

```
Stellar Ledger
      │
      ▼
Rust Ingestion Engine
      │
      ├── Transactions
      ├── Operations
      ├── Payments
      ├── Contract Events
      ├── Token Transfers
      └── Accounts
             │
             ▼
        PostgreSQL
             │
             ▼
       Query Layer
```

The indexer focuses initially on correctness and reliability rather than competing with large hosted indexing providers.

---

## 15. Webhook Engine

Allows applications to subscribe to StellarForge events.

```
Stellar event → Event Engine → Webhook Dispatcher → Application
```

**Example payload:**

```json
{
  "type": "payment.confirmed",
  "network": "testnet",
  "asset": "USDC",
  "amount": "25",
  "source": "G...",
  "destination": "G...",
  "transaction": "..."
}
```

The webhook system will eventually support event filtering, signing, retries with exponential backoff, delivery history, idempotency, and dead-letter handling.

---

## 16. Rust CLI

StellarForge provides a developer-focused CLI:

```sh
stellarforge init
stellarforge network list

stellarforge account inspect <ADDRESS>

stellarforge tx inspect <HASH>
stellarforge tx simulate <FILE>
stellarforge tx submit <FILE>

stellarforge contract inspect <ADDRESS>
stellarforge contract call <ADDRESS>

stellarforge events watch <ADDRESS>
stellarforge payment verify <HASH>
```

The CLI prioritizes useful developer workflows rather than replacing `stellar-cli`.

---

## 17. Smart Contracts

Reference Soroban contracts demonstrating how the infrastructure can be used.

### Payment Contract
- Create, release, cancel, and refund payments
- Query payment state and emit payment events

### Escrow Contract
- Create and fund escrow
- Release or refund funds, manage dispute state

### Subscription Contract *(future)*
- Create subscriptions with defined payment intervals
- Track state, execute payments, and handle cancellation

Contracts are deliberately small and auditable — primarily reference implementations and integration targets.

---

## 18. Security

Security is a core project requirement:

- `SECURITY.md` and dependency auditing via `cargo audit`
- Static analysis and automated tests
- Fuzz testing for critical parsers
- Input validation and safe error handling
- No private-key logging or secret storage in source code
- Signed release artifacts where practical

> **Private keys should never be written to logs.** Transaction signing is designed so applications can provide their own signing mechanism. StellarForge avoids unnecessarily handling private keys.

---

## 19. Testing Strategy

### Unit Tests
Every Rust crate has unit tests for core logic.

### Integration Tests
- Transactions, payments, Soroban calls
- Event decoding, error handling, indexing

### Contract Tests
Every Soroban contract includes unit tests, authorization tests, failure-path tests, event tests, and integration tests.

### Testnet Tests
Where practical, CI or scheduled workflows execute selected integration tests against Stellar testnet.

---

## 20. Developer Experience

A contributor should be able to clone and start working immediately:

```sh
git clone <repository>
cd stellarforge
cargo build
cargo test
```

Documentation covers architecture, development setup, running tests, creating a contract, using the CLI, filing issues, and submitting PRs.

---

## 21. Contribution Model

StellarForge is designed specifically for external contributors. Issues should be:

- Clearly scoped, technically specific, and independently testable
- Small enough to complete within a Wave where appropriate
- Assigned a clear difficulty and linked to relevant documentation

**Issue labels:**

`good first issue` · `help wanted` · `rust` · `soroban` · `transactions` · `payments` · `indexer` · `events` · `cli` · `documentation` · `testing` · `security`

---

## 22. Initial Contributor Backlog

### Beginner

| # | Issue |
|---|-------|
| 001 | Add Stellar address validation |
| 002 | Add network configuration (mainnet/testnet/custom RPC) |
| 003 | Add structured transaction errors |
| 004 | Add XLM payment builder |
| 005 | Add transaction inspection CLI (`stellarforge tx inspect <HASH>`) |
| 006 | Add payment memo support |

### Intermediate

| # | Issue |
|---|-------|
| 007 | Implement transaction submission pipeline |
| 008 | Implement confirmation tracking |
| 009 | Add retry middleware with configurable backoff |
| 010 | Implement SEP-41 payment support |
| 011 | Add Soroban contract invocation |
| 012 | Add Soroban simulation utilities |
| 013 | Add contract error decoding |
| 014 | Add event decoder (Soroban events → typed structures) |
| 015 | Add contract inspection CLI (`stellarforge contract inspect <ADDRESS>`) |

### Advanced

| # | Issue |
|---|-------|
| 016 | Implement Rust event ingestion engine |
| 017 | Add persistent event storage (PostgreSQL) |
| 018 | Implement transaction recovery for interrupted indexing jobs |
| 019 | Add webhook dispatcher |
| 020 | Add webhook retry and dead-letter handling |
| 021 | Add Soroban contract indexing |
| 022 | Build local development environment (Stellar/Soroban) |
| 023 | Add integration-test framework with reusable fixtures |
| 024 | Add fuzz testing for XDR/event parsing |
| 025 | Add TypeScript bindings |

---

## 23. Issue Design Philosophy

Issues should not be vague.

| ❌ Bad | ✅ Good |
|--------|---------|
| Improve transactions. | Implement transaction confirmation polling with configurable timeout and exponential backoff. The implementation must expose `confirmed`, `failed`, `timeout`, and `network-error` states and include unit tests covering each state. |

Every issue should answer:

1. What needs to be built?
2. Why is it needed?
3. Where should it live?
4. What behavior is expected?
5. How will it be tested?
6. What constitutes completion?

---

## 24. Drips Wave Strategy

StellarForge is designed to fit the Drips Wave contribution model. The repository should **not** be submitted immediately after creation.

**Before applying, the project should demonstrate:**

- Working Rust code and tests
- Clear architecture and documentation
- Contribution guidelines and real issues
- Meaningful commit history
- At least one functional end-to-end workflow
- Clear relevance to Stellar with opportunities for external contributors

If accepted, issues can be added to the Wave with complexity levels (Trivial / Medium / High) corresponding to base Point values of 100, 150, and 200.

---

## 25. Initial MVP

The first public release focuses on a working, well-tested core — not the full vision.

| Area | Scope |
|------|-------|
| **Rust Core** | Network config, address validation, common errors, RPC/Horizon config |
| **Transaction Engine** | Construction, submission, confirmation, basic retry |
| **Payments** | XLM payment, SEP-41 token payment |
| **Soroban** | Contract invocation, simulation, basic event decoding |
| **CLI** | Account, transaction, payment, and contract inspection |
| **Contracts** | One payment contract |
| **Quality** | Unit tests, integration tests, CI, documentation, contribution guide, security policy |

---

## 26. MVP Success Criteria

**A Stellar developer can complete this workflow:**

```
1. Install StellarForge
        ↓
2. Configure Stellar testnet
        ↓
3. Create / prepare a payment
        ↓
4. Submit transaction
        ↓
5. Wait for confirmation
        ↓
6. Decode the result
        ↓
7. Inspect transaction through CLI
```

**A Soroban developer can complete this workflow:**

```
1. Configure a contract
        ↓
2. Invoke a contract
        ↓
3. Simulate the invocation
        ↓
4. Submit the transaction
        ↓
5. Receive / decode events
```

---

## 27. Long-Term Roadmap

| Phase | Focus |
|-------|-------|
| **Phase 1 — Foundation** | Rust workspace, core crate, network config, error system, transaction primitives, CI, testing infrastructure |
| **Phase 2 — Transactions & Payments** | Transaction engine, XLM payments, token payments, confirmation tracking, retry system |
| **Phase 3 — Soroban** | Contract invocation, simulation, event decoding, contract utilities, reference contract |
| **Phase 4 — CLI** | Account, transaction, contract, payment, and event tools |
| **Phase 5 — Indexing** | Ledger ingestion, event indexing, PostgreSQL storage, query layer |
| **Phase 6 — Webhooks** | Event subscriptions, webhook delivery, retries, signatures, idempotency |
| **Phase 7 — Ecosystem SDK** | TypeScript bindings, documentation, examples, framework integrations |

---

## 28. Project Principles

| Principle | Description |
|-----------|-------------|
| **Rust first** | Core infrastructure implemented in Rust |
| **Open source first** | Important functionality remains available to the community |
| **Composable** | Individual components usable without adopting the entire stack |
| **Explicit over magical** | Stellar concepts remain understandable |
| **Secure by default** | Minimize key exposure; validate external input |
| **Contributor friendly** | Every major subsystem contains opportunities for external contributors |
| **Production minded** | Examples demonstrate realistic usage, not toy demos |
| **Ecosystem aligned** | Complement existing Stellar infrastructure; don't duplicate it |

---

## 29. What Makes StellarForge Different

Stellar already has excellent foundational infrastructure. StellarForge is not competing with those foundations — it builds on top of them.

```
Official Stellar infrastructure
          │
          ├── Stellar SDK
          ├── Soroban SDK
          ├── Stellar CLI
          ├── Horizon
          └── Stellar RPC
                    │
                    ▼
             ┌─────────────┐
             │ StellarForge│
             └──────┬──────┘
                    │
        ┌───────────┼────────────┐
        ▼           ▼            ▼
    Payments   Transactions   Events
        │           │            │
        └───────────┼────────────┘
                    ▼
             Applications
```

The value proposition is **integration**, **reusable infrastructure**, **developer experience**, and **contributor-friendly tooling**.

---

## 30. Definition of Done

A feature is not complete merely because it compiles. A complete contribution includes:

- [ ] Implementation
- [ ] Unit tests
- [ ] Integration tests (where applicable)
- [ ] Documentation
- [ ] Error handling
- [ ] Example usage (where appropriate)
- [ ] No unnecessary breaking API changes
- [ ] Formatted and Clippy-clean code
- [ ] CI passing

Security-sensitive components require additional review.

---

## 31. Final Project Statement

StellarForge is a Rust-first open-source infrastructure toolkit for Stellar and Soroban.

It provides reusable building blocks for transactions, payments, Soroban contracts, event processing, indexing, webhooks, CLI tooling, and developer testing.

The project exists to reduce duplicated infrastructure work across Stellar applications while creating a healthy open-source environment where developers can contribute meaningful Rust and Soroban code.

The immediate objective is not to build everything. The immediate objective is to build a small, high-quality Rust core, establish strong contributor practices, and progressively expand the infrastructure around it.

---

> **Build once. Reuse everywhere. Build it in Rust.**

---

### Strategic Note

Don't put the entire roadmap into the initial repository release. The `project.md` can describe the full vision, but the actual first release should be much smaller.

The strongest initial proof is: **Rust core → transaction engine → payment → Soroban call → event decoding → CLI → tests.** That gives something tangible before approaching Drips, and aligns with what Drips wants: clearly scoped issues that outside contributors can realistically complete during a Wave.

Because the official Stellar repositories already have mature Rust infrastructure (e.g. `rs-soroban-sdk`, `stellar-cli`), the `README.md` must explicitly explain why StellarForge exists alongside them — not as a competing replacement.

**Recommended next document: `ARCHITECTURE.md`** — defining actual Rust crates, dependencies, traits, database schema, transaction flow, event model, and MVP API before any code is written.
