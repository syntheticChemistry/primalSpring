# Crypto Consumption Hierarchy Standard

> **Scope**: All 13 NUCLEUS primals and downstream springs/gardens.
> Defines how each primal consumes cryptographic capabilities, how bonding
> contexts escalate cipher requirements, and the recommended posture for
> crypto-consuming primals.
>
> **Canonical source**: `infra/wateringHole/` (this is the primalSpring
> local copy for spring context; the canonical version lives in the
> ecosystem-wide wateringHole).
>
> **Date**: May 2, 2026
> **Phase**: BTSP Phase 3 (ChaCha20-Poly1305 AEAD) convergence
> **License**: AGPL-3.0-or-later

---

## Part 1 — Primal Crypto Profiles

Every NUCLEUS primal uses cryptography. The question is not *whether* a
primal does crypto, but *which layers* it handles locally vs delegates.

Three layers exist:

| Layer | Purpose | Examples | Stakes if buggy |
|-------|---------|----------|-----------------|
| **Integrity** | Domain correctness | Blake3, SHA-256, Merkle proofs, content addressing | Silent data corruption |
| **Authentication** | Session establishment, identity proof | HMAC-SHA256, HKDF handshake key, Ed25519 (delegated) | Failed handshake (visible) |
| **Confidentiality** | Transport encryption | ChaCha20-Poly1305 AEAD, length-prefixed framing | Null cipher fallback (visible) |

### Per-Primal Profile

| Primal | Domain | Integrity (local) | Authentication | Confidentiality (Phase 3) |
|--------|--------|-------------------|----------------|---------------------------|
| **BearDog** | Security / crypto provider | Blake3, SHA-256 | All primitives (IS the Tower) | Full AEAD (provider) |
| **Songbird** | Networking / orchestration | — | BearDog-delegated | Full AEAD |
| **biomeOS** | Orchestration / federation | — | BearDog-delegated | Full AEAD |
| **rhizoCrypt** | Ephemeral DAG | Blake3 (vertex ID, Merkle) | Self-derived HKDF | Full AEAD |
| **loamSpine** | Permanent ledger | Blake3 (entry hash, chain link, Merkle) | BearDog-delegated | Full AEAD |
| **sweetGrass** | Semantic provenance | SHA-256 (content hash, braids) | BearDog-delegated | Full AEAD |
| **NestGate** | Sovereign storage | — | Self-derived HKDF | Full AEAD |
| **Squirrel** | AI coordination | — | BearDog-delegated | Full AEAD |
| **skunkBat** | Defensive security | — | BearDog-delegated | Full AEAD |
| **toadStool** | Compute scheduling | — | BearDog-delegated | Full AEAD |
| **barraCuda** | GPU math engine | — | BearDog-delegated | Full AEAD |
| **coralReef** | Shader compiler | — | BearDog-delegated | Full AEAD |
| **petalTongue** | Desktop UI | — | BearDog-delegated | Full AEAD |

**Observation (historical, now resolved)**: loamSpine already did Blake3 locally for content addressing,
chain linking, and Merkle proofs. This was cryptographic integrity — the
highest-stakes local crypto operation in its domain. Yet it declines to do
HKDF+AEAD, which has strictly lower failure consequences.

---

## Part 2 — Key Acquisition Patterns

Two patterns exist for obtaining the `handshake_key` used in Phase 3
session key derivation:

### Pattern A — Self-Derive from FAMILY_SEED

The primal reads `FAMILY_SEED` from its environment and derives the
handshake key locally:

```
handshake_key = HKDF-SHA256(ikm=family_seed, salt="btsp-v1", info="handshake")
```

BearDog is not in this code path. The primal independently computes the
same key that BearDog would, because both share the same family seed.

**Used by**: rhizoCrypt, NestGate

### Pattern B — Tower-Provided Key

The primal sends `family_seed` to BearDog via `btsp.session.create`,
then extracts the `handshake_key` (or `session_key`) from BearDog's
`btsp.session.verify` response. The primal receives the key material
from the Tower — it does not derive from raw seed.

