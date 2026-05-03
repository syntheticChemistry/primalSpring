# BearDog Crypto Wire Contract

> Interface specification for downstream springs coding against BearDog's JSON-RPC surfaces.
> Date: May 2, 2026 | BTSP Phase: 3 (13/13 FULL AEAD — ChaCha20-Poly1305 encrypted framing)

## Transport

| Channel | Protocol | Auto-detection |
|---------|----------|----------------|
| UDS | JSON-RPC 2.0 over raw TCP framing | First-byte peek: `{` → JSON-RPC, else BTSP binary |
| TCP | JSON-RPC 2.0 over raw TCP or BTSP binary | Same peek pattern |

Socket path: `/run/user/$UID/primal-beardog-$FAMILY_ID.sock`

## Namespaces

BearDog exposes ~150+ methods across these handler groups. Method names prefixed
with `beardog.` are cross-primal aliases of the canonical `crypto.*` names.

---

### `crypto.*` — Core Cryptographic Operations (97 methods)

**Signatures**

| Method | Params | Response |
|--------|--------|----------|
| `crypto.sign` / `crypto.sign_ed25519` | `{ message: base64, key_id?: string }` | `{ signature: base64, algorithm: "ed25519" }` |
| `crypto.verify` / `crypto.verify_ed25519` | `{ message: base64, signature: base64, public_key: base64 }` | `{ valid: bool }` |
| `crypto.ecdsa.p256.sign` | `{ message: base64, key_id?: string }` | `{ signature: base64, algorithm: "ecdsa-p256" }` |
| `crypto.ecdsa.p384.sign` | `{ message: base64, key_id?: string }` | `{ signature: base64, algorithm: "ecdsa-p384" }` |

**Key Exchange**

| Method | Params | Response |
|--------|--------|----------|
| `crypto.generate_keypair` | `{ algorithm?: "ed25519"\|"x25519"\|"p256" }` | `{ public_key: base64, key_id: string }` |
| `crypto.x25519_generate_ephemeral` | `{}` | `{ public_key: base64, key_id: string }` |
| `crypto.x25519_derive_secret` | `{ our_key_id: string, their_public: base64 }` | `{ shared_secret: base64 }` |
| `crypto.ecdh_derive` / `crypto.ecdhe.p256.generate` | `{ their_public: base64 }` | `{ shared_secret: base64, our_public: base64 }` |

**AEAD**

| Method | Params | Response |
|--------|--------|----------|
| `crypto.encrypt_chacha20_poly1305` / `crypto.chacha20_poly1305_encrypt` | `{ plaintext: base64, key: base64, nonce?: base64 }` | `{ ciphertext: base64, nonce: base64, tag: base64 }` |
| `crypto.decrypt_chacha20_poly1305` / `crypto.chacha20_poly1305_decrypt` | `{ ciphertext: base64, key: base64, nonce: base64, tag: base64 }` | `{ plaintext: base64 }` |
| `crypto.aead.aes_gcm_256.encrypt` | `{ plaintext: base64, key: base64, aad?: base64 }` | `{ ciphertext: base64, nonce: base64, tag: base64 }` |

**Hashing / HMAC**

| Method | Params | Response |
|--------|--------|----------|
| `crypto.hash` / `crypto.blake3_hash` | `{ data: base64 }` | `{ hash: hex }` |
| `crypto.hmac_sha256` | `{ data: base64, key: base64 }` | `{ mac: base64 }` |

**KDF / Password**

| Method | Params | Response |
|--------|--------|----------|
| `crypto.kdf.argon2id` | `{ password: string, salt?: base64 }` | `{ derived_key: base64, salt: base64 }` |
| `crypto.kdf.tls12_prf` | `{ secret: base64, label: string, seed: base64, length: u32 }` | `{ output: base64 }` |

---

### `genetic.*` — Multi-Tier Genetics (registered under CryptoHandler)

| Method | Params | Response |
|--------|--------|----------|
| `genetic.derive_lineage_seed` | `{ family_seed: base64 }` | `{ lineage_seed: base64 }` |
| `genetic.derive_key` | `{ lineage_seed: base64, context: string, generation?: u32 }` | `{ derived_key: hex }` |
| `genetic.generate_challenge` | `{ lineage_seed: base64 }` | `{ challenge: base64 }` |
| `genetic.verify_challenge` | `{ challenge: base64, response: base64, lineage_seed: base64 }` | `{ valid: bool }` |
| `genetic.generate_lineage_proof` | `{ lineage_seed: base64, statement: string }` | `{ proof: base64 }` |
| `genetic.verify_lineage_proof` | `{ proof: base64, lineage_seed: base64, statement: string }` | `{ valid: bool }` |
| `genetic.generate_lineage_certificate` | `{ lineage_seed: base64, subject: string }` | `{ certificate: base64 }` |
| `genetic.verify_lineage_certificate` | `{ certificate: base64, lineage_seed: base64 }` | `{ valid: bool, subject: string }` |
| `genetic.derive_device_seed` | `{ lineage_seed: base64, device_id: string }` | `{ device_seed: base64 }` |

**Encoding**: `family_seed` and `lineage_seed` are base64. Derived keys and proofs are hex.

