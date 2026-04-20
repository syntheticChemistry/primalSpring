# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.9.17+ | Yes       |
| 0.9.x   | Yes       |
| 0.8.x   | Maintenance |

## Reporting a Vulnerability

primalSpring is AGPL-3.0-or-later open source. If you discover a security
vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainer directly with a description of the vulnerability
3. Include steps to reproduce if possible
4. Allow reasonable time for a fix before public disclosure

## Security Posture

- **Zero unsafe code**: `#![forbid(unsafe_code)]` at workspace level
- **Zero C dependencies**: enforced by `deny.toml` (ecoBin compliant)
- **No network listeners by default**: the JSON-RPC server binds to Unix
  domain sockets by default; TCP is available for cross-gate and mobile
  deployments but requires explicit configuration
- **Capability-based discovery**: no hardcoded addresses or credentials
- **No secrets in source**: API keys are passed via environment variables
  or `testing-secrets/` (gitignored)

## Dependency Auditing

Dependencies are audited via `cargo deny check` which enforces:
- License allowlist (AGPL-compatible only)
- Advisory database checks
- C dependency ban list (14 crates banned for ecoBin compliance)