```
primal → BearDog: btsp.session.create { family_seed: base64(...) }
BearDog → primal: { session_id, server_ephemeral_pub, challenge, ... }
  ... handshake continues ...
BearDog → primal: btsp.session.verify → { verified: true, session_key: base64(32 bytes) }
primal stores session_key as handshake_key for Phase 3
```

**Used by**: sweetGrass, biomeOS, Songbird, Squirrel, skunkBat,
barraCuda, petalTongue, toadStool, coralReef

### loamSpine's Current Posture

loamSpine uses Pattern B for Phase 2 handshake — it sends family_seed to
BearDog, receives session state, and verifies via `btsp.session.verify`.
However, it **discards the returned key material** and returns
`cipher: "null"` from `btsp.negotiate`. The minimum evolution is to stop
discarding the key and use it for Phase 3 HKDF.

### Delegation Boundary

Both patterns delegate **asymmetric crypto** (Ed25519 signing and
verification) to BearDog. This is a clean architectural boundary — private
key material stays in the Tower. No primal other than BearDog holds
signing keys.

The distinction is about **symmetric transport crypto**: HKDF key expansion
and ChaCha20-Poly1305 encrypt/decrypt. These operations MUST be local
because round-tripping every IPC frame through BearDog would destroy
latency and create a single point of failure.

---

## Part 3 — Bonding Escalation Hierarchy

The bonding model defines minimum cipher floors per bond type. This is
enforced by `min_cipher_for_bond()` in `ecoPrimal/src/btsp/mod.rs`:

| Bond Type | Trust Model | Min Cipher | Primal Requirement |
|-----------|-------------|------------|-------------------|
| **Covalent** | SharedFamilySeed / NuclearLineage | `Null` | All primals qualify |
| **Metallic** | Organizational / MitoBeaconFamily | `HmacPlain` | Authenticated transport minimum |
| **OrganoMetalSalt** | HybridTrust / Contractual | `HmacPlain` | Same as Metallic |
| **Ionic** | Contractual / MitoBeaconFamily | `ChaCha20Poly1305` | **Phase 3 AEAD required** |
| **Weak** | ZeroTrust | `ChaCha20Poly1305` | **Phase 3 AEAD required** |

### Consequence

Any primal that returns `cipher: "null"` from `btsp.negotiate` **cannot
participate in ionic or weak bond compositions**. The `BtspEnforcer`
will reject the connection at handshake time.

This is not a hypothetical constraint. The ecosystem already defines
ionic bond compositions in canonical standards:

- `healthSpring` dual-tower ionic-fenced enclave
- Cross-family data federation (sharded read keys)
- `crypto.ionic_bond.*` RPC surface on BearDog (propose/accept/seal)
- Idle compute federation (metallic → ionic edge)

### Trust Ordering

```
Covalent > Metallic > Ionic > Weak

Higher trust = lower cipher floor (more trust, less encryption needed)
Lower trust  = higher cipher floor (less trust, more encryption needed)
```

---

## Part 4 — Composition Contexts

### The Atomic Model

From `NUCLEUS_SPRING_ALIGNMENT.md` and `DESKTOP_NUCLEUS_DEPLOYMENT.md`:

| Atomic | Particle | Primals | Fragment |
|--------|----------|---------|----------|
| **Tower** | electron | BearDog + Songbird | `tower_atomic` |
| **Node** | proton | Tower + ToadStool + barraCuda + coralReef | `node_atomic` |
| **Nest** | neutron | Tower + NestGate + rhizoCrypt + loamSpine + sweetGrass | `nest_atomic` |
| **NUCLEUS** | atom | Tower + Node + Nest (9 unique primals) | `nucleus` |
| **Meta-tier** | cross-atomic | biomeOS + Squirrel + petalTongue | `meta_tier` |
| **Desktop NUCLEUS** | full 12 | All atomics + meta | `nucleus_desktop_cell` |

**Key principle**: Tower mediates ALL inter-atomic bonding. No cross-gate
communication happens without passing through the electron shell
(BearDog + Songbird).

