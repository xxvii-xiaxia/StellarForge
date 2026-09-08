# Security Policy

## Reporting a vulnerability

If you discover a security vulnerability in StellarForge, please report it privately rather
than opening a public issue. Use GitHub's
[private vulnerability reporting](https://github.com/xxvii-xiaxia/StellarForge/security/advisories/new)
for this repository.

Please include:

- A description of the vulnerability and its potential impact
- Steps to reproduce, or a minimal proof of concept
- The affected crate(s) and version(s)

We aim to acknowledge reports within 5 business days.

## Scope

Security-relevant areas of this project include:

- Private key handling (StellarForge should never log or persist private keys — see
  [`project.md` §18](project.md#18-security))
- Transaction construction, signing, and submission
- XDR / Soroban event and contract data parsing (untrusted network input)
- Webhook signature verification and delivery

## Supported versions

StellarForge is pre-1.0. Security fixes are applied to the latest published release only.

## Practices

- Dependencies are audited with [`cargo audit`](https://github.com/rustsec/rustsec) in CI.
- Parsers for untrusted input (XDR, Soroban events) are expected to be fuzz-tested before they
  are considered production-ready — see [`project.md` §19](project.md#19-testing-strategy).
- Signing is designed so applications supply their own signing mechanism; StellarForge avoids
  handling private keys directly wherever possible.
