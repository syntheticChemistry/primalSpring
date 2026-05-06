# NUCLEUS IPC Method Map

> Verified live against Desktop NUCLEUS (Phase 58, May 3, 2026).
> Every method below was confirmed via JSON-RPC over UDS.
> Phase 58: skunkBat wired as 13th NUCLEUS primal (defense/recon meta-tier).
> Upstream absorbed: NestGate v0.4.70 S48 (encrypt-at-rest, auth bypass),
> biomeOS v3.30 (deep debt), Songbird W178 (anyhow), Squirrel AN (HTTP providers,
> DISCOVERY_SOCKET resolution, crypto foundation), BearDog W75 (purpose-key module
> extraction), barraCuda Sprint 47b (role-based naming, self-registration),
> sweetGrass v0.7.28 (braid + anchor signing delegation).

All primals respond to `health.liveness` (status: "alive").
Discovery: most expose `primal.capabilities` or `capabilities.list` or `rpc.methods`.

## Service Mesh (Tower Discovery)

After startup, the composition layer registers all primals with Songbird
via `ipc.register`. Any primal or composition can then resolve capabilities
**without knowing primal names or socket paths**:

```
ipc.resolve   {"capability": "tensor"}  -> native_endpoint: unix://...barracuda-{fid}.sock
ipc.discover  {"capability": "dag"}     -> providers: [{primal_id: "rhizocrypt", ...}]
```

Primals don't know it's Songbird — they discover a "discovery" capability
socket and call `ipc.register`. The composition launcher handles this today.
Upstream primals should evolve to self-register at startup by probing for
`DISCOVERY_SOCKET` or `{XDG_RUNTIME_DIR}/biomeos/discovery-{family}.sock`.

### Self-Registration Pattern (for upstream primals)

At startup, each primal should:
1. Look for `DISCOVERY_SOCKET` env var (set by launcher)
2. Fall back to `{SOCKET_DIR}/discovery-{FAMILY_ID}.sock` (filesystem probe)
3. If found, send `ipc.register` with own capabilities and endpoint
4. If not found, continue in standalone mode (niche fallback: skip)

```json
{"jsonrpc":"2.0","method":"ipc.register","params":{
  "primal_id":"barracuda",
  "capabilities":["tensor","math","stats","linalg","ml"],
  "endpoint":"unix:///run/user/1000/biomeos/barracuda-{family}.sock"
},"id":1}
```

---

## Tower (electron)

### BearDog — Security / Crypto

Socket: `beardog-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `crypto.*` | `blake3_hash`, `sha3_256`, `sha256`, `sha512`, `hmac_sha256`, `sign`, `verify`, `encrypt` (+ purpose-key routing W74), `decrypt` (+ purpose-key routing W74), `derive_purpose_key` (W72), `sign_registration` (W72), `ed25519_generate_keypair`, `sign_ed25519`, `verify_ed25519`, `x25519_generate_ephemeral`, `x25519_derive_secret`, `chacha20_poly1305_encrypt/decrypt`, `aes256_gcm_encrypt/decrypt`, `argon2id_hash/verify` |
| `btsp.*` | `session.create`, `session.negotiate`, `session.verify`, `tunnel.establish`, `tunnel.encrypt`, `tunnel.decrypt`, `tunnel.status`, `tunnel.close`, `contact.exchange`, `verify_peer` |
| `beacon.*` | `generate`, `get_id`, `list_known`, `add_known`, `encrypt`, `try_decrypt` |
| `birdsong.*` | `encrypt`, `decrypt`, `generate_encrypted_beacon` |
| `genetic.*` | `derive_device_seed`, `derive_lineage_key`, `mix_entropy`, `generate_challenge`, `verify_lineage` |
| `secrets.*` | `store`, `retrieve` (+ lazy purpose-key derivation W74), `list`, `delete` |
| `tls.*` | `derive_handshake_secrets`, `derive_application_secrets`, `sign_handshake`, `verify_certificate` |
| `graph.*` | `validate_template`, `audit_origin`, `authorize_modification` |
| Meta | `rpc.methods`, `primal.info`, `capabilities.list`, `identity.get`, `health.*` |

### Songbird — Discovery / Networking

Socket: `songbird-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `ipc.*` | `register`, `resolve`, `resolve_by_name`, `discover`, `list` |
| `http.*` | `request`, `get`, `post` |
| `stun.*` | `get_public_address`, `bind`, `serve`, `stop`, `detect_nat_type` |
| `igd.*` | `discover`, `map_port`, `unmap_port`, `external_ip`, `auto_configure` |
| `relay.*` | `serve`, `stop`, `allocate`, `status` |
| `mesh.*` | `init`, `status`, `find_path`, `announce`, `peers`, `topology`, `auto_discover` |
| `onion.*` | `start`, `stop`, `status`, `connect`, `address` |
| `tor.*` | `status`, `connect`, `service.start`, `circuit.build` |
| `birdsong.*` | `generate_encrypted_beacon`, `decrypt_beacon`, `verify_lineage`, `advertise` |
| `punch.*` | `request`, `coordinate`, `status` |
| `peer.*` | `connect` |
| Meta | `rpc.discover`, `rpc.methods`, `capabilities.list`, `identity`, `health.*` |

