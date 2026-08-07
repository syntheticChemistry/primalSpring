# NUCLEUS Lab Integration — benchScale + agentReagents

**Owner**: primalSpring (consumer) + benchScale (provider) + agentReagents (templates)
**Status**: Spec — infrastructure ready, integration pending
**Wave**: 157a (Aug 7, 2026)

---

## Purpose

primalSpring validates NUCLEUS compositions. Until now, validation ran against
the live overwatch NUCLEUS on eastGate or relied on structural-only checks
(no primals running). This creates two problems:

1. **Blast radius**: live NUCLEUS tests can disturb overwatch operations
2. **Coverage gap**: structural checks miss runtime routing bugs

benchScale + agentReagents solve both by providing isolated, reproducible
NUCLEUS instances for testing. primalSpring gains the ability to spin up a
fresh NUCLEUS, run N2-N5 validation, and tear it down — without touching the
production mesh.

---

## Infrastructure Available

### benchScale (`infra/benchScale/`)

Pure Rust lab substrate for reproducible distributed testing. Docker backend
(production), libvirt backend (Phase 4).

**JSON-RPC server** (`benchscale server --port 9200`):
- `lab.create` — spin up topology from YAML
- `lab.destroy` — tear down lab
- `lab.list` / `lab.status` — enumerate running labs
- `topology.validate` — structural check before creation
- `node.health` — per-node liveness
- `validate ipc ENDPOINT` — IPC compliance testing

**NUCLEUS topologies** (in `topologies/`):

| Topology | What | Use case |
|----------|------|----------|
| `nucleus/full_nucleus.yaml` | All 13 primals, 490+ methods | Full NUCLEUS validation |
| `nucleus/provenance_trio.yaml` | rhizoCrypt + loamSpine + sweetGrass + bearDog | N4 provenance routing |
| `nucleus/tower_membrane.yaml` | bearDog + songBird + skunkBat | N3 Tower validation |
| `ecoprimals-nucleus-3node.yaml` | Primary NUCLEUS + compute peer + mobile Tower | Cross-gate scenarios |
| `ecoprimals-nucleus-full.yaml` | Full federation (3+ gates) | Mesh-level validation |

**Network presets** for realistic conditions:

| Preset | Latency | Loss | Use case |
|--------|---------|------|----------|
| `basement_lan` | 0.1ms | 0% | Local LAN (house1→house2) |
| `home_lan` | 1ms | 0.01% | Same-site gates |
| `friend_wan` | 25ms | 0.1% | Cross-WAN (southGate) |
| `mobile_cell` | 50ms | 1% | Mobile gate (pixelGate) |
| `satellite` | 600ms | 2% | Extreme conditions |

**Binary flow**:
```
primal repos → musl build → plasmidBin/primals/ → deploy-ecoprimals.sh → lab node /opt/ecoprimals/bin/
```

### agentReagents (`infra/agentReagents/`)

Template-driven VM image builder. Gate templates pre-bake primal binaries
and systemd services.

**Key templates**:

| Template | Primals | Resources | Use case |
|----------|---------|-----------|----------|
| `gate-nucleus-full.yaml` | All 13 + membrane CLI | 8 GB / 4 vCPU | Full NUCLEUS lab node |
| `gate-ubuntu24-biomeos.yaml` | songBird + bearDog + nestGate | 4 GB / 2 vCPU | Standard biomeOS gate |
| `gate-ubuntu24-gpu-sovereign.yaml` | Node Atomic + VFIO | 16 GB / 8 vCPU | GPU compute testing |
| `gate-aarch64-pixelgate.yaml` | Tower Atomic (ARM64) | 4 GB / 4 vCPU | Cross-arch validation |

**Container substrate**:
- `containers/nucleus-lab-node/Dockerfile` — reusable Docker image
- Pre-bakes: iproute2, socat, ssh, jq, git, python3
- plasmidBin integration: primal binaries baked into image

