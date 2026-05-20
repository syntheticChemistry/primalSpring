# Wave 24: Shadow Run Execution — Sovereignty Parity Proofs

**Date:** May 19, 2026
**From:** primalSpring (coordination spring)
**To:** projectNUCLEUS, bearDog team, songbird team, nestGate team, petalTongue team
**Priority:** HIGH — stadial requires shadow parity proof before cutover
**Blockers:** None critical — all components shipped, shadow runs can begin
**License:** AGPL-3.0-or-later

---

## Goal

Prove that sovereign components can replace their commercial counterparts
without regression. Each shadow run measures latency, reliability, and feature
parity. Cutover happens only after shadow parity is demonstrated.

The interstadial exit was structural — Waves 1-22 proved the ecosystem works.
Wave 24 proves it works **in production** against real baselines.

---

## Shadow Matrix

| # | Shadow | Sovereign | Commercial | Owner | Status | Metric |
|---|--------|-----------|------------|-------|--------|--------|
| S1 | TLS termination | bearDog :8443 (rustls) | Cloudflare TLS | bearDog | **LIVE** — H2-12 | 10ms vs 120ms tunnel |
| S2 | NAT traversal | songbird relay (TURN) | cloudflared tunnel | songbird | **READY** — TURN client shipped (Wave 205) | Latency + reliability |
| S3 | Content hosting | nestGate + petalTongue | GitHub Pages | nestGate + petalTongue | **READY** — transport parity (Session 60) | TTFB + cache hit rate |
| S4 | Auth / JupyterHub | bearDog BTSP + dual-auth | OAuth2 proxy | bearDog | **READY** — H2-2b | Auth latency + session mgmt |

---

## S1: BearDog TLS Shadow (LIVE)

### Current State

bearDog TLS termination via rustls is live in shadow mode (H2-12). Measured
latency: **10ms** sovereign vs **120ms** Cloudflare tunnel (12× improvement).

### What's Done

- rustls X.509 termination with per-IP sliding-window rate limiter (Wave 100)
- `deny.toml` ring policy reconciled — ring allowed only via rustls wrapper (Wave 105)
- ACME TLS integration path designed (`specs/ACME_TLS_INTEGRATION_PATH.md`)
- FIDO2/CTAP2 IPC surface for hardware-attested auth (Wave 103)

### Remaining for Cutover

- [ ] **ACME Phase 2**: implement `beardog-acme` crate — HTTP-01 challenge handler,
  certificate storage in `$BEARDOG_DATA_DIR/acme/`, hot-reload via `Arc<ServerConfig>` swap
- [ ] **ACME Phase 3**: renewal daemon — 12h check, 30-day-before-expiry renewal
- [ ] **Shadow metric collection**: 7-day side-by-side — requests/sec, p50/p95/p99
  latency, error rate, certificate rotation success
- [ ] **Cutover criteria**: sovereign p95 ≤ 1.5× commercial p95 for 7 consecutive days

### Composition

```
Request → bearDog :8443 (rustls TLS termination)
       → biomeOS (NeuralAPI routing)
       → target primal (capability.call)
```

---

## S2: Songbird NAT Relay Shadow

### Current State

songbird ships a full RFC 5766 TURN client (Wave 205), STUN wire-compliant,
Cloudflare DDNS, and 5-tier `ConnectionFallbackChain`. The relay can replace
cloudflared tunnel for NAT traversal.

### What's Done

- `songbird-turn-client` crate: TURN allocation + channel-bind + refresh
- `primal.announce` wired for mesh discovery
- STUN/NAT traversal: 5-tier fallback (direct → STUN → TURN → relay → fail)
- `capability.resolve` (Wave 199-201) wire parity

### Remaining for Shadow

- [ ] **Deploy relay node**: songbird relay on sovereign infrastructure
  (same machine as bearDog TLS endpoint)
- [ ] **Shadow routing**: dual-path traffic — cloudflared and songbird relay
  in parallel, compare results
- [ ] **Metric collection**: connection setup time, keepalive reliability,
  reconnect frequency, bandwidth overhead
- [ ] **Cross-gate test**: 2+ gates connected via songbird relay (validates
  the covalent mesh pattern from exp073)

### Composition

```
Gate A → songbird relay (TURN allocation)
      → Gate B (bearDog + Tower Atomic)
      → capability.call across gates
```

---

## S3: Content Hosting Shadow (NestGate + PetalTongue)

### Current State

nestGate has transport parity on all 4 surfaces (Session 60). petalTongue
has `backend=nestgate` live (v1.6.6) with `content.resolve("/")` → live
dashboard SSE.

### What's Done

- nestGate: 8 `content.*` methods on all 4 transports (SemanticRouter, isomorphic IPC, HTTP, primary UDS)
- petalTongue: `GET /` → `content.resolve("/")`, SPA + CORS, live dashboard SSE
- Content pipeline smoke graph: `content_pipeline_smoke.toml`