### Composition Scenarios and Crypto Requirements

**1. Same-family covalent (local Desktop NUCLEUS)**

Single user, single machine. All primals share `FAMILY_SEED` and are in
the same trust domain. Null cipher is sufficient per `min_cipher_for_bond`.
loamSpine works as-is. This is the development and personal-use case.

**2. healthSpring ionic-fenced enclave**

Dual-tower pattern from `NUCLEUS_SPRING_ALIGNMENT.md` Phase 47:
- Tower A (data custody): NestGate-A + Provenance Trio A behind an
  **ionic fence** — "data cannot leave Tower A as raw"
- Tower B (analytics): receives only de-identified aggregates via an
  **ionic bridge**
- BearDog enforces cross-family ionic bond

loamSpine sits inside the Nest atomic (neutron) within Tower A.
If loamSpine cannot sustain ChaCha20 transport, the ionic fence has a
plaintext gap. The data that "cannot leave as raw" traverses an
unencrypted channel.

**3. Cross-family data federation with read keys**

When friends federate storage and pass sharded read keys, the transport
between NUCLEUS instances is ionic-bonded. Every primal in the Nest
atomic (NestGate, rhizoCrypt, loamSpine, sweetGrass) must handle
encrypted framing. A single primal returning `cipher: "null"` creates
a plaintext gap in the federation.

**4. Compute enclave validation**

Proving computation was isolated requires authenticated + encrypted
channels to trust the attestation. The anchoring pipeline
(`ANCHORING_PIPELINE.md`) chains:

```
rhizoCrypt DAG vertices
  → BearDog Ed25519 signatures
    → Merkle root
      → loamSpine certificate (permanent record)
        → sweetGrass attribution braid
          → public chain anchor (Bitcoin/Ethereum)
```

If loamSpine's transport is unencrypted, the certificate's chain of
custody has a plaintext gap between the signed Merkle root and the
permanent ledger entry.

**5. Metallic compute pool**

GPU fleet sharing across an organization (e.g. hotSpring multi-GPU
dispatch). `HmacPlain` is the minimum, but practical deployments carry
tensor results on the data plane and will want ChaCha20 for
confidentiality.

**6. Weak / defensive mesh**

skunkBat threat intelligence sharing across unknown peers. ChaCha20
required, ZeroTrust accepted. All participants must implement Phase 3.

**7. Plasmodium (multi-NUCLEUS covalent)**

Two or more NUCLEUS instances bonded covalently across machines.
Even though covalent bonds have a Null cipher floor, crossing machine
boundaries makes encrypted transport practically necessary. Network
traffic is observable; localhost UDS traffic is not.

---

## Part 5 — The Integrity Paradox

Primals that already perform local cryptographic integrity operations
(Blake3, SHA-256) for their core domain have a **higher-stakes** local
crypto dependency than HKDF+AEAD would represent.

| Operation | Local in loamSpine? | Consequence of bug |
|-----------|--------------------|--------------------|
| Blake3 content addressing | Yes | Silent data corruption — chain broken |
| Blake3 chain linking | Yes | Silent integrity loss — undetected tampering |
| Blake3 Merkle proofs | Yes | Invalid certificates — trust chain broken |
| HKDF-SHA256 key derivation | No | Null cipher fallback — visible, graceful |
| ChaCha20-Poly1305 AEAD | No | Null cipher fallback — visible, graceful |

loamSpine already trusts itself with the highest-stakes crypto in its
domain. The refusal to do HKDF+AEAD is not a security boundary — it is
an artificial one that creates real composition gaps.

The same paradox applies to any primal doing local hashing:
- rhizoCrypt: Blake3 for vertex IDs and Merkle trees (resolved — full AEAD)
- sweetGrass: SHA-256 for content hashes and braids (resolved — full AEAD)
- NestGate: full AEAD (module wired, deps added, transport connected)

---

## Part 6 — Recommended Posture per Primal Role

### Tier 1 — Crypto Provider

