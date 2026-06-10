// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Perceptron feature extraction — 36-dim dispatch telemetry vectors.
//!
//! Transforms raw [`DispatchMetric`] records into fixed-width feature vectors
//! suitable for `ml.mlp_train` on barraCuda. The perceptron learns adaptive
//! routing weights from historical dispatch patterns.
//!
//! # Feature Vector Layout (36 dimensions)
//!
//! | Dim | Name | Range | Source |
//! |-----|------|-------|--------|
//! | 0–6 | `tier_onehot` | {0,1} | CompositionTier (7 variants) |
//! | 7–11 | `route_onehot` | {0,1} | RoutePath (5 variants) |
//! | 12 | `latency_norm` | [0,1] | latency_ms / MAX_LATENCY_MS |
//! | 13 | `success` | {0,1} | 1.0 if succeeded |
//! | 14–26 | `owner_onehot` | {0,1} | primal index (13 primals) |
//! | 27 | `hour_sin` | [-1,1] | sin(2π * hour/24) |
//! | 28 | `hour_cos` | [-1,1] | cos(2π * hour/24) |
//! | 29 | `day_sin` | [-1,1] | sin(2π * day_of_week/7) |
//! | 30 | `day_cos` | [-1,1] | cos(2π * day_of_week/7) |
//! | 31 | `method_hash_0` | [0,1] | method name hash bits 0–7 / 255 |
//! | 32 | `method_hash_1` | [0,1] | method name hash bits 8–15 / 255 |
//! | 33 | `method_hash_2` | [0,1] | method name hash bits 16–23 / 255 |
//! | 34 | `method_hash_3` | [0,1] | method name hash bits 24–31 / 255 |
//! | 35 | `domain_depth` | [0,1] | number of '.' separators / MAX_DEPTH |
//!
//! # Label (training target)
//!
//! For routing optimization, the label is the **normalized reward**:
//! `reward = success * (1.0 - latency_norm)` — faster successes score higher.

use super::RoutePath;
use super::metrics::DispatchMetric;
use crate::composition::neural_routing::CompositionTier;

/// Feature vector dimensionality.
pub const FEATURE_DIM: usize = 36;

/// Maximum latency for normalization (10 seconds). Dispatches slower than
/// this are clamped to 1.0.
const MAX_LATENCY_MS: f64 = 10_000.0;

/// Maximum method depth for normalization.
const MAX_DEPTH: f64 = 4.0;

/// Ordered primal slugs for one-hot encoding (dims 14–26).
///
/// Derived from `Primal::ALL` at runtime to stay in sync with
/// the canonical primal list — no hardcoded slug array.
fn primal_slugs() -> Vec<&'static str> {
    crate::primal_names::Primal::ALL
        .iter()
        .map(|p| p.slug())
        .collect()
}

/// A 36-dimensional feature vector extracted from a dispatch metric.
#[derive(Debug, Clone)]
pub struct FeatureVector {
    /// Fixed-width feature array.
    pub features: [f64; FEATURE_DIM],
    /// Training label (reward signal).
    pub label: f64,
}

