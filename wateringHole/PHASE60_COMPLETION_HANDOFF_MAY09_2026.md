# Phase 60 Completion Handoff — Downstream Products & Spring Teams

**From**: primalSpring v0.9.25 (syntheticChemistry)
**Date**: May 9, 2026
**To**: Spring teams (delta), downstream products (projectNUCLEUS, esotericWebb, sporeGarden), primal consumers

---

## Headline

**13/13 primals at zero upstream debt.** All Tier 1-3 gaps from the Phase 60
audit are resolved. Every primal has adopted JH-0 MethodGate, BTSP Phase 3 AEAD,
and Capability Wire Standard L2+. The ecosystem is ready for interstadial
feature evolution.

---

## 1. What Resolved Since Phase 59

| Gap | What | Who Fixed | Impact |
|-----|------|-----------|--------|
| DF-2 | toadStool `auth.mode` env var hardcoded to permissive | toadStool (S233) + primalSpring (initial fix) | Enforced mode now works |
| DF-3 | songbird/squirrel TCP `auth.mode` | songbird (transport-aware CallerContext), squirrel (documented intentional UDS-only) | TCP dispatch now classified |
| U1 | Stale `validation/CHECKSUMS` | primalSpring | Integrity validation passes |
| U2 | Missing `by_capability` on operation nodes | primalSpring (3 graphs) | guidestone structural check passes |
| U3 | Missing `bonding_policy` on profile graphs | primalSpring (34 graphs) | guidestone structural warning resolved |
| P4 | biomeOS test helpers in production | biomeOS (v3.49) | Zero mock code in release builds |
| P6 | petalTongue bare `#[allow]` | petalTongue (97 files) | All lint suppression documented |
| P7 | squirrel 1,105-line test file | squirrel (split to 3 modules) | Maintainability |
| Tier 3 | coralReef `eprintln!` → `tracing` | coralReef (Iter 95) | Structured logging in multi-primal deployment |
| Tier 3 | bearDog HSM mock feature gate | bearDog (Wave 98) | Defense-in-depth `#[cfg(test)]` |
| Tier 3 | barraCuda `unwrap()` | barraCuda (false positive confirmed) | No production unwrap |
| Tier 3 | nestgate `unwrap()` | nestgate (NG-13 false positive confirmed) | No production unwrap |
| JH-2 | Resource envelope enforcement | biomeOS (v3.48) + toadStool (S232) | `timeout_ms` enforced on forwarding |
| JH-11 | Cross-primal token federation | deferred (ecosystem-wide) | biomeOS `_resource_envelope` workaround |

---

## 2. Composition Patterns for NUCLEUS Deployment via Neural API

### How springs deploy through biomeOS

```
Spring binary (guidestone)
  └─ CompositionContext::discover()    ← 5-tier escalation
       └─ biomeOS Neural API           ← semantic routing
            └─ capability.call(domain, method, params)
                 └─ primal UDS socket  ← MethodGate pre-dispatch
```

**Discovery escalation (tier order)**:
1. Songbird routing (`ipc.resolve`)
2. Neural API (`capability.call`)
3. UDS convention (`$XDG_RUNTIME_DIR/biomeos/{primal}-{fid}.sock`)
4. Socket registry scan
5. TCP probing (opt-in, covalent mesh only)

### Deploy graph structure

Every NUCLEUS deployment is a TOML deploy graph:

```toml
[graph.metadata]
name = "my_deployment"
composition_model = "nucleated"  # pure | nucleated | validation

[graph.bonding_policy]
bond_type = "Ionic"              # Ionic | Covalent | Metallic | Weak
trust_model = "MethodGate"

[[graph.nodes]]
name = "beardog"
role = "tower"
capabilities = ["crypto", "security", "auth"]

[[graph.nodes]]
name = "my_spring_app"
role = "consumer"
capabilities = ["my_domain"]
```

### MethodGate integration for spring consumers

Springs consuming primals through biomeOS don't need to implement MethodGate
themselves — biomeOS validates tokens on the forwarding path. However, springs
that expose their own UDS endpoints (e.g., guidestone binaries registered via
`ipc.register`) SHOULD implement MethodGate for direct peer access.

Pattern for spring guidestone binaries:

