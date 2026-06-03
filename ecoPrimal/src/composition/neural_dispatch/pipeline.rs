// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Perceptron training pipeline — end-to-end telemetry → weights.
//!
//! Orchestrates the full pipeline:
//! 1. Read `dispatch_telemetry.jsonl` from `XDG_DATA_HOME/biomeos/`
//! 2. Extract 36-dim feature vectors via [`super::perceptron`]
//! 3. Call `ml.perceptron_train` on barraCuda (via capability.call)
//! 4. Write resulting weights to `neural_routing_perceptron.bin`
//! 5. biomeOS auto-loads weights on next composition reload

use std::path::{Path, PathBuf};

use super::perceptron::{self, FeatureVector};
use crate::composition::CompositionContext;

/// Pipeline result containing training metadata.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    /// Number of telemetry records processed.
    pub records_processed: usize,
    /// Number of valid feature vectors extracted.
    pub vectors_extracted: usize,
    /// Whether the RPC to barraCuda succeeded.
    pub training_success: bool,
    /// Path where the weights binary was written (if successful).
    pub weights_path: Option<PathBuf>,
    /// Training loss from barraCuda (if returned).
    pub final_loss: Option<f64>,
}

/// Default telemetry source path.
#[must_use]
pub fn default_telemetry_path() -> PathBuf {
    let xdg_data = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap_or_default()));
    PathBuf::from(xdg_data).join("biomeos/dispatch_telemetry.jsonl")
}

/// Default weights output path.
#[must_use]
pub fn default_weights_path() -> PathBuf {
    let xdg_data = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap_or_default()));
    PathBuf::from(xdg_data).join("biomeos/neural_routing_perceptron.bin")
}

/// Run the full perceptron training pipeline.
///
/// # Steps
/// 1. Read telemetry from `telemetry_path`
/// 2. Extract feature vectors
/// 3. Call `ml.perceptron_train` on barraCuda via composition context
/// 4. Write weights to `weights_path`
///
/// # Errors
///
/// Returns an error if telemetry cannot be read or training fails.
pub fn run_pipeline(
    ctx: &mut CompositionContext,
    telemetry_path: &Path,
    weights_path: &Path,
) -> Result<PipelineResult, PipelineError> {
    let records = read_telemetry(telemetry_path)?;
    let records_processed = records.len();

    if records.is_empty() {
        return Ok(PipelineResult {
            records_processed: 0,
            vectors_extracted: 0,
            training_success: false,
            weights_path: None,
            final_loss: None,
        });
    }

    let vectors = perceptron::extract_batch(&records);
    let vectors_extracted = vectors.len();

    let training_records: Vec<serde_json::Value> = vectors
        .iter()
        .map(|fv| {
            serde_json::json!({
                "features": fv.features.as_slice(),
                "label": fv.label,
            })
        })
        .collect();

    let train_result = call_perceptron_train(ctx, &training_records, weights_path);

    match train_result {
        Ok(loss) => Ok(PipelineResult {
            records_processed,
            vectors_extracted,
            training_success: true,
            weights_path: Some(weights_path.to_owned()),
            final_loss: loss,
        }),
        Err(e) => Err(PipelineError::Training(format!("{e}"))),
    }
}

/// Produce training vectors from telemetry without calling barraCuda.
///
/// Useful for validation and offline feature inspection.
#[must_use]
pub fn extract_vectors(telemetry_path: &Path) -> Vec<FeatureVector> {
    read_telemetry(telemetry_path)
        .map(|records| perceptron::extract_batch(&records))
        .unwrap_or_default()
}

/// Pipeline errors.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    /// Cannot read telemetry source.
    #[error("telemetry read: {0}")]
    TelemetryRead(#[from] std::io::Error),
    /// Training RPC failed.
    #[error("training: {0}")]
    Training(String),
}

fn read_telemetry(
    path: &Path,
) -> Result<Vec<super::metrics::DispatchMetric>, std::io::Error> {
    use std::io::{BufRead, BufReader};

    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let metrics: Vec<_> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if line.trim().is_empty() {
                return None;
            }
            serde_json::from_str(&line).ok()
        })
        .collect();

    Ok(metrics)
}

fn call_perceptron_train(
    ctx: &mut CompositionContext,
    training_records: &[serde_json::Value],
    output_path: &Path,
) -> Result<Option<f64>, crate::ipc::error::IpcError> {
    let params = serde_json::json!({
        "records": training_records,
        "learning_rate": 0.01,
        "epochs": 10,
        "output_path": output_path.to_string_lossy(),
    });

    let resp = ctx.call("compute", "ml.perceptron_train", params)?;

    let loss = resp.get("final_loss").and_then(serde_json::Value::as_f64);
    Ok(loss)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn sample_telemetry_jsonl() -> String {
        [
            r#"{"method":"crypto.sign_ed25519","owner":"beardog","tier":"Tower","latency_ms":42,"success":true,"route_path":"CapabilityCall","timestamp_epoch_ms":1717400000000}"#,
            r#"{"method":"storage.store","owner":"nestgate","tier":"Nest","latency_ms":120,"success":true,"route_path":"CapabilityCall","timestamp_epoch_ms":1717400001000}"#,
            r#"{"method":"compute.dispatch","owner":"toadstool","tier":"Node","latency_ms":8,"success":false,"route_path":"CompositionDispatch","timestamp_epoch_ms":1717400002000}"#,
        ].join("\n")
    }

    #[test]
    fn read_telemetry_parses_valid_jsonl() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{}", sample_telemetry_jsonl()).unwrap();

        let metrics = read_telemetry(tmp.path()).unwrap();
        assert_eq!(metrics.len(), 3);
        assert_eq!(&*metrics[0].method, "crypto.sign_ed25519");
        assert_eq!(&*metrics[1].method, "storage.store");
    }

    #[test]
    fn extract_vectors_from_file() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{}", sample_telemetry_jsonl()).unwrap();

        let vectors = extract_vectors(tmp.path());
        assert_eq!(vectors.len(), 3);
        assert_eq!(vectors[0].features.len(), 36);
    }

    #[test]
    fn default_paths_are_reasonable() {
        let telemetry = default_telemetry_path();
        assert!(telemetry.to_string_lossy().contains("dispatch_telemetry"));

        let weights = default_weights_path();
        assert!(weights.to_string_lossy().contains("neural_routing_perceptron"));
    }

    #[test]
    fn pipeline_empty_telemetry() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "").unwrap();

        let mut ctx = CompositionContext::discover();
        let result = run_pipeline(&mut ctx, tmp.path(), Path::new("/tmp/test.bin")).unwrap();
        assert_eq!(result.records_processed, 0);
        assert!(!result.training_success);
    }
}
