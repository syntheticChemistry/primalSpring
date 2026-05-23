# Wave 42 — Neural API Deployment Guide

> **Audience**: All 13 primal teams
> **Date**: 2026-05-22 (Wave 42)
> **Status**: **SUPERSEDED** (May 23, 2026) — All 12/12 announcing primals compliant. Resolved in Wave 45. Retained as reference guide.

## Why

biomeOS v3.68+ has a full adaptive routing layer: the Neural API scores
providers by latency, error rate, affinity, and cost — then routes
`capability.call` to the best candidate. **But this only works when primals
announce themselves with the v3.68 schema.** Without correct announcements,
the routing layer has weights but no training data.

Current adoption is uneven:

| Primal | `primal.announce` status |
|--------|--------------------------|
| biomeOS | Full handler (registration authority) — not a self-announcer |
| songbird | Handler exists but **schema mismatch**: uses `provided_capabilities` instead of `capabilities`, missing `socket` field, no `cost_hints`/`latency_estimates` |
| toadStool | Stub handler in `identity.rs` marked `dead_code`, not wired |
| bearDog | No JSON-RPC handler; mDNS-style stubs only |
| nestgate | Internal `announce_self()` with discovery stubs, no JSON-RPC handler |
| Other 9 primals | Unknown — likely stubs or nothing |

## Wire Schema

Send this JSON-RPC request to biomeOS's neural-api socket on startup:

```json
{
  "jsonrpc": "2.0",
  "method": "primal.announce",
  "params": {
    "primal": "beardog",
    "socket": "/run/user/1000/biomeos/beardog-ecoPrimal.sock",
    "pid": 12345,
    "capabilities": ["crypto", "security"],
    "methods": [
      "crypto.encrypt",
      "crypto.decrypt",
      "crypto.hash",
      "crypto.sign",
      "crypto.verify",
      "security.attest",
      "security.audit"
    ],
    "semantic_mappings": {
      "crypto.sha256": "crypto.hash"
    },
    "signal_tiers": ["tower"],
    "cost_hints": {
      "crypto": 5.0,
      "security": 10.0
    },
    "latency_estimates": {
      "crypto": 2,
      "security": 15
    },
    "version": "2.14.0",
    "attestation": null
  },
  "id": 1
}
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `primal` | string | **yes** | Your primal name (lowercase, e.g. `beardog`) |
| `socket` | string | **yes** | Full path to your Unix socket or `host:port` for TCP |
| `pid` | u32 | no | Process ID for lifecycle tracking |
| `capabilities` | string[] | **yes** | Capability domains you serve (e.g. `["crypto", "security"]`) |
| `methods` | string[] | recommended | Individual methods you expose (e.g. `["crypto.hash"]`) |
| `semantic_mappings` | object | no | Consumer name → actual RPC method (for translation registry) |
| `signal_tiers` | string[] | recommended | Which atomic tiers you participate in: `tower`, `node`, `nest`, `meta` |
| `cost_hints` | object | recommended | Per-capability cost (arbitrary units, lower = cheaper). Seeds routing weight cost factor |
| `latency_estimates` | object | recommended | Per-capability expected latency (ms). Seeds routing weight latency factor |
| `version` | string | no | Your primal version string |
| `attestation` | string | no | Signed attestation (verified via bearDog when present) |

### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "primal": "beardog",
    "capabilities_registered": 2,
    "methods_registered": 7,
    "signal_tiers_joined": ["tower"],
    "attestation_verified": false
  },
  "id": 1
}
```

## Signal Tier Membership

Each primal belongs to one or more composition tiers. Your `signal_tiers`
array determines which signal graphs can route through you:

| Tier | Primals | Description |
|------|---------|-------------|
| **tower** | bearDog, songbird, skunkBat | Security + relay backbone |
| **node** | toadStool, barraCuda, coralReef | Compute + data processing |
| **nest** | nestgate, rhizoCrypt, loamSpine, sweetGrass | Storage + persistence + local data |
| **meta** | petalTongue, squirrel | Content + UI + user-facing |

## Cost Hints

Cost hints let the routing layer prefer cheaper providers when quality
is equal. Values are arbitrary but should be consistent within a domain:

```json
{
  "cost_hints": {
    "compute": 100.0,
    "storage": 10.0,
    "crypto": 5.0
  }
}
```

The routing score formula:

```
score = affinity × (1 - error_rate) / (1 + latency/100) - cost/1000
```

A provider with `cost_hint: 5.0` gets a -0.005 penalty; `cost_hint: 100.0`
gets -0.1. Cost matters most when latency and reliability are similar.

## Latency Estimates

Self-reported latency (ms) seeds the EWMA before real observations
accumulate. After ~5 dispatches, operational data dominates.

