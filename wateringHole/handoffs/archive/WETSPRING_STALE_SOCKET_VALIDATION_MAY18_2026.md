# wetSpring: Stale Socket Pattern — Science Pipeline Validation

**Date:** May 18, 2026
**From:** primalSpring (coordination spring)
**To:** wetSpring team (southGate)
**Priority:** MEDIUM — pattern ready for ingestion into science pipelines
**License:** AGPL-3.0-or-later

---

## Context

Your May 18 stale socket report triggered an ecosystem-wide response. The
pattern is now resolved at all layers and ready for validation in science
pipelines. This blurb summarizes what shipped and what wetSpring should ingest.

## What Shipped (Ecosystem-Wide)

### Consumer Side (primalSpring — your observation led to this)

- **`socket_is_alive(path)`** — connect-probe with 50ms timeout replaces
  `path.exists()` across all 6 discovery tiers (`discover_primal`,
  `discover_by_capability`, `NeuralBridge::discover`, manifest, socket
  registry, capability-named sockets)
- **`DEAD_SOCKET_CACHE`** — process-level negative cache (`OnceLock<Mutex<HashSet<PathBuf>>>`)
  prevents repeated ~100ms probe costs for known-dead sockets within a session
- **Standards**: `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.3.0 §5-6

### Server Side (all 14 primals — confirmed clean)

Every primal now has `unlink()` before `bind()` at all UDS bind sites. Notable:

- **toadStool S264**: 6/6 sites audited, CLI daemon + DisplayServer gaps fixed
- **barraCuda**: 2 `transport.rs` bind sites fixed post-sweep
- **biomeOS + songbird**: socket hygiene confirmed in CHANGELOGs

### Infrastructure (plasmidBin — we own this)

- **`doctor.sh`**: stale socket detection section (fuser + python3 fallback)
- **`stop_gate.sh`**: post-kill socket cleanup for `biomeos/`, `ecoprimals/`, `/tmp/biomeos/`
- **`start_primal.sh`**: pre-start stale socket removal at `--socket` path

---

## What wetSpring Should Ingest

### 1. Connect-Probe Pattern for Trio Discovery

Your pipelines that discover biomeOS/songbird/toadStool via socket files
should adopt the connect-probe pattern instead of file-exists:

```python
import socket
import os

def socket_is_alive(path: str, timeout: float = 0.05) -> bool:
    """Check if a Unix domain socket has an active listener."""
    if not os.path.exists(path):
        return False
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(timeout)
        s.connect(path)
        s.close()
        return True
    except (ConnectionRefusedError, FileNotFoundError, OSError):
        return False
```

This eliminates the ~100ms timeout you observed per stale socket. The 50ms
connect timeout is a ceiling — live sockets respond in <1ms, dead sockets
fail with ECONNREFUSED immediately.

### 2. Session-Level Negative Cache

Once `Trio available: false` is determined for a session, cache it:

```python
_dead_sockets: set[str] = set()

def discover_trio(socket_dir: str) -> dict | None:
    """Discover trio with negative caching."""
    for sock in Path(socket_dir).glob("*.sock"):
        path = str(sock)
        if path in _dead_sockets:
            continue
        if socket_is_alive(path):
            return connect_trio(path)
        _dead_sockets.add(path)
    return None
```

This is the Python equivalent of primalSpring's `DEAD_SOCKET_CACHE`. For your
264-clone Tenaillon pipeline, this eliminates 21 failed connections (3 trio
calls × 7 clones) → 0 after the first discovery attempt.

### 3. Degradation Path Validation

Your existing degradation path (`Trio available: false` → local IDs) is correct.
The new pattern just makes it faster:

| Scenario | Before | After |
|----------|--------|-------|
| Trio running | ~1ms discovery | ~1ms discovery |
| Trio crashed (stale sockets) | ~100ms × N attempts | ~1ms (ECONNREFUSED) |
| Trio not deployed | ~100ms × N attempts | ~1ms (file not found) |
| Trio crashed, 2nd+ attempt | ~100ms | 0ms (negative cache) |

### 4. Science Pipeline Integration Points

These are the specific places in wetSpring where the pattern should land:

- **breseq pipeline** (Barrick 2009, Tenaillon 2016): trio capability calls
  for provenance, attestation, and braid handoff
- **Clone-level parallelism**: the `compute.fan_out` dispatch path that
  calls trio per-clone — negative cache is critical here
- **E2E study harness**: the `Trio available` check at pipeline startup

### 5. Validation Scenario

We recommend adding a validation scenario (or extending an existing one)
that exercises the degradation path:

```
Scenario: stale_socket_degradation
  Given: stale trio sockets exist (no listener)
  When: pipeline discovers trio via socket_is_alive()
  Then: discovery returns None within 5ms (not 100ms+)
  And: negative cache prevents re-probe on subsequent calls
  And: pipeline falls back to local IDs without error
```

---

## Escalation Items (for your records)

| Item | Owner | Status |
|------|-------|--------|
| skunkBat `seed_fingerprint` missing from plasmidBin manifest | plasmidBin CI (harvest pipeline) | GAP — should auto-populate on next skunkBat push |
| sourDough `Cargo.toml` version 0.1.0 vs docs 0.3.0 | sourDough team | Their debt — version bump missed |
| R11 PID files alongside sockets | All primals | DEPRIORITIZED — connect-probe provides equivalent liveness |

---

## References

- `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.3.0 §5-6
- `DEPLOYMENT_VALIDATION_STANDARD.md` (stale socket hygiene section)
- `STALE_SOCKET_CLEANUP_UPSTREAM_MAY18_2026.md` (full ecosystem blurb)
- primalSpring `ecoPrimal/src/ipc/discover.rs` — `socket_is_alive()`, `DEAD_SOCKET_CACHE`
- plasmidBin `doctor.sh` — stale socket detection section

Thank you for the production observation. Your report triggered a full ecosystem
sweep and all 14 primals are now confirmed clean.
