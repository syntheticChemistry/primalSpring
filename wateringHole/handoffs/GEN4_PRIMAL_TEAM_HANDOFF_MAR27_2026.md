# gen4 Primal Team Handoff — Phase 17 Learnings

**Date**: March 27, 2026
**From**: primalSpring Phase 17 (gen4 Deployment Evolution)
**To**: biomeOS, Squirrel, Songbird, BearDog, petalTongue teams

---

## Summary

primalSpring Phase 17 validated the biomeOS substrate model end-to-end: biomeOS
as neural-api orchestrator, capability routing across primals, cross-gate
deployment to Pixel via ADB, Squirrel AI integration, and spring deploy graph
validation for all 7 sibling springs. This handoff documents findings,
integration gaps, and evolution recommendations for each primal team.

---

## biomeOS Team

### What Was Validated
- **Coordinated mode on Eastgate**: 24 capability domains, 39 deploy graphs loaded
- **Capability routing**: `crypto.generate_keypair`, `beacon.generate`, `beacon.generate_encrypted`
  routed correctly through biomeOS → BearDog/Songbird
- **Graph listing API**: `graph.list` returns all loaded deploy graphs with metadata
- **Spring deploy sweep**: All 7 sibling spring deploy graphs + 4 pipeline graphs loaded and reported by API

### Integration Gaps Found
1. **Squirrel abstract socket routing**: Squirrel binds to abstract socket `@squirrel`.
   biomeOS attempts filesystem socket routing. This means `ai.*` capability routing
   through biomeOS to Squirrel fails at the transport layer. biomeOS needs to support
   abstract socket addresses in its routing table, or Squirrel needs to support
   filesystem sockets as a fallback.

2. **No aarch64 biomeOS binary**: Pixel deployments cannot run biomeOS as the substrate
   orchestrator. All Pixel validation relied on Eastgate biomeOS routing to remote
   Pixel primals via TCP. Cross-compiling biomeOS for `aarch64-unknown-linux-musl`
   would enable full sovereign substrate deployment on mobile/ARM.

3. **TCP routing awareness**: biomeOS routes to Unix sockets by default. For cross-gate
   scenarios (Eastgate → Pixel), primalSpring exp076 had to use raw TCP directly to
   reach Pixel BearDog/Songbird. biomeOS could expose a `route.add_tcp` capability
   to register cross-gate TCP endpoints in its routing table.

### Recommendations
- Support abstract Unix sockets in the primal routing table
- Cross-compile for aarch64 (static musl, ecoBin compliant)
- Consider a `route.register` or `route.add_tcp` method for cross-gate primal registration
- The `graph.list` API is solid; expand it to include graph health/status per-node

---

## Squirrel Team

### What Was Validated
- **Direct connection**: Abstract socket `@squirrel` via `UnixStream::connect_addr`
  with `SocketAddr::from_abstract_name("squirrel")`. Health check succeeds.
- **ai.* domain**: Squirrel registers `ai.query`, `ai.context.create`, `ai.context.list`
  capabilities in biomeOS.
- **Cross-primal AI**: Squirrel correctly discovers sibling primals via `env_sockets`

### Integration Gaps Found
1. **Abstract socket only**: Squirrel exclusively uses abstract Unix sockets (`@squirrel`)
   despite `--socket` CLI argument. This prevents:
   - biomeOS filesystem socket routing (`$XDG_RUNTIME_DIR/biomeos/squirrel-*.sock`)
   - `PrimalClient::connect()` which expects filesystem paths
   - Standard 5-tier discovery fallback

2. **primalSpring workaround**: exp077 uses `std::os::unix::net::SocketAddr::from_abstract_name`
   and `UnixStream::connect_addr` — this is Linux-specific and bypasses the standard
   discovery/transport stack.

### Recommendations
- Add filesystem socket support as primary or fallback (alongside abstract)
- Respect `--socket /path/to/squirrel.sock` CLI argument for filesystem binding
- Register filesystem socket path in biomeOS routing table on startup
- Abstract sockets are fine as an optimization, but filesystem path is the ecosystem standard

---

## Songbird Team

### What Was Validated
- **BirdSong beacons**: `birdsong.generate_encrypted_beacon` routed through biomeOS → Songbird
  generates valid encrypted beacons with BearDog crypto
- **Cross-gate health**: Pixel Songbird health via HTTP GET (`/health`) on port 19200
  (ADB-forwarded from Pixel 9200)
- **Mesh operations**: `mesh.init`, `mesh.announce`, `mesh.peers`, `mesh.status`
  validated on both Eastgate and Pixel Songbird instances
- **TCP JSON-RPC**: Pixel Songbird responds to JSON-RPC over TCP (newline-delimited)

### Integration Gaps Found
1. **BearDog socket discovery on Pixel**: Songbird tried `@biomeos_beardog` abstract socket
   for BearDog (not bound on Pixel). Required explicit `BEARDOG_SOCKET=tcp:127.0.0.1:9100`
   env var to connect to BearDog via TCP instead.

