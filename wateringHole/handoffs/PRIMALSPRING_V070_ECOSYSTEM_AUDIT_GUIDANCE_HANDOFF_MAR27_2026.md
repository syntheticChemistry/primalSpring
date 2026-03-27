# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring v0.7.0 — Ecosystem Audit Guidance Handoff

**Date**: March 27, 2026
**From**: primalSpring (coordination and composition validation spring)
**To**: All primal teams, all spring teams, biomeOS
**Phase**: 16.1 (Coverage Evolution + Deep Debt Audit complete)

---

## Context

primalSpring has completed a comprehensive deep audit and coverage evolution
across two sessions. The patterns that proved effective here are documented
below as **specific, actionable audit guidance** each primal and spring team
can review and enact independently to solve remaining deep debt.

### primalSpring Current State (verified March 27, 2026)

- 378 tests (0 failures, 42 ignored live), 72.5% library coverage
- 87/87 composition gates passing
- Zero TODOs, FIXMEs, HACKs in Rust source
- Zero `unsafe` (workspace `forbid`), zero `#[allow()]` in production
- Zero C dependencies (`deny.toml` enforced, ecoBin compliant)
- All files under 1000 LOC, all experiments with structured provenance
- All hardcoded primal names evolved to `primal_names::*` constants
- All timeouts/tolerances centralized in `tolerances/` module
- Zero clippy warnings (pedantic + nursery + cast discipline)
- Zero `cargo deny` policy violations

---

## Patterns for Ecosystem Absorption

### 1. Centralized Tolerances Module (HIGH PRIORITY — all primals)

**Problem**: Scattered `Duration::from_secs(5)`, `1000` ms, `<= 1`
magic numbers across IPC, launcher, and validation code. Each is a
silent policy decision that drifts independently.

**Solution**: Single `tolerances/` module with `pub const` entries.
Each constant has doc comments with:
- **Source**: where the value came from
- **Validated**: when it was operationally confirmed
- **Provenance**: which phase/session established it

**Pattern**:
```rust
/// IPC socket read/write timeout.
///
/// Source: biomeOS IPC convention (5s default for local sockets).
/// Validated: Phase 16 operational measurement (Mar 2026).
pub const IPC_SOCKET_TIMEOUT_SECS: u64 = 5;
```

**Audit check**: `rg 'Duration::from_secs\(' src/ | grep -v tolerances`
— any hits are candidates for centralization.

---

### 2. Primal Names Module (HIGH PRIORITY — all springs)

**Problem**: String literals like `"beardog"`, `"toadstool"` scattered
across experiments, tests, and IPC code. One typo = silent discovery failure.

**Solution**: `primal_names.rs` with `pub const` slug and display entries:
```rust
pub const BEARDOG: &str = "beardog";
pub const TOADSTOOL: &str = "toadstool";
```

**Rules**:
- Function calls: `discover_primal(primal_names::BEARDOG)`
- Comparisons: `if name == primal_names::SONGBIRD`
- JSON wire format strings stay as literals (wire protocol, not identity)
- Test fixture data stays as literals (test-only, not production routing)

**Audit check**: `rg '"beardog"|"songbird"|"toadstool"|"squirrel"|"nestgate"' src/ --glob '!*/tests/*'`

---

### 3. Capability Registry Sync Test (HIGH PRIORITY — all primals with niche)

**Problem**: TOML registry and code capability list drift apart silently.

**Solution**: Test that `include_str!`s the TOML and asserts exact parity:
```rust
#[test]
fn capabilities_match_registry_toml() {
    let toml_str = include_str!("../../config/capability_registry.toml");
    let parsed: toml::Value = toml::from_str(toml_str).unwrap();
    let caps_in_toml: Vec<&str> = parsed["capabilities"]
        .as_array().unwrap()
        .iter().filter_map(|c| c.get("method")?.as_str())
        .collect();
    for code_cap in CAPABILITIES {
        assert!(caps_in_toml.contains(code_cap));
    }
    for toml_cap in &caps_in_toml {
        assert!(CAPABILITIES.contains(toml_cap));
    }
}
```

