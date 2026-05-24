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
| bearDog | PASS | PASS `"alive"` | PASS | PASS | PASS | PASS |
| songbird | PASS | PASS `"alive"` | PASS | PASS | PASS | PASS |
| toadStool | PASS | PASS `"alive"` (S272) | PASS | PASS | PASS | PASS |
| biomeOS | PASS | PASS `"alive"` (v3.72) | PASS | N/A | PASS | PASS |
| nestgate | PASS (S71) | PASS `"alive"` (S71) | PASS | PASS | PASS | PASS |
| squirrel | PASS | PASS | PASS | PASS | PASS | PASS |
| barraCuda | PASS alias (W47) | PASS `"alive"` | PASS | PASS | PASS | PASS |
| petalTongue | PASS | PASS `"alive"` | PASS | PASS | PASS | PASS (W47) |
| rhizoCrypt | PASS alias (W47) | PASS `"alive"` | PASS | PASS | PASS | PASS |
| loamSpine | PASS | PASS `"alive"` | PASS (+uptime_s) | PASS | PASS | PASS |
| sweetGrass | PASS | PASS `"alive"` | PASS | PASS | PASS | PASS |
| coralReef | PASS (W47) | PASS `"alive"` (W47) | PASS | PASS | PASS | PASS |
| skunkBat | PASS (W47) | PASS `"alive"` | PASS (W47) | PASS | PASS | PASS (W47) |

### Summary (May 24, 2026)

- **13/13 PASS** across all dimensions
- All behavioral convergence items from the initial audit have been resolved
- Standard published → 9 primals responded and fixed within hours

---

## 6. Upstream Asks — ALL RESOLVED (May 24, 2026)

All 13 asks across 9 primals were resolved within hours of the standard being
published. The ecosystem achieved **13/13 behavioral convergence** in a single wave.

| Primal | Items | Resolution |
|--------|------:|------------|
| nestgate | 2 | S71: `--socket` CLI flag + `health.liveness` → `"alive"` on all 5 transports |
| skunkBat | 4 | W47: `--socket`, `lifecycle.status`, SIGTERM, port 9750 |
| toadStool | 1 | S272: `health.liveness` always `"alive"` (boot → `health.readiness`) |
| barraCuda | 1 | W47: `--socket` aliased to `--unix` |
| rhizoCrypt | 1 | W47: `--socket` aliased to `--unix` |
| coralReef | 2 | W47: `--socket` CLI + `health.liveness` → `"alive"` |
| biomeOS | 1 | v3.72: `health.check` → `"alive"` (was `"healthy"`) |
| petalTongue | 1 | W47: `src/signal.rs` SIGTERM + SIGINT handler |
| loamSpine | 0 | "Tokio crash" was CLI mismatch (`serve`→`server`), already fixed in plasmidBin |
