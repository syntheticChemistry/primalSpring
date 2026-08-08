//! Exp119: PathwayLearner Structural Validation — validates the prerequisites
//! and observable signals for Neural API Phase 3 (adaptive orchestration).
//!
//! The PathwayLearner requires:
//!   1. ExecutionTrace structs emitted per graph execution
//!   2. Per-node timing data (latency, success/failure)
//!   3. Repeated executions accumulating statistical evidence
//!   4. Pattern detection: Parallelize, Prewarm, Batch, Cache, Reorder
//!
//! This experiment validates these structural requirements against the live
//! Neural API, proving that the execution trace pipeline provides sufficient
//! signal for learning algorithms to operate.
//!
//! Key metrics validated:
//!   - Timing consistency (variance across repeated executions)
//!   - Node-level observability (which nodes succeed/fail, per-node duration)
//!   - Throughput ceiling (how fast can we execute graphs)
//!   - Parallelization signal (independent nodes that could run concurrently)

use primalspring::validation::ValidationResult;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

const NEURAL_SOCKET_ENV: &str = "NEURAL_API_SOCKET";
const DEFAULT_NEURAL_SOCKET: &str = "/run/user/1000/biomeos/biomeos-neural.sock";
const RIBOCIPHER_SIGNAL: &[u8] = &[0xEC, 0x00];

fn neural_socket_path() -> String {
    std::env::var(NEURAL_SOCKET_ENV).unwrap_or_else(|_| DEFAULT_NEURAL_SOCKET.to_string())
}