---

## Node (proton)

### ToadStool — Compute Dispatch

Socket: `toadstool-{family_id}.sock`

| Method | Description |
|--------|-------------|
| `compute.capabilities` | Available resources (CPU cores, memory, GPU) |
| `compute.dispatch` | Submit a compute job |
| Meta | `health.*`, `capabilities.list`, `identity.get` |

**Encrypted dispatch** (S205): `compute.dispatch.submit` encrypts payloads via BearDog
when `BEARDOG_SOCKET` is available. Purpose key retrieved lazily via
`secrets.retrieve("nucleus:{family}:purpose:compute")`, cached after first call.
Payload encrypted with `crypto.encrypt` (ChaCha20-Poly1305), result decrypted with
`crypto.decrypt`. Standalone mode: plaintext (zero behavioral change).

**Self-registration** (S207): At startup, calls `register_with_discovery()` via
`DISCOVERY_SOCKET` — sends `ipc.register` with `primal_id: "toadstool"`,
capabilities `["compute.dispatch","compute.capabilities"]`. Fire-and-forget.
Both `run_server_main` and `DaemonServer` startup paths covered.

### barraCuda — Tensor / Math Engine

Socket: `barracuda-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `tensor.*` | `create`, `matmul`, `matmul_inline`, `add`, `scale`, `clamp`, `reduce`, `sigmoid`, `batch.submit` |
| `stats.*` | `mean`, `std_dev`, `variance`, `correlation`, `pearson`, `weighted_mean`, `chi_squared`, `anova_oneway`, `eigh` |
| `linalg.*` | `solve`, `eigenvalues`, `svd`, `qr` |
| `spectral.*` | `fft`, `power_spectrum`, `stft` |
| `activation.*` | `softmax`, `gelu`, `fitts`, `hick`, `sigmoid` (via `math.sigmoid`) |
| `ml.*` | `mlp_forward`, `attention` |
| `noise.*` | `perlin2d`, `perlin3d` |
| `rng.*` | `uniform` |
| `fhe.*` | `ntt`, `pointwise_mul` |
| `device.*` | `list`, `probe` |
| Meta | `primal.capabilities`, `capabilities.list`, `identity.get`, `health.*` |

### coralReef — GPU Shader Compiler

Socket: `coralreef-core-default.sock` (JSON-RPC)
Note: also has tarpc socket at `coralreef-{family_id}.sock`.

| Method | Description |
|--------|-------------|
| `health.liveness` | `{"alive": true}` |
| `health.check` | Extended health with supported architectures |

> coralReef's primary capabilities are consumed via tarpc (not JSON-RPC).
> barraCuda delegates shader compilation to coralReef when GPU is available.

---

## Nest (neutron)

### NestGate — Storage

Socket: `nestgate-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `storage.*` | `store`, `retrieve`, `exists`, `delete`, `list`, `stats`, `store_blob`, `retrieve_blob`, `retrieve_range`, `store_stream`, `store_stream_chunk`, `retrieve_stream`, `retrieve_stream_chunk` |
| Meta | `capabilities.list`, `identity.get`, `discover_capabilities`, `health.*` |

