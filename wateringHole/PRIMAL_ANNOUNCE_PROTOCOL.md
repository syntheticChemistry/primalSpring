# Primal Announce Protocol

**Status**: Active  
**Version**: 1.0  
**Date**: 2026-05-15

## Overview

`primal.announce` is a single JSON-RPC method that atomically registers a primal
across biomeOS's lifecycle manager, capability router, and translation registry.
It replaces the prior pattern of sending separate `lifecycle.register`,
`capability.register`, and `method.register` calls.

## Wire Format

```json
{
  "method": "primal.announce",
  "params": {
    "primal": "beardog",
    "socket": "/run/user/1000/biomeos/beardog-default.sock",
    "pid": 12345,
    "capabilities": ["crypto", "security"],
    "methods": ["crypto.encrypt", "crypto.hash", "security.verify"],
    "semantic_mappings": {
      "crypto.sha256": "crypto.hash"
    },
    "signal_tiers": ["tower"],
    "attestation": "<hex-encoded-signed-attestation>",
    "version": "0.4.2"
  }
}
```

## Field Reference

| Field | Required | Description |
|-------|----------|-------------|
| `primal` | Yes | Primal name (must match socket naming convention) |
| `socket` | Yes | Transport endpoint (Unix socket path or `tcp://host:port`) |
| `pid` | No | Process ID for lifecycle tracking |
| `capabilities` | No | Coarse capability domains (registered on NeuralRouter) |
| `methods` | No | Individual `domain.operation` methods (registered as translations) |
| `semantic_mappings` | No | Consumer-facing name to actual RPC method aliases |
| `signal_tiers` | No | Signal tiers this primal participates in (`tower`/`node`/`nest`/`meta`) |
| `attestation` | No | Hex-encoded signed attestation (verified via BearDog when present) |
| `version` | No | Primal version string |

## Registration Order

1. **Lifecycle**: If `pid` is provided, registers with `LifecycleManager` for health tracking
2. **Capability**: Each entry in `capabilities` is registered on `NeuralRouter`
3. **Methods + Translations**: Each method in `methods` creates a translation entry; domains not in `capabilities` are auto-registered. `semantic_mappings` adds consumer-facing aliases.
4. **Signal Tiers**: Valid tiers are recorded for PathwayLearner graph extension

## Field Normalization

Primals MUST use `primal` and `socket` field names (not `primal_id`/`socket_path`).
This resolves the SongBird field naming mismatch identified in the Neural API audit.

## Backward Compatibility

Existing primals using separate `capability.register` + `lifecycle.register` calls
continue to work. `primal.announce` is additive — it does not deprecate the
individual registration methods.

## Signal Tier Membership

When a primal declares `signal_tiers: ["tower"]`, it indicates that signal graphs
for that tier (e.g., `tower_publish.toml`) may include this primal as a participant.
PathwayLearner uses this membership to suggest graph extensions when new primals
join a tier.

## Attestation

When `attestation` is provided, biomeOS records it for downstream verification.
Full verification requires BearDog (`crypto.verify`) and is performed lazily
on first capability.call through the announced primal.