impl FeatureVector {
    /// Extract features from a single dispatch metric.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "feature normalization — sub-unit precision acceptable for ML inputs"
    )]
    pub fn from_metric(metric: &DispatchMetric) -> Self {
        let mut features = [0.0_f64; FEATURE_DIM];

        // Dims 0–6: CompositionTier one-hot
        features[tier_index(metric.tier)] = 1.0;

        // Dims 7–11: RoutePath one-hot
        features[7 + route_index(&metric.route_path)] = 1.0;

        // Dim 12: normalized latency
        features[12] = (metric.latency_ms as f64 / MAX_LATENCY_MS).min(1.0);

        // Dim 13: success indicator
        features[13] = if metric.success { 1.0 } else { 0.0 };

        // Dims 14–26: primal owner one-hot
        if let Some(idx) = primal_index(&metric.owner) {
            features[14 + idx] = 1.0;
        }

        // Dims 27–30: cyclical time encoding
        let (hour_sin, hour_cos, day_sin, day_cos) =
            cyclical_time_features(metric.timestamp_epoch_ms);
        features[27] = hour_sin;
        features[28] = hour_cos;
        features[29] = day_sin;
        features[30] = day_cos;

        // Dims 31–34: method name hash (locality-sensitive)
        let hash = method_hash(&metric.method);
        features[31] = f64::from(hash[0]) / 255.0;
        features[32] = f64::from(hash[1]) / 255.0;
        features[33] = f64::from(hash[2]) / 255.0;
        features[34] = f64::from(hash[3]) / 255.0;

        // Dim 35: method domain depth
        let depth = metric.method.chars().filter(|&c| c == '.').count();
        features[35] = (depth as f64 / MAX_DEPTH).min(1.0);

        // Label: reward = success * (1 - latency_norm)
        let latency_norm = features[12];
        let label = features[13] * (1.0 - latency_norm);

        Self { features, label }
    }

    /// Serialize as a JSON-lines training record.
    #[must_use]
    pub fn to_jsonl(&self) -> String {
        serde_json::json!({
            "features": self.features.as_slice(),
            "label": self.label,
        })
        .to_string()
    }
}

/// Extract a batch of feature vectors from dispatch metrics.
#[must_use]
pub fn extract_batch(metrics: &[DispatchMetric]) -> Vec<FeatureVector> {
    metrics.iter().map(FeatureVector::from_metric).collect()
}

/// Generate training data file from raw telemetry.
///
/// Reads `dispatch_telemetry.jsonl`, extracts features, writes to
/// `training_vectors.jsonl` — ready for `ml.mlp_train`.
///
/// # Errors
///
/// Returns IO errors from file operations or JSON parsing failures.
pub fn generate_training_data(
    telemetry_path: &std::path::Path,
    output_path: &std::path::Path,
) -> std::io::Result<usize> {
    use std::io::{BufRead, BufReader, Write};

    let file = std::fs::File::open(telemetry_path)?;
    let reader = BufReader::new(file);

    let out_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;
    let mut writer = std::io::BufWriter::new(out_file);

    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let Ok(metric) = serde_json::from_str::<DispatchMetric>(&line) else {
            continue;
        };
        let fv = FeatureVector::from_metric(&metric);
        writeln!(writer, "{}", fv.to_jsonl())?;
        count += 1;
    }

    writer.flush()?;
    Ok(count)
}

const fn tier_index(tier: CompositionTier) -> usize {
    match tier {
        CompositionTier::Tower => 0,
        CompositionTier::Node => 1,
        CompositionTier::Nest => 2,
        CompositionTier::Nucleus => 3,
        CompositionTier::Meta => 4,
        CompositionTier::Orchestration => 5,
        CompositionTier::Standalone => 6,
    }
}

const fn route_index(route: &RoutePath) -> usize {
    match route {
        RoutePath::CapabilityCall => 0,
        RoutePath::CompositionDispatch => 1,
        RoutePath::GraphExecute => 2,
        RoutePath::Unresolved => 3,
        RoutePath::Offline => 4,
    }
}

fn primal_index(owner: &str) -> Option<usize> {
    primal_slugs().iter().position(|&s| s == owner)
}

#[expect(
    clippy::cast_precision_loss,
    reason = "time normalization — hour/day values are small, no precision loss in practice"
)]
fn cyclical_time_features(epoch_ms: u64) -> (f64, f64, f64, f64) {
    use std::f64::consts::TAU;

    let secs = epoch_ms / 1000;
    let hour = ((secs % 86400) as f64) / 3600.0;
    let day = ((secs / 86400) % 7) as f64;

    let hour_sin = (TAU * hour / 24.0).sin();
    let hour_cos = (TAU * hour / 24.0).cos();
    let day_sin = (TAU * day / 7.0).sin();
    let day_cos = (TAU * day / 7.0).cos();

    (hour_sin, hour_cos, day_sin, day_cos)
}