2. **Health endpoint protocol**: Songbird exposes HTTP `/health` (GET) while most primals
   use JSON-RPC `health.check`. This dual-protocol design is fine but requires
   experiment code to handle both paths.

### Recommendations
- Document the `BEARDOG_SOCKET` env var for cross-gate deployments where abstract sockets aren't available
- Consider also accepting JSON-RPC `health.check` alongside HTTP `/health` for uniformity
- The mesh initialization flow (init → announce → peers) works well; document the expected call sequence

---

## BearDog Team

### What Was Validated
- **Crypto routing**: `crypto.generate_keypair` and `crypto.sign` routed through
  biomeOS → BearDog correctly on Eastgate
- **Cross-gate TCP**: Pixel BearDog responds to JSON-RPC over TCP (port 19100,
  ADB-forwarded from Pixel 9100) — `health.check`, `crypto.generate_keypair`
- **Beacon seed generation**: BearDog `beacon.generate` produces valid beacon seeds
  consumed by Songbird for encrypted beacon generation

### Integration Gaps Found
1. **Abstract socket regression on Android**: BearDog v0.9.0 cannot bind filesystem
   sockets on GrapheneOS/Android due to SELinux restrictions. Falls back to TCP only.
   This is consistent with prior findings (Phase 13).

2. **Newline-delimited JSON-RPC**: BearDog TCP requires `\n`-terminated requests.
   Raw `nc` probes without trailing newline fail silently. exp076 uses
   `printf '...\\n' | nc -w 3` pattern.

### Recommendations
- Document the `\n`-terminated JSON-RPC requirement for TCP connections
- For Android/GrapheneOS: TCP is the correct transport; document as the standard for mobile deployments
- BearDog is the most reliable cross-gate primal; no major issues found

---

## petalTongue Team

### What Was Validated
- **Deploy graph presence**: `tower_ai_viz.toml` graph exists and loads in biomeOS
- **biomeOS integration**: petalTongue nodes appear in biomeOS graph listing,
  `visualization.render.dashboard` and `visualization.grammar` capabilities registered
- **Graph overlay model**: petalTongue composes at any atomic tier via overlay graphs
  (tower_ai_viz, nest_viz) — validated structurally

### Recommendations
- No blocking issues found
- petalTongue is the visualization surface for gen4 prototypes; the `graphs/gen4/gen4_interactive_substrate.toml`
  includes petalTongue as the UI layer alongside Squirrel AI and full crypto/mesh

---

## Cross-Cutting Learnings

### Transport Stack
- **Unix sockets**: Primary for same-host (Eastgate). 5-tier discovery works well.
- **TCP**: Required for cross-gate (Eastgate → Pixel via ADB port forwarding).
  `PrimalClient` only supports Unix sockets currently; raw TCP was used for cross-gate experiments.
- **Abstract sockets**: Linux-specific, used by Squirrel. Not compatible with standard
  discovery or biomeOS routing. Needs ecosystem-level decision on support.
- **HTTP**: Songbird health endpoint. Works but diverges from JSON-RPC standard.

### Deployment Patterns
- ADB port forwarding (`adb forward tcp:19100 tcp:9100`) is reliable for Pixel development
- `nohup` + `adb shell` hangs; use script-based restart (`adb push script && adb shell sh script`)
- Pixel primals need explicit env vars for inter-primal communication since abstract
  sockets aren't available

### gen4 Prototype Graphs
Four gen4 prototype deploy graphs are available in `primalSpring/graphs/gen4/`:
1. `gen4_sovereign_tower.toml` — Dark Forest ready (beardog + songbird + squirrel + primalspring)
2. `gen4_science_substrate.toml` — Multi-spring pipeline (beardog + songbird + toadstool + nestgate + primalspring)
3. `gen4_agentic_tower.toml` — AI-orchestrated (beardog + songbird + squirrel + biomeOS graph orchestration)
4. `gen4_interactive_substrate.toml` — Full surface (all interaction primals + crypto + mesh)

These are candidates for biomeOS to execute as first-class deploy graphs.

---

## Experiments Reference

| Exp | Domain | Primals Exercised |
|-----|--------|-------------------|
| 075 | biomeOS Neural API live | biomeOS, BearDog, Songbird |
| 076 | Cross-gate neural routing | BearDog (TCP), Songbird (HTTP+TCP) via Pixel |
| 077 | Squirrel AI bridge | Squirrel (abstract socket), biomeOS |
| 078 | petalTongue viz surface | petalTongue, biomeOS |
| 079 | Spring deploy sweep | biomeOS (all sibling spring graphs) |
| 080 | Cross-spring ecology | biomeOS (cross-spring capability routing) |

---

**License**: AGPL-3.0-or-later