```json
{
  "latency_estimates": {
    "crypto": 2,
    "compute": 200,
    "storage": 50
  }
}
```

Primals that self-report latency get an affinity boost from 0.5 to 0.6
(cooperating primals are preferred).

## When to Announce

1. **On startup** — after your socket is listening, send `primal.announce`
   to biomeOS's neural-api socket
2. **On capability change** — if you add/remove capabilities at runtime,
   re-announce with the updated set
3. **On version upgrade** — re-announce with the new version string

### Socket Discovery

Find biomeOS's neural-api socket using standard tiered lookup:

1. `$NEURAL_API_SOCKET` env override
2. `$XDG_RUNTIME_DIR/biomeos/neural-api-{family}.sock`
3. `/tmp/biomeos/neural-api-{family}.sock`

Where `{family}` is from `$ECOPRIMALS_FAMILY_ID` (default: `ecoPrimal`).

## Per-Primal Implementation Notes

### bearDog (Tower — crypto, security)
- Add JSON-RPC handler for `primal.announce` as a self-announcement on startup
- Capabilities: `["crypto", "security"]`
- Methods: all `crypto.*` and `security.*` from capability registry
- Signal tier: `["tower"]`
- Low cost hints — crypto is fast and cheap

### songbird (Tower — relay, communication)
- **Fix schema mismatch**: rename `provided_capabilities` → `capabilities`, add `socket` field
- Add `cost_hints` and `latency_estimates`
- Capabilities: `["relay", "communication", "presence"]`
- Signal tier: `["tower"]`

### toadStool (Node — compute, science)
- Wire the existing `primal_announce` stub in `identity.rs` into dispatch
- Capabilities: `["compute", "science", "inference"]`
- Signal tier: `["node"]`
- Higher cost hints — compute is expensive

### barraCuda (Node — data, pipeline)
- Add `primal.announce` handler
- Capabilities: `["data", "pipeline", "transform"]`
- Signal tier: `["node"]`

### coralReef (Node — protocol, network)
- Add `primal.announce` handler
- Capabilities: `["protocol", "network"]`
- Signal tier: `["node"]`

### nestgate (Nest — storage, dag)
- Wire existing `announce_self()` to emit proper JSON-RPC `primal.announce`
- Capabilities: `["storage", "dag", "replication"]`
- Signal tier: `["nest"]`

### rhizoCrypt (Nest — dag, integrity)
- Add `primal.announce` handler
- Capabilities: `["dag", "integrity", "merkle"]`
- Signal tier: `["nest"]`

### loamSpine (Nest — local data, telemetry)
- Add `primal.announce` handler
- Capabilities: `["telemetry", "local_data", "metrics"]`
- Signal tier: `["nest"]`

### sweetGrass (Nest — local config, environment)
- Add `primal.announce` handler
- Capabilities: `["config", "environment"]`
- Signal tier: `["nest"]`

### skunkBat (Tower — security, stealth)
- Add `primal.announce` handler
- Capabilities: `["stealth", "obfuscation"]`
- Signal tier: `["tower"]`

### petalTongue (Meta — content, language)
- Add `primal.announce` handler
- Capabilities: `["content", "language", "translation"]`
- Signal tier: `["meta"]`

### squirrel (Meta — UI, interaction)
- Add `primal.announce` handler
- Capabilities: `["ui", "interaction"]`
- Signal tier: `["meta"]`

## New RPC Endpoints (Wave 42)

The Neural API now exposes utilization tracking:

| Method | Description |
|--------|-------------|
| `neural_api.routing_weights` | Full routing weight table snapshot |
| `neural_api.route_explain` | Explain routing decision for a method |
| `neural_api.composition_patterns` | All registered composition patterns |
| `neural_api.plan_tier` | Deployment blueprint for a composition tier |
| `neural_api.utilization` | **NEW** Hot/cold method utilization tracking |

## Validation

After implementing `primal.announce`, verify by:

1. Start biomeOS neural-api and your primal
2. Your primal sends `primal.announce` on startup
3. Query `neural_api.routing_weights` — you should see entries for your capabilities
4. Call `capability.call` through your domain — weights should update
5. Query `neural_api.utilization` — your methods should appear in hot/cold tracking

## Weight Persistence (Wave 42)

Routing weights now survive biomeOS restarts via redb persistence. This
means:
- First launch: all providers start with exploration bonus (score +0.1)
- Subsequent launches: weights are loaded from disk, routing starts warm
- Circuit breakers are preserved — a provider with 5+ consecutive failures
  stays open until cooldown expires

The weights database lives at `$XDG_DATA_HOME/biomeos/routing_weights.redb`
(or `$HOME/.local/share/biomeos/routing_weights.redb`).