/// Locality-sensitive hash of a method name. Uses FNV-1a to produce 4 bytes
/// that capture method identity without requiring a lookup table.
fn method_hash(method: &str) -> [u8; 4] {
    let mut h: u32 = 0x811c_9dc5; // FNV offset basis
    for byte in method.as_bytes() {
        h ^= u32::from(*byte);
        h = h.wrapping_mul(0x0100_0193); // FNV prime
    }
    h.to_le_bytes()
}

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "exact float equality is correct for one-hot encoding tests"
)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn sample_metric() -> DispatchMetric {
        DispatchMetric {
            method: Arc::from("crypto.sign_ed25519"),
            owner: Arc::from("beardog"),
            tier: CompositionTier::Tower,
            latency_ms: 42,
            success: true,
            route_path: RoutePath::CapabilityCall,
            timestamp_epoch_ms: 1_717_400_000_000,
        }
    }

    #[test]
    fn feature_vector_dimensionality() {
        let fv = FeatureVector::from_metric(&sample_metric());
        assert_eq!(fv.features.len(), FEATURE_DIM);
    }

    #[test]
    fn tier_onehot_correct() {
        let fv = FeatureVector::from_metric(&sample_metric());
        assert_eq!(fv.features[0], 1.0); // Tower = index 0
        assert_eq!(fv.features[1], 0.0);
        assert_eq!(fv.features[6], 0.0);
    }

    #[test]
    fn route_onehot_correct() {
        let fv = FeatureVector::from_metric(&sample_metric());
        assert_eq!(fv.features[7], 1.0); // CapabilityCall = route index 0
        assert_eq!(fv.features[8], 0.0);
    }

    #[test]
    fn latency_normalized() {
        let fv = FeatureVector::from_metric(&sample_metric());
        let expected = 42.0 / MAX_LATENCY_MS;
        assert!((fv.features[12] - expected).abs() < 1e-10);
    }

    #[test]
    fn success_encoded() {
        let fv = FeatureVector::from_metric(&sample_metric());
        assert_eq!(fv.features[13], 1.0);

        let mut failed = sample_metric();
        failed.success = false;
        let fv2 = FeatureVector::from_metric(&failed);
        assert_eq!(fv2.features[13], 0.0);
    }

    #[test]
    fn owner_onehot_correct() {
        let fv = FeatureVector::from_metric(&sample_metric());
        assert_eq!(fv.features[14], 1.0); // beardog = primal index 0
        assert_eq!(fv.features[15], 0.0);
    }

    #[test]
    fn label_is_reward() {
        let fv = FeatureVector::from_metric(&sample_metric());
        let expected = 1.0 * (1.0 - 42.0 / MAX_LATENCY_MS);
        assert!((fv.label - expected).abs() < 1e-10);
    }

    #[test]
    fn cyclical_features_bounded() {
        let fv = FeatureVector::from_metric(&sample_metric());
        for &dim in &[27, 28, 29, 30] {
            assert!(fv.features[dim] >= -1.0 && fv.features[dim] <= 1.0);
        }
    }

    #[test]
    fn method_hash_deterministic() {
        let h1 = method_hash("crypto.sign_ed25519");
        let h2 = method_hash("crypto.sign_ed25519");
        assert_eq!(h1, h2);
        let h3 = method_hash("tensor.matmul");
        assert_ne!(h1, h3);
    }

    #[test]
    fn domain_depth_encoded() {
        let fv = FeatureVector::from_metric(&sample_metric());
        assert!((fv.features[35] - 1.0 / MAX_DEPTH).abs() < 1e-10);
    }

    #[test]
    fn batch_extraction() {
        let metrics = vec![sample_metric(), sample_metric()];
        let batch = extract_batch(&metrics);
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn jsonl_serialization() {
        let fv = FeatureVector::from_metric(&sample_metric());
        let json = fv.to_jsonl();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let features = parsed["features"].as_array().unwrap();
        assert_eq!(features.len(), FEATURE_DIM);
    }
}