**BearDog** only. Implements all cryptographic primitives: signatures,
AEAD, KDF, KEX, hashing, HMAC, genetics, beacons, ionic bonds. IS the
Tower electron shell. Exposes ~150+ JSON-RPC methods for ecosystem
consumption. All other primals delegate asymmetric crypto here.

### Tier 2 — Crypto-Native Consumer

Primals whose core domain involves crypto-adjacent operations:

| Primal | Why crypto-native |
|--------|-------------------|
| **rhizoCrypt** | Content-addressed DAG, Merkle trees, Blake3 integrity |
| **sweetGrass** | SHA-256 content hashes, provenance braids, attribution |
| **skunkBat** | Threat detection, defensive security, network analysis |
| **Songbird** | Network orchestration, TLS mediation, protocol routing |

These primals should always ship Phase 3 AEAD. They may use Pattern A
(self-derive) or Pattern B (Tower-provided) for key acquisition.
Crypto is natural in their domain.

### Tier 3 — Crypto-Consuming Consumer

Primals whose core domain is not crypto but who consume crypto for
transport security:

| Primal | Core domain | Crypto consumption |
|--------|-------------|-------------------|
| **loamSpine** | Permanent ledger | Blake3 integrity + Full AEAD (resolved) |
| **NestGate** | Sovereign storage | Full AEAD (module wired, resolved) |
| **coralReef** | Shader compiler | Full AEAD (transport wired, resolved) |
| **petalTongue** | Desktop UI | Full AEAD (done) |
| **toadStool** | Compute scheduling | Full AEAD (done) |
| **barraCuda** | GPU math engine | Full AEAD (done) |
| **Squirrel** | AI coordination | Full AEAD (done) |
| **biomeOS** | Orchestration | Full AEAD (done) |

Pattern B (Tower-provided key) is the recommended approach for this
tier. The key material comes from BearDog; the primal only needs to:

1. Parse `session_key` from `btsp.session.verify` response
2. Add `hkdf`, `sha2`, `chacha20poly1305`, `zeroize` as deps
3. Implement `SessionKeys::derive()` (HKDF-SHA256 expansion)
4. Wire `encrypt`/`decrypt` into a length-prefixed frame loop
5. Switch transport after successful `btsp.negotiate`

This is the same pattern 10 of 13 primals have already shipped. The
RustCrypto crates are pure Rust, pass `deny.toml`, and add no C/asm
dependencies.

### The Delegation Refinement

The principle "delegate all crypto to Tower" needs three-layer
refinement:

| Crypto layer | Where it runs | Why |
|-------------|---------------|-----|
| **Asymmetric** (Ed25519 sign/verify) | BearDog (Tower) | Private keys stay in Tower. Clean boundary. |
| **Symmetric transport** (HKDF + ChaCha20) | Local (in-process) | Cannot round-trip every frame through BearDog. Latency + SPOF. |
| **Integrity** (Blake3 / SHA-256) | Local (in-process) | Already local. Highest stakes. |

Delegating symmetric transport crypto to BearDog is architecturally
impossible for the same reason you cannot delegate Blake3 hashing to
BearDog: every data access would require an IPC round-trip to hash or
encrypt, destroying the primal's ability to function.

---

## Part 7 — Resolution Status (ALL RESOLVED — May 2, 2026)

All three primals shipped full Phase 3 AEAD. 13/13 NUCLEUS primals are now
at full encrypted framing, completing ecosystem convergence.

### loamSpine — RESOLVED (`3dcd6b7`)

**Was**: `IONIC-BOND-BLOCKING` — `cipher: "null"`, discarded Tower key.