---

### 4. `#![forbid(unsafe_code)]` + `deny.toml` (ALL primals)

**Audit check**:
- Crate root: `#![forbid(unsafe_code)]`
- Workspace lints: `[workspace.lints.rust] unsafe_code = "forbid"`
- `deny.toml` with banned C-linking crates (aws-lc-sys, cmake, cc, etc.)
- Run `cargo deny check` — zero advisories, bans, license, source issues

---

### 5. Coverage Saturation Analysis (ALL primals)

Don't chase 100% — identify the **coverage ceiling** for each module:

| Module type | Testable offline? | Strategy |
|-------------|-------------------|----------|
| Pure logic (parsing, validation, types) | Yes | Target 95%+ |
| IPC clients (socket I/O, timeouts) | Partial | Mock server in tests, accept 60-70% |
| Process launchers (spawn, wait) | No | Test error paths, accept 30-40% |
| Binary entrypoints (main.rs) | No | Not testable via `cargo test` |

**primalSpring's measured ceilings** (for reference):
- tolerances, emergent, graphs, protocol, dispatch, error: 100%
- coordination: 83% (live primals needed for rest)
- niche: 73% (registration needs live biomeOS)
- launcher: 32% (process spawn is integration-only)
- bin/*: 0% (binary entrypoints)

---

### 6. Tick Slack / Magic Number Audit (springs with timing)

**Audit check**: `rg '<= 1|<= 2|abs_diff.*1\b' src/`

Any inline tolerance comparison should be replaced with a named constant.

---

## Per-Primal Deep Debt Audit Guidance

### BearDog (Cryptography)

**Current**: Production Ready (A+ LEGENDARY, 99/100), 72 JSON-RPC methods

**Audit items**:
1. **Tolerance centralization**: Audit crypto operation timeouts. If any
   `Duration::from_*()` exists outside a tolerances module, centralize.
2. **Capability registry sync**: If niche capabilities exist in code AND
   a TOML/YAML config, add a sync test.
3. **`primal_names` absorption**: If BearDog references other primals by
   string literal in production code (not tests), migrate to constants.
4. **Coverage measurement**: Run `cargo llvm-cov` and identify your ceiling.
   Crypto modules should hit 95%+. Socket/IPC paths will be lower.
5. **`deny.toml` freshness**: Prune `allow` list to licenses actually in
   your resolved dependency graph.

---

### Songbird (Network)

**Current**: Production Ready (S+, 9,969 tests, ~72% coverage, 30 crates)

**Audit items**:
1. **Tolerance centralization**: Network timeouts (STUN, Tor, federation,
   NAT traversal) are prime candidates. Ensure each has Source/Validated
   provenance in docs.
2. **`primal_names` absorption**: Songbird discovers BearDog and other
   primals — verify all discovery calls use centralized constants.
3. **File size audit**: 30 crates means many files. Verify all under
   1000 LOC. Smart-refactor (extract logic, not just split) any over.
4. **Unsafe audit**: Pure Rust Tor likely has `unsafe` in crypto paths.
   Audit each `unsafe` block: can it be replaced with safe Rust at
   equivalent performance? Document any that must remain.
5. **ecoBin compliance**: Verify zero C dependencies in the final binary
   link (not just in Cargo.toml — check `ldd` output on musl builds).

---

### ToadStool (Compute)

**Current**: Production Ready

**Audit items**:
1. **Dual protocol evolution**: ToadStool has tarpc + JSON-RPC. Ensure
   the JSON-RPC path has full error typing (IpcError pattern, not string
   errors). Verify `normalize_method()` is wired for dispatch.
2. **`primal_names` absorption**: ToadStool references barraCuda and
   coralReef by name — centralize slug constants.
3. **Tolerance centralization**: GPU dispatch timeouts, health check
   intervals, shader compilation timeout — all should be named constants.
4. **Coverage**: Focus on the JSON-RPC handler paths. The tarpc paths
   are likely well-covered. Measure and report the ceiling.

---

### NestGate (Storage)

**Current**: Production Ready

**Audit items**:
1. **ZFS fallback coverage**: The filesystem fallback path when ZFS is
   absent should be thoroughly tested. This is a critical degradation path.
2. **Tolerance centralization**: Storage operation timeouts, replication
   intervals, compaction thresholds — named constants with provenance.
3. **Data provenance**: Verify all stored data has traceable provenance
   (accession numbers for public data, commit hashes for generated data).
4. **`primal_names` absorption**: Storage APIs that reference other
   primals for replication targets should use constants.

---

### Squirrel (AI)

**Current**: Alpha, Anthropic Claude integration

**Audit items**:
1. **API key handling**: Verify no API keys in code, tests, or committed
   config. `.gitignore` should block `*.key`, `credentials.json`, etc.
2. **Timeout centralization**: AI inference timeouts are highly variable.
   Document the expected range and centralize the default.
3. **Graceful degradation**: When the AI provider is unreachable, verify
   the primal degrades to a useful error (not a panic or hang).
4. **MCP tool definitions**: Verify tool schemas match actual handler
   signatures. Add a sync test if not present.
5. **Coverage**: AI client paths will have low coverage (need live API).
   Focus on error handling paths and response parsing.

---

### biomeOS (Orchestration)

**Current**: Neural API operational, graph executor live

**Audit items**:
1. **Graph executor coverage**: biomeOS graph execution is the composition
   engine. Verify structural tests exist for all graph patterns (Sequential,
   Parallel, ConditionalDag, Pipeline, Continuous, PathwayLearner).
2. **Enrollment flow**: Neural API enrollment should have a test that
   exercises the full register → discover → call cycle offline.
3. **Tolerance centralization**: Graph startup timeouts, primal health
   poll intervals, Neural API response deadlines — all named constants.
4. **`normalize_method()` adoption**: Verify all dispatch paths use
   `normalize_method()` for prefix-agnostic routing (no raw string matching).
5. **Socket path convention**: Verify `$XDG_RUNTIME_DIR/biomeos/` is
   the canonical socket directory, with `/tmp/biomeos/` fallback.

---

### Provenance Trio (sweetGrass, rhizoCrypt, loamSpine)

**Current**: Structural readiness

**Audit items**:
1. **Circuit breaker**: primalSpring's `ipc/provenance.rs` uses an
   epoch-based circuit breaker with exponential backoff for trio calls.
   Trio primals should implement complementary rate limiting.
2. **Type alignment**: Verify `begin_session`, `record_step`,
   `complete_experiment` request/response types match across all three.
3. **Capability registration**: Each trio primal should register its
   capabilities with biomeOS on startup.
4. **Offline testing**: Add tests that exercise the full provenance
   pipeline with mock responses (no live trio needed).

---

### coralReef (Sovereign Shader Compiler)

**Current**: Phase 10 Iteration 59 (3038 tests, 65.8% coverage)

**Audit items**:
1. **Coverage push**: 65.8% → 75%+. Focus on encoder paths and error
   handling. Hardware-dependent paths can stay at current coverage.
2. **`primal_names` absorption**: If coralReef references other primals
   (toadStool, barraCuda) in production code, use constants.
3. **File size audit**: Verify all files under 1000 LOC.
4. **`deny.toml` enforcement**: Ensure ecoBin compliance — zero C
   dependencies in the shader compiler itself.

---

### barraCuda (Math Library)

**Current**: v0.3.10 (806 WGSL shaders, wgpu 28)

**Audit items**:
1. **FMA policy documentation**: Ensure `FmaPolicy` and precision tier
   documentation has provenance (which GPU, which test, which date).
2. **Shader coverage**: 806 shaders — verify each has at least one
   CPU-GPU parity test.
3. **Resilient GPU tests**: Verify GPU test pool delegation works when
   no GPU is available (graceful skip, not panic).
4. **`deny.toml`**: Ensure wgpu's transitive deps don't pull in
   banned C crates.

---

## Per-Spring Deep Debt Audit Guidance

### All Springs (groundSpring, wetSpring, airSpring, hotSpring, etc.)

**Common audit items**:
1. **Coverage gate**: Add `fail-under-lines` to CI (airSpring: 90%,
   primalSpring: 70%). Each spring should set a floor that ratchets up.
2. **Tolerance module**: Every spring with IPC, timing, or numeric
   validation should have a `tolerances/` module. No inline magic numbers.
3. **`primal_names` constants**: Every spring that discovers primals at
   runtime should use centralized slug constants.
4. **`#![forbid(unsafe_code)]`**: Application-layer crate roots.
5. **`deny.toml`**: Prune unused license allowances. Ban C-linking crates.
6. **`validate_release.sh`**: Each spring should have a release gate script
   that checks fmt + clippy + deny + test floor + docs.
7. **Handoff naming**: `{SPRING}_{VERSION}_{TOPIC}_HANDOFF_{DATE}.md`
8. **CONTRIBUTING.md + SECURITY.md**: Every spring should have both.
9. **Structured provenance**: `with_provenance(source, date)` on every
   experiment/validation binary.
10. **Builder `.run()` pattern**: `ValidationResult::new(title).with_provenance(...).run(sub, |v| { ... })`
    eliminates boilerplate in experiment binaries.

### hotSpring (Physics)

- 87 experiments — verify all use `primal_names::*` and `ValidationResult` builder
- GPU shaders: map each local WGSL to barraCuda absorption status
- Layer 8/9 blocker documentation should have provenance dates

### wetSpring (Biology)

- 1,902 tests at 91.20% — excellent. Audit for any remaining hardcoded
  primal names in the 355 binaries.
- 234 named tolerances — model for other springs to follow.

### groundSpring (Earth Sciences)

- 1020+ tests at ≥92% — model coverage. Verify tolerance provenance
  on all 110 delegations (67 CPU + 43 GPU).

### airSpring (Agriculture)

- 90.56% coverage gated at 90 — excellent. Verify `default-features = false`
  on barraCuda dep (no unnecessary GPU compilation in CI).

### ludoSpring (Game Science)

- 91.27% coverage — excellent. Verify 42 Python parity tests still
  reproducible (rerun and confirm no baseline drift).

### neuralSpring (Neural/ML)

- Upstream contract pinning: verify tolerance invariant tests still pass
  against latest barraCuda.

---

## plasmidBin Integration Path

The next evolution tier for all primals:

1. **Binary harvesting**: Each primal builds musl static binaries and
   deposits them in `plasmidBin/primals/{name}/`
2. **primalSpring integration tests**: The 42 `--ignored` tests run
   against plasmidBin artifacts — this is the live composition CI tier
3. **Deploy graph validation**: primalSpring validates that plasmidBin
   binaries compose correctly via biomeOS graphs
4. **Release gate**: `validate_release.sh` + `validate_remote_gate.sh`
   form the quality gate before deployment

---

## What primalSpring Learned (for ecosystem absorption)

### Zero-TODO Codebase
After audit: zero TODO, FIXME, HACK, XXX in Rust source. Debt is
tracked in specs/ and handoffs, not inline. Inline debt markers rot.

### Coverage Ceiling Is Honest
Don't fake coverage with trivial tests on untestable paths. Identify
the ceiling per module type, test what's offline-testable to saturation,
and document the rest as "needs live ecosystem CI tier."

### Test Count Floor Ratchets
`validate_release.sh` has `MIN_TESTS=378`. It only goes up. This
prevents silent test deletion or exclusion.

### Hardcoding Is a Spectrum
Wire protocol strings ("beardog" in JSON) are not hardcoding — they're
the wire format. But `discover_primal("beardog")` is identity coupling.
The fix is `primal_names::BEARDOG` for the constant, and
`discover_by_capability("security")` for the loose coupling path.

### Graceful Degradation > Hard Failure
Every primal probe in primalSpring degrades gracefully when the target
is absent. `check_skip()` and `check_or_skip()` produce honest SKIP
results instead of fake PASS or hard FAIL. This pattern should be
universal across all springs.

---

**License**: AGPL-3.0-or-later
