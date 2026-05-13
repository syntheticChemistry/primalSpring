# primalSpring v0.9.25 UniBin Eukaryotic Handoff · May 9, 2026

**Audience:** Downstream products, sibling spring teams, and projectNUCLEUS integrators absorbing the UniBin (eukaryotic) evolution.

**Purpose:** Practical guidance for adopting the single-binary model, the shared certification library, validation scenarios with tracks and tiers, and migration off deprecated harness/launcher/probe APIs.

---

## 1. What Changed

primalSpring evolved from a **prokaryotic** layout to a **eukaryotic** layout:

| Before (prokaryotic) | After (eukaryotic) |
|----------------------|---------------------|
| 89 individual experiment binaries + 3 standalone binaries (`primalspring_primal`, `primalspring_guidestone`, `validate_all`) | Single **`primalspring_unibin`** binary (exposed as `primalspring` in operational flows) |

**Certification:** The certification engine (L0–L8 guidestone layers) is absorbed into **`ecoPrimal/src/certification/`** as reusable library modules.

**Validation scenarios:** Twenty representative experiments are absorbed as validation scenario modules under **`ecoPrimal/src/validation/scenarios/`**.

**Registry and taxonomy:** A **ScenarioRegistry** provides a **track taxonomy (9 tracks)** and **tier filtering** (**Rust** / **Live** / **Both**).

**Deprecated surface:** Harness, launcher, and probe APIs that no longer reflect the eukaryotic model are **properly deprecated**, each with a clear **reason** in the deprecation attributes and docs so integrators know what to use instead.

---

## 2. UniBin CLI

Common entry points for certification, validation, IPC, and health:

```text
primalspring certify              # L0-L8 composition certification
primalspring certify --layer 3    # run up to layer 3
primalspring certify --bare       # L0 only, no primals needed
primalspring validate             # run all validation scenarios
primalspring validate --track atomic-composition
primalspring validate --scenario tower-atomic
primalspring validate --tier rust  # Tier 1 only (no IPC)
primalspring validate --tier live  # Tier 2 only (requires primals)
primalspring validate --list      # list all scenarios
primalspring serve                # JSON-RPC 2.0 IPC server
primalspring status               # composition health summary
primalspring version              # version info
```

Use **`certify`** before deployment to prove composition layering; use **`validate`** with **`--tier`** to scope CI vs live checks.

---

## 3. Two-Tier Validation Architecture

- **Tier 1 (Rust):** Pure structural validation, **no IPC**. Safe for default CI pipelines and fast feedback.
- **Tier 2 (Live):** Requires **deployed primals** from **plasmidBin** and exercises live composition behavior.

Filter with **`primalspring validate --tier rust`**, **`--tier live`**, or run broader sets as needed for your pipeline stage.

---

## 4. For Downstream Springs

**Absorbing this pattern**

1. Add parallel **`certification/`** and **`validation/scenarios/`** modules in your own crate(s), following the same separation: certification layers vs executable scenario modules.
2. Import **`primalspring::certification`** (and extend with domain-specific rules) for **composition certification on top** of shared L0–L8 semantics where applicable.
3. Use the **`ScenarioMeta`** provenance pattern so every scenario carries stable identity, track, tier, and lineage for registry listing and audit.
4. **Upgrade deprecated APIs** intentionally:
   - e.g. **`probe_primal`** → **`CompositionContext::health_check()`** (and related composition-first APIs).
   - Prefer **`CompositionContext`** for discovery and calls instead of ad-hoc probes.

---

## 5. For projectNUCLEUS

- **UniBin** is the **deployment-ready** binary for NUCLEUS compositions.
- **`primalspring certify`** validates **composition correctness** before deployment.
- **`primalspring validate --tier live`** validates **live** compositions against deployed primals.
- **Backward compatibility:** **`primalspring serve`** exposes the **same JSON-RPC methods** as prior IPC surfaces so existing automation can migrate incrementally.

---

## 6. Composition Patterns

- **CompositionContext** with **5-tier discovery** is the standard entry for resolving and calling primals in a composition.
- **Bearer token** threading via **`call_authenticated()`** for secured calls across the stack.
- **BTSP Phase 3 AEAD** applied consistently (**13/13 primals**).
- **biomeOS Neural API** for orchestration flows.

Teams should model new work on these patterns rather than resurrecting one-off binaries per experiment.

---

## 7. What Next (Next Stadial Wave)

- Absorb the remaining **~69** experiment scenarios into **`validation/scenarios/`** (or successor layout) under the same registry discipline.
- **Retire legacy binaries:** `primalspring_primal`, `primalspring_guidestone`, `validate_all`.
- Target end state: **full eukaryotic** — **one binary**, **one library**, **one fossil record** (traceable scenario and certification history).

---

## 8. Migration Checklist for Springs

Use this when bumping a spring or product repo onto UniBin semantics.

- [ ] Pin **primalSpring v0.9.25** (or the release tag that ships UniBin for your line).
- [ ] Replace **`probe_primal()`** → **`CompositionContext::health_check()`** (and composition-aware health).
- [ ] Replace **`PrimalClient::connect()`** → **`CompositionContext::call()`** (or the documented composition call path for your stack).
- [ ] Replace **AtomicHarness** → **plasmidBin + biomeOS** deployment for live-tier validation.
- [ ] Add **`#[allow(deprecated, reason = "...")]`** only where **short-term backward compatibility** is required; plan removal.
- [ ] Consider absorbing your own experiments into **`validation/scenarios/`** with **`ScenarioMeta`** and track/tier registration.

---

*Document version aligns with primalSpring v0.9.25 UniBin eukaryotic handoff · May 9, 2026.*