### Remaining for Shadow

- [ ] **Mirror GitHub Pages content**: replicate current public site to
  nestGate content store via `content.put`
- [ ] **DNS shadow**: `staging.` subdomain pointing to petalTongue :8080
- [ ] **Metric collection**: TTFB, cache hit rate, 404 rate, content freshness
- [ ] **petalTongue static asset serving**: verify CSS/JS/image serving matches
  GitHub Pages behavior (cache headers, MIME types, compression)

### Composition

```
Browser → petalTongue :8080 (HTTP)
       → nestGate content.resolve (content-addressed storage)
       → BLAKE3 hash verification
```

---

## S4: Auth Shadow (BearDog BTSP + JupyterHub)

### Current State

bearDog has ionic tokens (Ed25519-signed, scoped, with expiry — Wave 102),
BTSP Phase 3 AEAD (ChaCha20-Poly1305), and FIDO2/CTAP2 hardware attestation.
The dual-auth path for JupyterHub (H2-2b) is ready to start.

### What's Done

- Ed25519 ionic tokens with `ttl_seconds`/`expires_at` (Wave 102)
- BTSP P3 full AEAD on all 13 primals
- FIDO2/CTAP2 `discover`/`register`/`authenticate` (Wave 103)
- Cross-primal token federation: `auth.public_key` + `BearDogVerifier` (Wave 99 + biomeOS v3.51)

### Remaining for Shadow

- [ ] **JupyterHub integration**: bearDog as auth provider for JupyterHub
  (replaces OAuth2 proxy). `BEARDOG_TLS_MODE=shadow` dual-auth config.
- [ ] **Session management**: bearDog token → JupyterHub session mapping
- [ ] **Metric collection**: auth latency (target: <50ms), session creation
  time, token refresh reliability, FIDO2 enrollment success rate

---

## Orchestration: biomeOS Shadow Deploy

biomeOS v3.53 shipped `composition.deploy.shadow` — dry-run validation with
3 routing tests. This is the orchestration layer for all shadow runs.

### Deploy Graph

```toml
[graph]
name = "sovereignty_shadow"
description = "4-track shadow run: TLS + NAT + content + auth"
shadow = true

[[graph.nodes]]
primal = "beardog"
by_capability = "crypto"
shadow_target = "cloudflare_tls"

[[graph.nodes]]
primal = "songbird"
by_capability = "relay"
shadow_target = "cloudflared_tunnel"

[[graph.nodes]]
primal = "nestgate"
by_capability = "content"
shadow_target = "github_pages"

[[graph.nodes]]
primal = "petaltongue"
by_capability = "visualization"
shadow_target = "github_pages_frontend"
```

---

## Membrane Telemetry

projectNUCLEUS has the membrane telemetry pipeline:
- `membrane_telemetry.sh` — collection script
- `membrane_7day.toml` — 7-day rolling window graph

Shadow runs feed into this pipeline. Each shadow emits metrics in the
skunkBat audit format so the provenance trio can track shadow-vs-commercial
performance over time.

---

## Timeline

| Phase | What | Duration | Gate |
|-------|------|----------|------|
| 1. Deploy | Stand up all 4 shadow services | 1 week | All services healthy |
| 2. Parallel | Dual-path traffic (shadow + commercial) | 2 weeks | Metrics flowing |
| 3. Measure | 7-day continuous parity measurement | 1 week | p95 ≤ 1.5× commercial |
| 4. Cutover | Switch DNS, retire commercial | — | Parity proven |

---

## Deliverables

| # | Deliverable | Owner | Blocks |
|---|-------------|-------|--------|
| D1 | bearDog ACME Phase 2 (auto-cert) | bearDog | S1 cutover |
| D2 | songbird relay node deployed | songbird | S2 shadow |
| D3 | nestGate content mirror + petalTongue staging DNS | nestGate + petalTongue | S3 shadow |
| D4 | bearDog JupyterHub dual-auth | bearDog | S4 shadow |
| D5 | 7-day membrane telemetry report (all 4 shadows) | projectNUCLEUS | Cutover gate |
| D6 | Cutover execution (DNS switch) | projectNUCLEUS | Stadial |

---

## References

- `BIOMEOS_V360_STADIAL_GATE_READINESS_MAY17_2026.md` — biomeOS readiness
- `specs/ACME_TLS_INTEGRATION_PATH.md` (bearDog) — ACME design
- `INTERSTADIAL_EXIT_CRITERIA.md` — L4 shadow run targets
- `WAVE22_COMPOSITION_PATTERNS_HANDOFF_MAY18_2026.md` — composition patterns
- primalSpring sovereignty scenarios: `membrane-composition`, `sovereignty-parity`, `content-sovereignty`
