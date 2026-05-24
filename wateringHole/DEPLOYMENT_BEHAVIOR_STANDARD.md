# Deployment Behavior Standard — Post-Primordial Convergence

**Version**: 1.0.0 (Wave 47)
**Date**: May 24, 2026
**Audience**: All primal teams
**Status**: ACTIVE — defines expected deployment surface for NUCLEUS compositions

---

## Motivation

Post-primordial, the ecosystem deploys primals through `plasmidBin`'s
`nucleus_launcher.sh` and `start_primal.sh`. Every CLI, health endpoint,
and socket behavior difference requires a per-primal workaround in the
launcher. These workarounds are fragile and break when primals evolve
independently. This standard defines the **minimum behavioral contract**
that every primal must satisfy for clean NUCLEUS deployment.

---

## 1. CLI Surface

### Server Subcommand

Every primal binary MUST accept a `server` subcommand that starts the
JSON-RPC IPC listener. This is the only subcommand `start_primal.sh` invokes.

### Required Flags

| Flag | Behavior | Type |
|------|----------|------|
| `--socket PATH` | Bind UDS at this path | `PathBuf` |
| `--port PORT` | Bind TCP on this port (opt-in; UDS is default) | `u16` |
| `--family-id ID` | Set family identifier for BTSP | `String` |

`--socket` MUST accept an absolute filesystem path and bind a Unix domain
socket there. If the primal uses a different internal flag name (e.g.
`--unix`), it MUST alias `--socket` to it.

### Recommended Flags

| Flag | Behavior |
|------|----------|
| `--bind ADDR` | TCP bind address (default `127.0.0.1`) |
| `--abstract` | Use abstract socket namespace |
| `--pid-file PATH` | Write PID file |

---

## 2. Health Endpoints

### Required Methods

| Method | Response | Transport | Gate |
|--------|----------|-----------|------|
| `health.liveness` | `{"status":"alive"}` | All active | Public (BTSP-exempt) |
| `lifecycle.status` | `{"primal":"<name>","version":"<ver>","status":"running","uptime_s":<n>}` | All active | Public (BTSP-exempt) |

`health.liveness` is the **probe target** for `nucleus_launcher.sh` health
sweeps. It MUST return within 5 seconds and MUST NOT require authentication.

The response shape is `{"status":"alive"}` — the L2 wire standard. Primals
MUST NOT return `{"alive":true}` without a `"status"` field, as health
probes check `jq -r .status`.

### Recommended Methods

| Method | Purpose |
|--------|---------|
| `health.version` | Return `{"version":"<semver>","primal":"<name>"}` |
| `health.metrics` | Operational metrics for observatory |

---

## 3. Socket Behavior

### Path Convention

Default UDS path: `$XDG_RUNTIME_DIR/biomeos/<primal>[-<family_id>].sock`

Fallback when `XDG_RUNTIME_DIR` is unset: `/tmp/biomeos/<primal>[-<family_id>].sock`

The `--socket` flag overrides this default completely.

### Cleanup

On startup: Remove stale socket at bind path if no process holds it (connect-probe or PID check).

On shutdown (SIGTERM/SIGINT): Remove socket file, remove PID sidecar if present.

### PID Sidecar (Optional)

If writing a PID file, place it at `<socket_path>.pid` alongside the socket.

---

## 4. Startup Behavior

### primal.announce

Every primal (except biomeOS, which receives announcements) MUST send a
`primal.announce` JSON-RPC call to biomeOS on startup, after binding its
socket. This is best-effort — if biomeOS is unreachable, the primal starts
normally.

### Signal Handling

The binary MUST handle both `SIGTERM` and `SIGINT` gracefully:
1. Stop accepting new connections
2. Drain in-flight requests (with timeout)
3. Remove socket file
4. Exit cleanly

---

## 5. Current Compliance Matrix

| Primal | `--socket` | `health.liveness` shape | `lifecycle.status` | `primal.announce` | Cleanup | Signals |
|--------|:----------:|:-----------------------:|:------------------:|:-----------------:|:-------:|:-------:|
| bearDog | PASS | `"alive"` PASS | PASS | PASS | PASS | PASS |
| songbird | PASS | `"alive"` PASS | PASS | PASS | PASS | PASS |
| toadStool | PASS | `"alive"` PASS | PASS | PASS | PASS | PASS |
| biomeOS | PASS | mixed | PASS | N/A | PASS | PASS |
| nestgate | **FAIL** env-only | **FAIL** `{alive:true}` | PASS | PASS | PASS | PASS |
| squirrel | PASS | PASS | PASS | PASS | PASS | PASS |
| barraCuda | **FAIL** `--unix` | `"alive"` PASS | PASS | PASS | PASS | PASS |
| petalTongue | PASS | mixed | PASS | PASS | PASS | partial |
| rhizoCrypt | **FAIL** `--unix` | `"alive"` PASS | PASS | PASS | PASS | PASS |
| loamSpine | PASS | `"alive"` PASS | PASS | PASS | PASS | PASS |
| sweetGrass | PASS | `"alive"` PASS | PASS | PASS | PASS | PASS |
| coralReef | **FAIL** no CLI | **FAIL** `{alive:true}` | PASS | PASS | PASS | PASS |
| skunkBat | **FAIL** no CLI | `"alive"` PASS | **FAIL** missing | PASS | PASS | **FAIL** SIGINT only |

### Summary

- **`--socket` flag**: 8/13 PASS, 5 need aliases or additions
- **`health.liveness` shape**: 10/13 PASS, 3 return non-standard shape
- **`lifecycle.status`**: 12/13 PASS, skunkBat missing
- **`primal.announce`**: 12/13 (biomeOS exempt)
- **Graceful shutdown**: 12/13, skunkBat SIGINT-only

---

## 6. Upstream Asks (Priority Order)

### Critical (blocks NUCLEUS health sweeps)

| Primal | Ask | Effort |
|--------|-----|--------|
| **loamSpine** | Fix Tokio double-runtime crash on NUCLEUS start | HIGH — blocks southGate entirely |

### High (blocks uniform deployment)

| Primal | Ask | Effort |
|--------|-----|--------|
| **nestgate** | Add `--socket PATH` CLI flag (alias to `NESTGATE_SOCKET` env) | LOW — clap field + env bridge |
| **nestgate** | Normalize `health.liveness` → `{"status":"alive"}` | LOW — response shape change |
| **barraCuda** | Alias `--socket` → `--unix` in clap | LOW — clap alias |
| **rhizoCrypt** | Alias `--socket` → `--unix` in clap | LOW — clap alias |
| **coralReef** | Add `--socket PATH` flag (set UDS bind path) | LOW — clap field |
| **coralReef** | Normalize `health.liveness` → `{"status":"alive"}` | LOW — response shape |
| **skunkBat** | Add `--socket PATH` flag | LOW — clap field |
| **skunkBat** | Add `lifecycle.status` method | LOW — handler + route |
| **skunkBat** | Handle SIGTERM (not just SIGINT) | LOW — `tokio::signal::unix` |
| **skunkBat** | Align default port to 9750 (matches `ports.env`) | LOW — constant change |

### Medium (improves observability)

| Primal | Ask | Effort |
|--------|-----|--------|
| **biomeOS** | Normalize `health.liveness` on api socket → `{"status":"alive"}` | LOW |
| **petalTongue** | Add explicit SIGTERM handler | LOW |
| **toadStool** | Add `health.liveness` instant response (even during boot) | LOW |