**Encrypt-at-rest** (v0.4.70 S48): `storage.store`/`retrieve`/`store_blob`/`retrieve_blob`
auto-encrypt/decrypt transparently when a key is available. Key resolution:
1. `NESTGATE_ENCRYPTION_KEY` env (hex-64 or base64-44)
2. BearDog `secrets.retrieve("nucleus:{family}:purpose:storage")` via `BEARDOG_SOCKET`
3. None — standalone mode, plaintext (backward compat)

Envelope: `{"v":1,"ct":"<b64>","n":"<b64>","alg":"chacha20-poly1305"}`. Unencrypted
data on disk detected and returned as-is (migration-safe).

**Auth bypass** (v0.4.70 S48): `NESTGATE_AUTH_MODE=beardog` skips JWT validation at
startup — delegates auth to BearDog within NUCLEUS compositions.

### rhizoCrypt — Working Memory (DAG)

Socket: `rhizocrypt-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `dag.session.*` | `create`, `get`, `list`, `discard` |
| `dag.event.*` | `append`, `append_batch` |
| `dag.vertex.*` | `get`, `query`, `children` |
| `dag.slice.*` | `get`, `list`, `checkout`, `resolve` |
| `dag.frontier.*` | `get` |
| `dag.genesis.*` | `get` |
| `dag.merkle.*` | `root`, `proof`, `verify` |
| `dag.dehydration.*` | `trigger`, `status` |
| Meta | `primal.capabilities`, `capabilities.list`, `identity.get`, `health.*`, `tools.list`, `tools.call` |

### loamSpine — Permanent Ledger

Socket: `loamspine-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `spine.*` | `create`, `query` |
| `certificate.*` | `issue`, `transfer`, `verify` |
| `bonding.ledger.*` | `store`, `retrieve`, `list` |
| `slice.*` | `anchor`, `checkout` |
| `proof.*` | `generate`, `verify` |
| `anchor.*` | `publish`, `verify` |
| `braid.*` | `commit` |
| Meta | `primal.capabilities`, `capability.list`, `health.check` |

**Tower-signed entries** (Apr 28): `entry.append` and `session.commit` sign entries
via BearDog `crypto.sign_ed25519` when `BEARDOG_SOCKET` is set. Entry metadata carries
`tower_signature` (base64 Ed25519) and `tower_signature_alg`. Chain hash commits to
the signed entry. `prepare_entry()` + `append_prepared_entry()` split enables signing
between creation and chain append. Standalone mode (no BearDog) produces unsigned entries.
BTSP tunnel consumption documented as next frontier — loamSpine completes the 4-step
handshake but does not yet use tunnels for encrypted replication. 1,509 tests.

### sweetGrass — Provenance

Socket: `sweetgrass-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `braid.*` | `create`, `get`, `get_by_hash`, `query`, `delete`, `commit` |
| `anchoring.*` | `anchor`, `verify` |
| `provenance.*` | `graph`, `export_provo`, `export_graph_provo` |
| `attribution.*` | `chain`, `calculate_rewards`, `top_contributors` |
| `compression.*` | `compress_session`, `create_meta_braid` |
| `contribution.*` | `record`, `record_session`, `record_dehydration` |
| `pipeline.*` | `attribute` |
| `composition.*` | `tower_health`, `node_health`, `nest_health`, `nucleus_health` |
| Meta | `capabilities.list`, `identity.get`, `health.*`, `tools.list`, `tools.call` |

**Signing delegation** (v0.7.28): `braid.create` delegates signing to BearDog
`crypto.sign` (Ed25519 over UDS JSON-RPC) via `CryptoDelegate` module. Braids carry
`Witness::from_tower_ed25519` with `tier: "tower"` and `did:key:z6Mk...` agent DID
constructed from BearDog's public key. `anchoring.anchor` also delegates signing.
Graceful degradation: unsigned `tier: "open"` witnesses when BearDog is unavailable.
Socket resolution: `BEARDOG_SOCKET` → `SECURITY_PROVIDER_SOCKET` →
`BIOMEOS_SOCKET_DIR/security.sock` → `XDG_RUNTIME_DIR/biomeos/security.sock`.

---

## Meta (cross-atomic)

### biomeOS — Coordinator

Socket: `biomeos.sock` or `neural-api-{family_id}.sock`

Coordinator primal. Manages deployment graphs and lifecycle.
Method introspection via its own `neural-api` endpoint.

### Squirrel — AI Agent

Socket: `squirrel-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `inference.*` | `complete`, `embed`, `models`, `register_provider`, `unregister_provider` |
| `ai.*` | `query`, `complete`, `chat`, `list_providers` |
| `context.*` | `create`, `update`, `summarize` |
| `tool.*` | `execute`, `list` |
| `graph.*` | `parse`, `validate` |
| `capability.*` | `announce`, `discover`, `list` |
| `discovery.*` | `peers` |
| `lifecycle.*` | `register`, `status` |
| Meta | `capabilities.list`, `identity.get`, `system.*`, `health.*` |

