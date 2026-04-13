# Niche Starter Patterns — From NUCLEUS to Domain Composition

**Date**: April 12, 2026
**Owner**: primalSpring
**For**: All downstream springs beginning composition evolution
**License**: AGPL-3.0-or-later

---

## How Springs Evolve

```
Research Paper → Python Baseline → Rust Validation → Primal Composition
```

Every spring has completed Python → Rust. The next layer is Rust → Primal
Composition: proving that the same science works when composed from NUCLEUS
primals rather than local Rust math.

---

## Step 1: Common NUCLEUS Base (All Springs)

Every spring starts with this experiment pattern. Copy `exp094_composition_parity`
and adapt:

```rust
use primalspring::composition::{CompositionContext, validate_parity};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("mySpring — NUCLEUS Composition Parity")
        .with_provenance("myspring_exp_composition", "2026-04-12")
        .run("mySpring: NUCLEUS parity", |v| {
            let mut ctx = CompositionContext::from_live_discovery();

            // Tower: verify trust boundary is alive
            v.section("Tower");
            match ctx.call("security", "health.liveness", serde_json::json!({})) {
                Ok(r) => v.check_bool("tower_alive", true, &format!("{r}")),
                Err(e) if e.is_connection_error() => v.check_skip("tower_alive", &format!("{e}")),
                Err(e) => v.check_bool("tower_alive", false, &format!("{e}")),
            }

            // YOUR NICHE HERE — see Step 2 examples below
            v.section("Niche Composition");
            niche_parity(&mut ctx, v);
        });
}
```

---

## Step 2: Niche-Specific Patterns

### hotSpring — GPU Math Parity (Physics)

hotSpring validates that barraCuda's WGSL shaders reproduce Python NumPy/SciPy
results for lattice QCD and HPC physics.

```rust
fn niche_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    // Python: np.dot([1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0], ...) = trace
    // SU(3) matrix trace via barraCuda tensor.matmul
    validate_parity(
        ctx, v,
        "su3_trace_identity",
        "tensor",
        "tensor.matmul",
        serde_json::json!({
            "a": {"data": [1,0,0, 0,1,0, 0,0,1], "shape": [3,3]},
            "b": {"data": [1,0,0, 0,1,0, 0,0,1], "shape": [3,3]}
        }),
        "trace",
        3.0,  // identity matrix trace
        tolerances::EXACT_PARITY_TOL,
    );

    // df64 double precision: critical for QCD where f32 insufficient
    validate_parity(
        ctx, v,
        "df64_precision_sum",
        "tensor",
        "tensor.df64.sum",
        serde_json::json!({"data": [1e-15, 2e-15, 3e-15]}),
        "value",
        6e-15,
        tolerances::DOUBLE_EMULATION_TOL,
    );
}
```

**Graph**: `hotspring_qcd_proto_nucleate.toml`
**Key capability**: `tensor` (barraCuda), `shader` (coralReef for custom WGSL)
**What to hand back**: df64 precision requirements, multi-GPU dispatch patterns

---

### neuralSpring — AI/ML Composition

neuralSpring validates that WGSL shader compositions reproduce PyTorch/JAX
inference results for tokenization, attention, and model execution.

```rust
fn niche_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    // Shader compilation for attention kernel
    match ctx.call(
        "shader",
        "shader.compile.wgsl",
        serde_json::json!({"source": ATTENTION_WGSL, "entry_point": "main"}),
    ) {
        Ok(r) => {
            let compiled = r.get("status")
                .and_then(|s| s.as_str())
                .is_some_and(|s| s == "compiled");
            v.check_bool("attention_shader_compiles", compiled, &format!("{r}"));
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("attention_shader_compiles", &format!("{e}"));
        }
        Err(e) => v.check_bool("attention_shader_compiles", false, &format!("{e}")),
    }

    // Softmax via barraCuda tensor ops
    // Python: scipy.special.softmax([1.0, 2.0, 3.0]) = [0.0900, 0.2447, 0.6652]
    validate_parity_vec(
        ctx, v,
        "softmax_3elem",
        "tensor",
        "tensor.softmax",
        serde_json::json!({"data": [1.0, 2.0, 3.0]}),
        "result",
        &[0.0900, 0.2447, 0.6652],
        tolerances::CPU_GPU_PARITY_TOL,
    );
}
```

**Graph**: `neuralspring_inference_proto_nucleate.toml`
**Key capability**: `shader` (coralReef), `tensor` (barraCuda), `ai` (Squirrel)
**What to hand back**: tokenization patterns, attention kernel requirements

---

### healthSpring — Encrypted Data Composition

healthSpring validates bonding model compositions for encrypted health data
enclaves where compute never sees plaintext.

```rust
fn niche_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    // Encrypt patient data via BearDog
    match ctx.call(
        "security",
        "crypto.encrypt",
        serde_json::json!({"plaintext": "patient_vitals_2026", "key_id": "health_enclave_01"}),
    ) {
        Ok(r) => {
            let ciphertext = r.get("ciphertext").and_then(|c| c.as_str()).unwrap_or("");
            v.check_bool("encrypt_nonempty", !ciphertext.is_empty(), "ciphertext produced");

            // Store encrypted data via NestGate
            let _ = ctx.call(
                "storage",
                "storage.put",
                serde_json::json!({"key": "health_001", "value": ciphertext}),
            );
            // Retrieve and verify round-trip
            // The decryption key never leaves the security primal
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("encrypt_nonempty", &format!("{e}"));
        }
        Err(e) => v.check_bool("encrypt_nonempty", false, &format!("{e}")),
    }
}
```

**Graph**: `healthspring_enclave_proto_nucleate.toml`
**Key capability**: `security` (BearDog), `storage` (NestGate), `provenance` (trio)
**What to hand back**: bonding model requirements, multi-family encryption patterns

---

### wetSpring — Life Science Pipelines

wetSpring validates genomics/proteomics pipeline compositions using
barraCuda's FFT and tensor ops for sequence analysis.

```rust
fn niche_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    // FFT for spectral analysis of sequence data
    // Python: np.fft.fft([1.0, 0.0, 1.0, 0.0]) = [2+0j, 0+0j, 2+0j, 0+0j]
    validate_parity_vec(
        ctx, v,
        "fft_4elem_real",
        "tensor",
        "tensor.fft",
        serde_json::json!({"data": [1.0, 0.0, 1.0, 0.0]}),
        "magnitudes",
        &[2.0, 0.0, 2.0, 0.0],
        tolerances::FFT_PARITY_TOL,
    );
}
```

**Graph**: `wetspring_lifescience_proto_nucleate.toml`
**Key capability**: `tensor` (barraCuda FFT), `storage` (NestGate for sequence data)
**What to hand back**: FFT precision requirements, large-tensor streaming patterns

---

## Step 3: Hand Back to primalSpring

After evolving your niche composition, hand back:

1. **Gaps discovered** — missing IPC methods, precision issues, dispatch patterns
2. **Tolerance findings** — which tolerances work for your domain
3. **New patterns** — any composition patterns other springs could use
4. **Wire contract needs** — what response schemas your domain requires

Create a handoff at:
```
ecoPrimals/infra/wateringHole/handoffs/{SPRING}_v{VERSION}_COMPOSITION_GAPS_HANDOFF_{DATE}.md
```

primalSpring absorbs these into `PRIMAL_GAPS.md` and passes to upstream primals.
The water cycle continues.