**Shipped**: `crates/loam-spine-core/src/btsp/phase3.rs` (382 LOC) with
full `SessionKeys` (HKDF-SHA256 + ChaCha20-Poly1305 + zeroize on drop).
Pattern B Tower-provided key accepted from BearDog's `btsp.session.verify`
response via `decode_session_key()`. `negotiate_btsp` returns
`cipher: "chacha20-poly1305"` when handshake key present, null cipher
graceful fallback otherwise. Async `read_encrypted_frame`/`write_encrypted_frame`
transport with standard `[4B len][12B nonce][ciphertext + tag]` wire format.
Deps added: `chacha20poly1305` 0.10, `hkdf` 0.13, `zeroize` 1.8.2.
Consistent with loamSpine's delegation principle — key material from Tower,
primal performs symmetric expansion and framing only.

### coralReef — RESOLVED (`f2d6bcf`)

**Was**: `CRYPTO-READY` — crypto implemented but transport not wired.

**Shipped**: `unix_jsonrpc.rs` fully upgraded with `handle_connection`
that checks first line for `btsp.negotiate`, calls `take_negotiated_keys(sid)`,
then enters `process_encrypted_frames` loop with 8 MiB frame guard.
`btsp_negotiate.rs` upgraded from null-only to real AEAD key derivation.
`dead_code` annotations removed. Full transport path connected.

### NestGate — RESOLVED (`ef3ac568f`)

**Was**: `MODULE-PENDING` — code on disk, not compiled or wired.

**Shipped**: `pub mod btsp_phase3;` declared in `rpc/mod.rs`. `hkdf` 0.12 +
`zeroize` 1 (with `derive` feature) added to workspace `Cargo.toml`.
`try_phase3_negotiate` + `run_encrypted_frame_loop` wired into both
`unix_socket_server/mod.rs` and `isomorphic_ipc/server/mod.rs` accept
paths. All three original gaps (module declaration, deps, wiring) closed.

---

## Appendix A — Genetics Tiers and Bond Alignment

From `NUCLEUS_SPRING_ALIGNMENT.md`:

| Tier | Type | Role | Cloneable | Bond Minimum |
|------|------|------|-----------|--------------|
| 1 | Mito-Beacon | Discovery, NAT, metadata | Yes | Metallic, Ionic |
| 2 | Nuclear | Permissions, auth, sessions | No (spawn fresh) | Covalent |
| 3 | Tag | Open channels (deprecated) | Yes | — |

Covalent bonds require NuclearLineage trust — nuclear genetics spawned
fresh per generation, never copied. Ionic and metallic bonds require
MitoBeaconFamily trust — discovery without sharing nuclear credentials.

The two-phase BTSP model ensures discovery (Phase 1, mito-beacon) never
exposes authorization material (Phase 2, nuclear session). Phase 3
(encrypted channel) adds confidentiality on top of the authenticated
session.

## Appendix B — Cross-References

| Document | Location |
|----------|----------|
| Bond types and trust model | `ecoPrimal/src/bonding/mod.rs` |
| Cipher floor enforcement | `ecoPrimal/src/btsp/mod.rs` `min_cipher_for_bond()` |
| Atomic composition types | `ecoPrimal/src/coordination/mod.rs` |
| Ionic bond protocol | `ecoPrimal/src/bonding/ionic.rs`, `ionic_rpc.rs` |
| NUCLEUS spring alignment | `infra/wateringHole/NUCLEUS_SPRING_ALIGNMENT.md` |
| Desktop NUCLEUS deployment | `infra/wateringHole/DESKTOP_NUCLEUS_DEPLOYMENT.md` |
| BearDog crypto stack | `infra/wateringHole/btsp/BEARDOG_TECHNICAL_STACK.md` |
| Anchoring pipeline | `infra/whitePaper/gen4/architecture/ANCHORING_PIPELINE.md` |
| Composition patterns | `infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md` |
| BTSP Phase 3 status | `docs/PRIMAL_GAPS.md` (Phase 3 scoreboard) |
| Upstream crosstalk | `wateringHole/UPSTREAM_CROSSTALK_AND_DOWNSTREAM_ABSORPTION.md` |
| Dark Forest genetics | `infra/wateringHole/birdsong/DARK_FOREST_BEACON_GENETICS_STANDARD.md` |
| Primal registry | `infra/wateringHole/PRIMAL_REGISTRY.md` |