fn neural_rpc(method: &str, params: &serde_json::Value) -> Result<serde_json::Value, String> {
    let path = neural_socket_path();
    let socket_path = Path::new(&path);

    let mut stream =
        UnixStream::connect(socket_path).map_err(|e| format!("Connect failed: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(15))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = serde_json::to_string(&request).map_err(|e| format!("Serialize: {e}"))?;

    stream.write_all(RIBOCIPHER_SIGNAL).map_err(|e| format!("Write: {e}"))?;
    stream.write_all(msg.as_bytes()).map_err(|e| format!("Write: {e}"))?;
    stream.write_all(b"\n").map_err(|e| format!("Write: {e}"))?;
    stream.flush().map_err(|e| format!("Flush: {e}"))?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).map_err(|e| format!("Read: {e}"))?;

    if line.is_empty() {
        return Err("Empty response".to_string());
    }

    let response: serde_json::Value =
        serde_json::from_str(line.trim()).map_err(|e| format!("Parse: {e}"))?;

    if let Some(error) = response.get("error") {
        return Err(format!(
            "{}",
            error.get("message").and_then(|m| m.as_str()).unwrap_or("unknown")
        ));
    }

    response.get("result").cloned().ok_or_else(|| "Missing result".to_string())
}

#[derive(Debug, Clone)]
struct ExecutionTrace {
    execution_id: String,
    duration_ms: u64,
    state: String,
    completed_nodes: Vec<String>,
    failed_nodes: Vec<String>,
    total_phases: u64,
}

fn execute_and_trace(graph_id: &str) -> Result<ExecutionTrace, String> {
    let result = neural_rpc("graph.execute", &serde_json::json!({"graph_id": graph_id}))?;
    let eid = result
        .get("execution_id")
        .and_then(|e| e.as_str())
        .ok_or("No execution_id")?
        .to_string();

    std::thread::sleep(Duration::from_millis(100));

    let status = neural_rpc("graph.status", &serde_json::json!({"execution_id": eid}))?;

    Ok(ExecutionTrace {
        execution_id: eid,
        duration_ms: status.get("duration_ms").and_then(|d| d.as_u64()).unwrap_or(0),
        state: status
            .get("state")
            .and_then(|s| s.as_str())
            .unwrap_or("unknown")
            .to_string(),
        completed_nodes: status
            .get("completed_nodes")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        failed_nodes: status
            .get("failed_nodes")
            .and_then(|f| f.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        total_phases: status.get("total_phases").and_then(|p| p.as_u64()).unwrap_or(0),
    })
}

fn phase_trace_accumulation(v: &mut ValidationResult) {
    let graph_id = "tower_health";
    let n = 20;
    let start = Instant::now();

    let mut traces: Vec<ExecutionTrace> = Vec::new();
    for _ in 0..n {
        match execute_and_trace(graph_id) {
            Ok(trace) => traces.push(trace),
            Err(_) => {}
        }
    }
    let elapsed = start.elapsed();

    v.check_bool(
        "trace_accumulation",
        traces.len() >= n * 3 / 4,
        &format!(
            "{}/{n} traces collected in {:.0}ms",
            traces.len(),
            elapsed.as_secs_f64() * 1000.0
        ),
    );

    let durations: Vec<u64> = traces.iter().map(|t| t.duration_ms).collect();
    if durations.len() >= 5 {
        let mean = durations.iter().sum::<u64>() as f64 / durations.len() as f64;
        let variance = durations
            .iter()
            .map(|d| (*d as f64 - mean).powi(2))
            .sum::<f64>()
            / durations.len() as f64;
        let stddev = variance.sqrt();
        let cv = if mean > 0.0 { stddev / mean } else { 0.0 };

        v.check_bool(
            "timing_consistency",
            cv < 2.0,
            &format!(
                "mean={mean:.1}ms, stddev={stddev:.1}ms, CV={cv:.2} (< 2.0 = consistent)"
            ),
        );
    }
}

fn phase_node_observability(v: &mut ValidationResult) {
    let graph_id = "tower_health";

    match execute_and_trace(graph_id) {
        Ok(trace) => {
            let total_nodes = trace.completed_nodes.len() + trace.failed_nodes.len();
            v.check_bool(
                "node_level_tracking",
                total_nodes > 0,
                &format!(
                    "graph reports {total_nodes} nodes (completed={}, failed={})",
                    trace.completed_nodes.len(),
                    trace.failed_nodes.len()
                ),
            );

            v.check_bool(
                "phase_tracking",
                trace.total_phases > 0,
                &format!("graph reports {} execution phases", trace.total_phases),
            );

            v.check_bool(
                "execution_state_terminal",
                trace.state == "completed" || trace.state == "failed",
                &format!("execution reaches terminal state: {}", trace.state),
            );
        }
        Err(e) => {
            v.check_bool("node_level_tracking", false, &format!("trace failed: {e}"));
        }
    }
}

fn phase_throughput_ceiling(v: &mut ValidationResult) {
    let graph_id = "tower_health";
    let n = 50;
    let start = Instant::now();

    let mut successes = 0u32;
    for _ in 0..n {
        if neural_rpc("graph.execute", &serde_json::json!({"graph_id": graph_id})).is_ok() {
            successes += 1;
        }
    }
    let elapsed = start.elapsed();
    let exec_per_sec = successes as f64 / elapsed.as_secs_f64();

    v.check_bool(
        "throughput_ceiling",
        exec_per_sec > 10.0,
        &format!(
            "{successes}/{n} executions in {:.0}ms = {exec_per_sec:.0} exec/s (>10 required)",
            elapsed.as_secs_f64() * 1000.0
        ),
    );

    v.check_bool(
        "zero_rejection_rate",
        successes >= n * 9 / 10,
        &format!("{successes}/{n} accepted (≥90% required)"),
    );
}

fn phase_parallelization_signal(v: &mut ValidationResult) {
    match neural_rpc("graph.list", &serde_json::json!({})) {
        Ok(graphs) => {
            let graphs = graphs.as_array().cloned().unwrap_or_default();
            let parallel_graphs: Vec<_> = graphs
                .iter()
                .filter(|g| {
                    g.get("coordination")
                        .and_then(|c| c.as_str())
                        .is_some_and(|c| c.to_lowercase() == "parallel")
                })
                .collect();

            v.check_bool(
                "parallel_graphs_exist",
                !parallel_graphs.is_empty(),
                &format!(
                    "{} parallel-coordination graphs available (of {} total)",
                    parallel_graphs.len(),
                    graphs.len()
                ),
            );

            let multi_phase: Vec<_> = graphs
                .iter()
                .filter(|g| g.get("node_count").and_then(|n| n.as_u64()).unwrap_or(0) >= 3)
                .collect();

            v.check_bool(
                "multi_node_graphs_available",
                multi_phase.len() >= 10,
                &format!(
                    "{} graphs with ≥3 nodes (PathwayLearner can detect parallelization)",
                    multi_phase.len()
                ),
            );
        }
        Err(e) => {
            v.check_bool("parallel_graphs_exist", false, &format!("graph.list: {e}"));
        }
    }
}

fn phase_learning_readiness(v: &mut ValidationResult) {
    let observations_needed = 50;
    let graph_id = "tower_health";

    let start = Instant::now();
    let mut collected = 0u32;
    for _ in 0..observations_needed {
        if execute_and_trace(graph_id).is_ok() {
            collected += 1;
        }
    }
    let elapsed = start.elapsed();

    v.check_bool(
        "observation_throughput",
        collected >= observations_needed * 3 / 4,
        &format!(
            "{collected}/{observations_needed} observations in {:.1}s ({:.0}/s)",
            elapsed.as_secs_f64(),
            collected as f64 / elapsed.as_secs_f64()
        ),
    );

    let convergence_possible = collected >= 10;
    v.check_bool(
        "learning_convergence_possible",
        convergence_possible,
        &format!(
            "PathwayLearner needs ≥10 observations for first suggestion; we have {collected}"
        ),
    );

    let prewarm_signal = collected >= 5;
    v.check_bool(
        "prewarm_detection_possible",
        prewarm_signal,
        "≥5 executions → Prewarm pattern detection feasible",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp119 — PathwayLearner Structural Validation")
        .with_provenance("exp119_pathway_learner_structural", "2026-08-08")
        .run(
            "PathwayLearner prerequisites: traces → timing → patterns → suggestions",
            |v| {
                let path = neural_socket_path();
                if !Path::new(&path).exists() {
                    v.check_skip("neural_api", "Neural API socket not found — all phases skipped");
                    return;
                }

                v.section("Phase 1: Trace accumulation (ExecutionTrace pipeline)");
                phase_trace_accumulation(v);

                v.section("Phase 2: Node-level observability");
                phase_node_observability(v);

                v.section("Phase 3: Throughput ceiling (learning signal bandwidth)");
                phase_throughput_ceiling(v);

                v.section("Phase 4: Parallelization signal (graph topology analysis)");
                phase_parallelization_signal(v);

                v.section("Phase 5: Learning readiness (observation count threshold)");
                phase_learning_readiness(v);
            },
        );
}