**HTTP provider support** (session AN): `inference.register_provider` accepts
`endpoint` param (`http://host:port`) for HTTP-based providers (e.g. Ollama).
Transport: lightweight raw TCP HTTP/1.1 to Ollama REST (`/api/generate`,
`/api/embeddings`). Health: TCP connect probe. No new deps (Tower Atomic).

**Discovery resolution** (session AN): `discover_capability()` now queries
`DISCOVERY_SOCKET` via `discovery.find_provider` as Method 2 (after explicit
env vars, before registry + socket scan). Graceful fallthrough.

**Crypto foundation** (session AN): `SecurityProviderClient` extended with
`retrieve_purpose_key(purpose)`, `encrypt_with_purpose(data, purpose)`,
`decrypt_with_purpose(envelope, purpose)` — wires to BearDog `secrets.retrieve`,
`crypto.encrypt`, `crypto.decrypt`. Awaiting BearDog server-side support.

### petalTongue — Visualization / Desktop UI

Socket: `petaltongue-{family_id}.sock`

| Namespace | Key Methods |
|-----------|------------|
| `visualization.*` | `render`, `render.stream`, `render.grammar`, `render.dashboard`, `render.scene`, `interact`, `interact.subscribe`, `provenance`, `export`, `validate` |
| `motor.*` | `set_panel`, `set_zoom`, `set_mode`, `fit_to_view`, `navigate` |
| `sensor.*` | `stream.subscribe` |
| `interaction.*` | `subscribe`, `poll` |
| `modality.*` | `visual`, `audio`, `terminal`, `haptic`, `braille`, `description` |
| `audio.*` | `synthesize` |
| `proprioception.*` | `get` |
| Meta | `capabilities.list`, `identity.get`, `lifecycle.status`, `health.*` |

**SceneGraph schema** for `visualization.render.scene`:
```json
{
  "scene": {
    "nodes": {
      "<node_id>": {
        "id": "<node_id>",
        "transform": {"a":1,"b":0,"tx":0,"c":0,"d":1,"ty":0},
        "primitives": [{"Text": {"x":0,"y":0,"content":"...","font_size":16,"color":{"r":1,"g":1,"b":1,"a":1},"anchor":"TopLeft","bold":false,"italic":false,"data_id":null}}],
        "children": [],
        "visible": true,
        "opacity": 1.0,
        "label": null,
        "data_source": null
      }
    },
    "root_id": "root"
  },
  "session_id": "my-session"
}
```

Primitive types: `Text`, `Rect`, `Point`, `Line`, `Polygon`, `Arc`, `Path`, `Image`, `Mesh3D`, `Sprite`.

### skunkBat — Defense / Recon (Phase 58)

Socket: `skunkbat-{family_id}.sock`  
TCP fallback: `9140`  
`required = false` (meta-tier enhancer)

| Namespace | Key Methods |
|-----------|------------|
| `defense.*` | `baseline_observe`, `status`, `alert` |
| `recon.*` | `metadata_scan`, `peers`, `topology` |
| `threat.*` | `assess`, `report` |
| `lineage.*` | `verify`, `chain` |
| `btsp.*` | `negotiate` |
| Meta | `health.liveness`, `capabilities.list`, `identity.get` |

skunkBat participates passively in all bonding contexts. It monitors connection
patterns, validates lineage chains, and flags anomalies. It does not gate
connections — primals function identically with or without it.