---

### `btsp.*` — Binary Trusted Secure Protocol

| Method | Params | Response |
|--------|--------|----------|
| `btsp.contact.exchange` | `{ contact_info: object }` | `{ exchanged: bool, peer_contact: object }` |
| `btsp.tunnel.establish` | `{ peer_id: string, mode?: string }` | `{ tunnel_id: string, status: string }` |
| `btsp.tunnel.encrypt` | `{ tunnel_id: string, data: base64 }` | `{ ciphertext: base64 }` |
| `btsp.tunnel.decrypt` | `{ tunnel_id: string, ciphertext: base64 }` | `{ plaintext: base64 }` |
| `btsp.tunnel.status` | `{ tunnel_id: string }` | `{ status: string, metrics: object }` |
| `btsp.tunnel.close` | `{ tunnel_id: string }` | `{ closed: bool }` |
| `btsp.server.create_session` | `{ client_hello: base64 }` | `{ session_id: string, server_hello: base64 }` |
| `btsp.server.verify` | `{ session_id: string, proof: base64 }` | `{ verified: bool }` |
| `btsp.server.negotiate` | `{ session_id: string, params: object }` | `{ agreed: object }` |

**Legacy aliases**: Underscore variants (`btsp.contact_exchange`, etc.) and path-style
(`beardog./btsp/contact/exchange`) are accepted but deprecated.

---

### `security.*` / `trust.*` — Trust Evaluation

| Method | Params | Response |
|--------|--------|----------|
| `security.evaluate` / `trust.evaluate` | `{ peer_id: string, context?: object }` | `{ decision: string, trust_level: u8, trust_level_name: string, reason: string }` |
| `security.lineage` / `trust.lineage` | `{ peer_id?: string }` | `{ lineage: object }` |
| `security.verify_consent` | `{ token: string, scope: string }` | `{ valid: bool }` |
| `security.issue_consent_token` | `{ scope: string, duration_secs?: u64 }` | `{ token: string, expires_at: string }` |
| `birdsong.encrypt` | `{ data: base64, beacon_id: string }` | `{ ciphertext: base64 }` |
| `birdsong.decrypt` | `{ ciphertext: base64, beacon_id: string }` | `{ plaintext: base64 }` |

---

### `beacon.*` — MitoBeacon Management

| Method | Params | Response |
|--------|--------|----------|
| `beacon.generate` | `{ family_id?: string }` | `{ beacon_id: string, public_key: base64 }` |
| `beacon.get_id` | `{}` | `{ beacon_id: string }` |
| `beacon.encrypt` | `{ data: base64, target_beacon: string }` | `{ ciphertext: base64 }` |
| `beacon.try_decrypt` | `{ ciphertext: base64, beacon_id: string }` | `{ plaintext: base64, success: bool }` |
| `beacon.list_known` | `{}` | `{ beacons: [string] }` |
| `beacon.add_known` | `{ beacon_id: string, public_key: base64 }` | `{ added: bool }` |

---

### `health.*`

| Method | Response |
|--------|----------|
| `health.liveness` | `{ status: "alive", primal: "beardog", version: string }` |
| `health.readiness` | `{ status: "ready", protocol: "btsp", capabilities_count: u32 }` |
| `health.check` | `{ status: "healthy", version: string, timestamp: string }` |

---

### Ionic Bond Operations

| Method | Params | Response |
|--------|--------|----------|
| `crypto.ionic_bond.propose` | `{ terms: object, peer_id: string }` | `{ proposal_id: hex, status: string }` |
| `crypto.ionic_bond.accept` | `{ proposal_id: hex, signature: hex }` | `{ bond_id: hex, status: string }` |
| `crypto.ionic_bond.seal` | `{ bond_id: hex }` | `{ sealed: bool }` |
| `crypto.ionic_bond.verify` | `{ bond_id: hex }` | `{ valid: bool, terms: object }` |
| `crypto.ionic_bond.revoke` | `{ bond_id: hex }` | `{ revoked: bool }` |
| `crypto.ionic_bond.list` | `{}` | `{ bonds: [object] }` |

---

### Other Handlers

| Group | Key Methods |
|-------|-------------|
| **Encryption** | `encryption.encrypt`, `encryption.decrypt` |
| **Secrets** | `secrets.store`, `secrets.retrieve`, `secrets.list`, `secrets.delete` |
| **Federation** | `federation.verify_family_member`, `federation.derive_subfed_key` |
| **Graph Security** | `graph.validate_template`, `graph.audit_origin`, `graph.authorize_modification` |
| **Relay** | `relay.authorize` |
| **Introspection** | `rpc.methods`, `primal.info`, `primal.capabilities` |

---

## Downstream Usage Pattern

```rust
use ecoPrimal::genetics::rpc::GeneticsClient;

let client = GeneticsClient::connect_uds(socket_path).await?;
let seed = client.derive_lineage_seed(family_seed_b64).await?;
let key = client.derive_key(&seed, "session-auth", 0).await?;
```

For raw JSON-RPC, send `{ "jsonrpc": "2.0", "method": "crypto.sign", "params": { ... }, "id": 1 }` over the UDS/TCP channel.
