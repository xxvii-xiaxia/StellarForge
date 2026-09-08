# Contributing to StellarForge

Thanks for your interest in contributing. StellarForge is designed specifically to be
approachable for external contributors — see [`project.md`](project.md) for the full
architecture and roadmap.

## Getting set up

```sh
git clone git@github.com:xxvii-xiaxia/StellarForge.git
cd StellarForge
cargo build
cargo test
```

You'll need a recent stable Rust toolchain (`rustup update stable`).

## Workflow

1. Find or open an issue. Issues are scoped to be independently completable — see
   [`project.md` §23](project.md#23-issue-design-philosophy) for what a well-written issue
   looks like.
2. Comment on the issue before starting significant work, so effort isn't duplicated.
3. Create a branch, make your change, and add tests. See the
   [Definition of Done](project.md#30-definition-of-done).
4. Run the full check suite locally before opening a PR:

   ```sh
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

5. Open a pull request using the PR template. Link the issue it closes.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/) style prefixes
(`feat:`, `fix:`, `docs:`, `test:`, `ci:`, `chore:`, `refactor:`), scoped to the crate where
useful, e.g. `feat(core): add address validation`.

## Code style

- Format with `cargo fmt` and keep Clippy warning-free.
- Prefer explicit, well-named types over generic/boxed abstractions — see
  [Project Principles](project.md#28-project-principles) ("explicit over magical").
- Never log or persist private keys. See [`SECURITY.md`](SECURITY.md).

## Issue labels

`good first issue` · `help wanted` · `rust` · `soroban` · `transactions` · `payments` ·
`indexer` · `events` · `cli` · `documentation` · `testing` · `security`

## Code of Conduct

This project follows the [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to
uphold it.
