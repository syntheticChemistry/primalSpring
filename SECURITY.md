# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.9.25+ | Yes       |
| 0.9.x   | Yes       |
| 0.8.x   | Archived  |

## Reporting a Vulnerability

primalSpring is AGPL-3.0-or-later open source. If you discover a security
vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainer directly with a description of the vulnerability
3. Include steps to reproduce if possible
4. Allow reasonable time for a fix before public disclosure

## Security Posture

- **Zero unsafe code**: `#![forbid(unsafe_code)]` on all 88 crate roots
  (lib + 3 bins + 93 experiments + 1 helper lib). `SeedConfig` + `OnceLock`
  replaced `env::set_var` (Rust 2024 marks it `unsafe`) — no `unsafe` blocks
  remain in any binary
- **Zero C dependencies**: enforced by `deny.toml` (ecoBin compliant)
- **No network listeners by default**: the JSON-RPC server binds to Unix
  domain sockets by default; TCP is available for cross-gate and mobile
  deployments but requires explicit configuration
- **Capability-based discovery**: no hardcoded addresses or credentials
- **No secrets in source**: API keys are passed via environment variables
  or `testing-secrets/` (gitignored)
- **BTSP default on all tiers**: guidestone expects BTSP authentication on
  every capability (Tower, Node, Nest, Provenance). **13/13 capabilities
  BTSP-authenticated** — full NUCLEUS convergence achieved. Cleartext
  connections are reported as FAIL in guidestone. Next phase: granular
  per-deployment policy control based on trust model and lineage
- **Seed provenance**: public BLAKE3 fingerprints for each primal published
  in `plasmidBin/manifest.toml` — Layer 0.5 verifies binary authenticity
  before any IPC
- **Entropy hierarchy**: machine-level mito tier (portable/clonable) for CI
  and automated validation; nuclear tier (future) for sovereign identity
- **Method gate (JH-0)**: pre-dispatch capability authorization on JSON-RPC
  dispatchers. Public methods (health, identity, capabilities) are exempt;
  all other methods require a capability token when enforcement is active.
  Default: `Enforced` with deny-by-default (`DenyVerifier`) when security
  provider is unreachable. `SecurityVerifier` discovered by capability at runtime

## Dependency Auditing

Dependencies are audited via `cargo deny check` which enforces:
- License allowlist (AGPL-compatible only)
- Advisory database checks
- C dependency ban list (20 crates banned for ecoBin compliance)