```rust
// In your spring's guidestone binary:
let gate = match std::env::var("MY_SPRING_AUTH_MODE").as_deref() {
    Ok("enforced" | "enforcing") => MethodGate::new(GateMode::Enforcing),
    _ => MethodGate::permissive(),
};
// Pre-dispatch: gate.check(method, caller_context)?;
```

---

## 3. Per-Spring Absorption Targets

### Universal targets (all 7 delta springs)

1. **`barracuda` optional dep**: `optional = true, default-features = false` in Cargo.toml
2. **Registry cross-sync CI**: test local method strings against primalSpring canonical 389
3. **primalSpring v0.9.25 pin**: guidestone requires v0.9.25 for `CompositionContext` and `validate_parity`
4. **Zero `#[allow()]` without reason**: use `#[expect(..., reason = "...")]`

### Per-spring evolution notes

| Spring | Key Evolution Target | Details |
|--------|---------------------|---------|
| hotSpring | Experiment convention | Move exp bins from `src/bin/` to `experiments/expNNN_*/` crates |
| healthSpring | primalSpring pin v0.9.25 | guidestone pinned to v0.9.17 — upgrade |
| wetSpring | L5 guidestone | Currently L4 — needs full primal proof with NUCLEUS cross-atomic |
| neuralSpring | L4-L5 guidestone | Currently L3 — biggest gap relative to peers |
| ludoSpring | Notebooks | Zero `.ipynb` — convert Python baselines to notebooks for sporePrint |
| groundSpring | Default-features flip | `barracuda` is `optional = true, default = true` → flip to `default = false` |
| airSpring | Workspace `deny.toml` | Only sub-crate deny files; add root-level; ban `aws-lc-sys` |

---

## 4. For Downstream Products

### projectNUCLEUS

- **265 PASS, 0 FAIL, 0 KNOWN_GAP** — Phase 60 validation complete
- All upstream gaps from the consolidated handoff are resolved
- JH-0 enforced across 13/13 primals — multi-user security gate is real
- JH-2 resource envelopes enforced by biomeOS + toadStool dispatch paths
- Gap registry reduced from 2,290 lines to 142 lines (13/13 clean)

### esotericWebb

- Deploy as pure composition via biomeOS graph execution
- Use `CompositionContext::discover()` for primal discovery
- No binary in plasmidBin — the graph IS the product
- ludoSpring `game.*` methods + petalTongue rendering + Squirrel AI compose the full experience

### sporeGarden / primals.eco

- sporePrint auto-sync pipeline operational (`auto-refresh.yml`)
- `SPOREPRINT_REFRESH_PAT` handles both source cloning and PR creation
- Notebook rendering: `render_notebooks.sh` with recursive glob
- Lab content pages: validation summaries + rendered notebooks per spring

---

## 5. primalSpring Quality Gate (as-of May 9, 2026)

| Metric | Value |
|--------|-------|
| Tests | 666 (618 passed + 48 ignored, 0 failed) |
| Experiments | 85 (19 tracks) |
| Deploy graphs | 74 |
| Capability methods | 389 (canonical registry) |
| Clippy warnings | 0 |
| `cargo fmt` violations | 0 |
| `cargo deny` | all pass |
| `unsafe` blocks | 0 |
| `#[allow()]` without reason | 0 |
| TODO/FIXME/HACK/DEBT markers | 0 |
| Upstream primal debt | 0 (13/13 clean) |

---

## 6. Key Files for Absorption

| What | Path |
|------|------|
| Capability registry | `config/capability_registry.toml` |
| MethodGate standard | `wateringHole/METHOD_GATE_STANDARD.md` |
| Gap registry (living) | `docs/PRIMAL_GAPS.md` |
| Cross-spring scorecard | `docs/CROSS_SPRING_PARITY_SCORECARD.md` |
| Evolution handoff | `wateringHole/PRIMALSPRING_V0925_EVOLUTION_HANDOFF_MAY09_2026.md` |
| Composition library | `tools/nucleus_composition_lib.sh` |
| NUCLEUS launcher | `tools/composition_nucleus.sh` |
| Binary fetch | `tools/fetch_primals.sh` |
| Crypto bootstrap | `tools/nucleus_crypto_bootstrap.sh` |