---

## Integration Patterns

### Pattern 1: Local Docker Lab (fastest, recommended for N2-N5)

```
primalSpring experiment → benchScale lab.create(provenance_trio.yaml)
                        → deploy-ecoprimals.sh
                        → run validation against lab sockets
                        → benchScale lab.destroy
```

Requirements:
- Docker installed on eastGate
- plasmidBin musl binaries current
- benchScale server running (`benchscale server --port 9200`)

### Pattern 2: Parallel NUCLEUS on eastGate

Run a second NUCLEUS instance alongside the overwatch NUCLEUS using different
socket paths and family IDs:

```
BIOMEOS_SOCKET_DIR=/run/user/1000/biomeos-lab \
BIOMEOS_FAMILY_ID=lab-01 \
biomeos nucleus start --graphs-dir graphs/ --socket-dir /run/user/1000/biomeos-lab/
```

primalSpring discovers the lab instance via `$NEURAL_API_SOCKET` override:
```
NEURAL_API_SOCKET=/run/user/1000/biomeos-lab/neural-api-lab-01.sock \
cargo run --bin exp091_primal_routing_matrix
```

### Pattern 3: LAN NUCLEUS (cross-gate validation)

Reach NUCLEUS on other gates via WireGuard mesh for N6 validation:

| Gate | WG IP | NUCLEUS | Access |
|------|-------|---------|--------|
| strandGate | 10.13.37.10 | Full NUCLEUS v4.57+ | UDS via SSH tunnel or TCP fallback |
| westGate | 10.13.37.11 | Full NUCLEUS v4.57+ | Same |
| blueGate | 10.13.37.12 | Full NUCLEUS v4.57+ (Windows) | TCP only (no UDS cross-platform) |

Cross-gate access requires Tier 5 TCP or songBird TURN relay.
primalSpring uses `TOWER_HOST=10.13.37.10 BIOMEOS_PORT=9800` for TCP path,
or `NeuralBridge` cross-gate dispatch for relay path.

### Pattern 4: agentReagents VM (full isolation)

For destructive tests or gate-simulation scenarios:

```
agentReagents image.create(gate-nucleus-full.yaml) → VM image
benchScale lab.create(ecoprimals-nucleus-3node.yaml, backend=libvirt) → 3 VMs
primalSpring → validate across VM mesh
benchScale lab.destroy → clean up
```

Phase 4 (libvirt backend) required. Docker backend available now.

---

## primalSpring Experiment Integration

### exp116_benchscale_nucleus_lab

First experiment exercising benchScale for isolated NUCLEUS testing:

1. `lab.create` with `provenance_trio.yaml` topology
2. Deploy primals via `deploy-ecoprimals.sh`
3. Discover lab NUCLEUS via `NEURAL_API_SOCKET` override
4. Run N2 validation: `capability.call("crypto", "sign_ed25519", ...)` → bearDog
5. Run N4 validation: session-scoped provenance commit through trio
6. `lab.destroy` cleanup
7. Assert: all routing succeeded through lab NUCLEUS

### Future: `primalspring validate --lab`

New subcommand concept — run validation suite against a benchScale lab
instead of the live NUCLEUS:

```bash
primalspring validate --lab provenance_trio --n2 --n4
```

Automatically:
1. Checks benchScale server availability
2. Creates lab from specified topology
3. Deploys primals
4. Runs specified N-task validation
5. Reports results
6. Destroys lab

---

## Cross-References

| Document | Relationship |
|----------|-------------|
| `infra/benchScale/README.md` | benchScale architecture and usage |
| `infra/benchScale/topologies/` | Available topology definitions |
| `infra/agentReagents/templates/` | VM/container templates |
| `specs/STAGE2_ACTIVATION.md` | N1-N6 tasks this infrastructure supports |
| `specs/MIXED_COMPOSITION_PATTERNS.md` | L0-L3 validation layers |
