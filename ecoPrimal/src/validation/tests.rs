// SPDX-License-Identifier: AGPL-3.0-or-later

use std::sync::Arc;

use super::*;

#[test]
fn new_result_has_zero_counts() {
    let v = ValidationResult::new("test");
    assert_eq!(v.passed, 0);
    assert_eq!(v.failed, 0);
    assert_eq!(v.skipped, 0);
    assert!(v.checks.is_empty());
}

#[test]
fn check_bool_pass_increments_passed() {
    let mut v = ValidationResult::new("test");
    v.check_bool("ok", true, "detail");
    assert_eq!(v.passed, 1);
    assert_eq!(v.failed, 0);
    assert!(v.all_passed());
}

#[test]
fn check_bool_fail_increments_failed() {
    let mut v = ValidationResult::new("test");
    v.check_bool("bad", false, "detail");
    assert_eq!(v.passed, 0);
    assert_eq!(v.failed, 1);
    assert!(!v.all_passed());
}

#[test]
fn check_skip_increments_skipped() {
    let mut v = ValidationResult::new("test");
    v.check_skip("pending", "needs live primals");
    assert_eq!(v.skipped, 1);
    assert_eq!(v.passed, 0);
    assert_eq!(v.failed, 0);
    assert!(!v.all_passed());
}

#[test]
fn all_passed_requires_at_least_one_pass() {
    let v = ValidationResult::new("test");
    assert!(!v.all_passed());
}

#[test]
fn check_latency_pass() {
    let mut v = ValidationResult::new("test");
    v.check_latency("fast", 100, 50_000);
    assert!(v.all_passed());
}

#[test]
fn check_latency_fail() {
    let mut v = ValidationResult::new("test");
    v.check_latency("slow", 100_000, 50_000);
    assert!(!v.all_passed());
}

#[test]
fn check_count_pass() {
    let mut v = ValidationResult::new("test");
    v.check_count("exact", 5, 5);
    assert!(v.all_passed());
}

#[test]
fn check_count_fail() {
    let mut v = ValidationResult::new("test");
    v.check_count("wrong", 3, 5);
    assert!(!v.all_passed());
}

#[test]
fn check_minimum_pass() {
    let mut v = ValidationResult::new("test");
    v.check_minimum("enough", 10, 5);
    assert!(v.all_passed());
}

#[test]
fn check_minimum_fail() {
    let mut v = ValidationResult::new("test");
    v.check_minimum("low", 2, 5);
    assert!(!v.all_passed());
}

#[test]
fn evaluated_excludes_skips() {
    let mut v = ValidationResult::new("test");
    v.check_bool("ok", true, "yes");
    v.check_skip("pending", "later");
    v.check_bool("bad", false, "no");
    assert_eq!(v.evaluated(), 2);
    assert_eq!(v.skipped, 1);
}

#[test]
fn display_format() {
    let mut v = ValidationResult::new("exp001");
    v.check_bool("ok", true, "yes");
    let display = format!("{v}");
    assert!(display.contains("exp001"));
    assert!(display.contains("1/1 passed"));
}

#[test]
fn display_format_with_skips() {
    let mut v = ValidationResult::new("exp001");
    v.check_bool("ok", true, "yes");
    v.check_skip("pending", "later");
    let display = format!("{v}");
    assert!(display.contains("1 skipped"));
}

#[test]
fn to_json_round_trip() {
    let mut v = ValidationResult::new("exp_json");
    v.check_bool("ok", true, "detail");
    v.check_skip("pending", "later");

    let json = v.to_json().unwrap();
    let back: ValidationResult = serde_json::from_str(&json).unwrap();
    assert_eq!(back.experiment, "exp_json");
    assert_eq!(back.passed, 1);
    assert_eq!(back.skipped, 1);
    assert_eq!(back.checks.len(), 2);
}

#[test]
fn exit_code_zero_on_pass() {
    let mut v = ValidationResult::new("test");
    v.check_bool("ok", true, "yes");
    assert_eq!(v.exit_code(), 0);
}

#[test]
fn exit_code_one_on_fail() {
    let mut v = ValidationResult::new("test");
    v.check_bool("bad", false, "no");
    assert_eq!(v.exit_code(), 1);
}

#[test]
fn check_or_skip_runs_check_when_some() {
    let mut v = ValidationResult::new("test");
    v.check_or_skip("found", Some(42u64), "not available", |val, v| {
        v.check_bool("value", val == 42, "expected 42");
    });
    assert_eq!(v.passed, 1);
    assert_eq!(v.skipped, 0);
}

#[test]
fn check_or_skip_skips_when_none() {
    let mut v = ValidationResult::new("test");
    v.check_or_skip::<u64, _>("missing", None, "not available", |_, _| {
        panic!("should not be called");
    });
    assert_eq!(v.passed, 0);
    assert_eq!(v.skipped, 1);
}

#[test]
fn provenance_none_by_default() {
    let v = ValidationResult::new("test");
    assert!(v.provenance.is_none());
}

#[test]
fn with_provenance_sets_field() {
    let v = ValidationResult::new("test").with_provenance("exp001_tower", "2026-03-18");
    let prov = v.provenance.as_ref().unwrap();
    assert_eq!(prov.source, "exp001_tower");
    assert_eq!(prov.baseline_date.as_deref(), Some("2026-03-18"));
    assert!(prov.description.is_none());
}

#[test]
fn with_provenance_full_sets_all_fields() {
    let v = ValidationResult::new("test").with_provenance_full(
        "exp050_compute_triangle",
        "2026-03-18",
        "Compute triangle coordination validation",
    );
    let prov = v.provenance.as_ref().unwrap();
    assert_eq!(prov.source, "exp050_compute_triangle");
    assert_eq!(prov.baseline_date.as_deref(), Some("2026-03-18"));
    assert_eq!(
        prov.description.as_deref(),
        Some("Compute triangle coordination validation")
    );
}

#[test]
fn provenance_survives_json_round_trip() {
    let mut v = ValidationResult::new("exp_prov").with_provenance("exp001_tower", "2026-03-18");
    v.check_bool("ok", true, "yes");

    let json = v.to_json().unwrap();
    let back: ValidationResult = serde_json::from_str(&json).unwrap();
    let prov = back.provenance.as_ref().unwrap();
    assert_eq!(prov.source, "exp001_tower");
}

#[test]
fn provenance_absent_omitted_from_json() {
    let mut v = ValidationResult::new("no_prov");
    v.check_bool("ok", true, "yes");

    let json = v.to_json().unwrap();
    assert!(!json.contains("provenance"));
}

#[test]
fn all_experiment_tracks_have_provenance_schema() {
    let experiment_ids = [
        "exp001", "exp002", "exp003", "exp004", "exp005", "exp006", "exp010", "exp011", "exp012",
        "exp013", "exp014", "exp015", "exp020", "exp021", "exp022", "exp023", "exp024", "exp025",
        "exp030", "exp031", "exp032", "exp033", "exp034", "exp040", "exp041", "exp042", "exp043",
        "exp044", "exp050", "exp051", "exp052", "exp053", "exp054", "exp055", "exp056", "exp057",
        "exp058", "exp059", "exp060", "exp061", "exp062", "exp063", "exp064", "exp065", "exp066",
        "exp067", "exp068", "exp069", "exp070", "exp071", "exp072", "exp073", "exp074", "exp075",
        "exp076", "exp077", "exp078", "exp079", "exp080", "exp081", "exp082", "exp083", "exp084",
        "exp085", "exp086", "exp087", "exp088", "exp089", "exp090", "exp091", "exp092", "exp093",
        "exp094", "exp095", "exp096",
    ];
    assert_eq!(
        experiment_ids.len(),
        75,
        "expected 75 experiments across tracks"
    );
    let tracks: std::collections::HashSet<u32> = experiment_ids
        .iter()
        .filter_map(|id| id.strip_prefix("exp"))
        .filter_map(|num| num.parse::<u32>().ok())
        .map(|n| n / 10)
        .collect();
    assert!(
        tracks.len() >= 8,
        "expected at least 8 tracks, got {}",
        tracks.len()
    );
}

#[test]
fn print_banner_does_not_panic() {
    ValidationResult::print_banner("Test Banner Title");
}

#[test]
fn with_sink_replaces_default() {
    let v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    assert_eq!(v.passed, 0);
    assert_eq!(v.failed, 0);
}

#[test]
fn evaluated_counts_pass_and_fail_only() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_bool("a", true, "pass");
    v.check_bool("b", false, "fail");
    v.check_skip("c", "skip");
    assert_eq!(v.evaluated(), 2);
    assert_eq!(v.passed, 1);
    assert_eq!(v.failed, 1);
    assert_eq!(v.skipped, 1);
}

#[test]
fn check_count_records_exact_match() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_count("count_ok", 5, 5);
    assert_eq!(v.passed, 1);
    v.check_count("count_bad", 3, 5);
    assert_eq!(v.failed, 1);
}

#[test]
fn check_minimum_records_threshold() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_minimum("min_ok", 5, 3);
    assert_eq!(v.passed, 1);
    v.check_minimum("min_bad", 1, 3);
    assert_eq!(v.failed, 1);
}

#[test]
fn exit_code_zero_when_all_passed() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_bool("ok", true, "yes");
    assert_eq!(v.exit_code(), 0);
}

#[test]
fn exit_code_one_when_failure() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_bool("bad", false, "no");
    assert_eq!(v.exit_code(), 1);
}

#[test]
fn display_format_includes_experiment_name() {
    let v = ValidationResult::new("my experiment").with_sink(Arc::new(NullSink));
    let display = format!("{v}");
    assert!(display.contains("my experiment"));
}

#[test]
fn section_does_not_panic() {
    let v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.section("IPC health");
}

#[test]
fn exit_code_skip_aware_zero_on_pass() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_bool("ok", true, "yes");
    assert_eq!(v.exit_code_skip_aware(), 0);
}

#[test]
fn exit_code_skip_aware_one_on_fail() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_bool("bad", false, "no");
    assert_eq!(v.exit_code_skip_aware(), 1);
}

#[test]
fn exit_code_skip_aware_two_when_all_skipped() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_skip("pending", "no live primals");
    assert_eq!(v.exit_code_skip_aware(), 2);
}

#[test]
fn exit_code_skip_aware_two_when_empty() {
    let v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    assert_eq!(v.exit_code_skip_aware(), 2);
}

#[test]
fn stdout_sink_section_does_not_panic() {
    let sink = StdoutSink;
    sink.section("test section");
}

#[test]
fn stdout_sink_write_summary_does_not_panic() {
    let sink = StdoutSink;
    sink.write_summary(5, 1, 2);
}

#[test]
fn check_result_passed_method() {
    let pass = CheckResult {
        name: "a".to_owned(),
        outcome: CheckOutcome::Pass,
        detail: "ok".to_owned(),
    };
    assert!(pass.passed());

    let fail = CheckResult {
        name: "b".to_owned(),
        outcome: CheckOutcome::Fail,
        detail: "no".to_owned(),
    };
    assert!(!fail.passed());
}

// ── check_relative tests ──

#[test]
fn check_relative_pass_exact() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_relative("exact", 1.0, 1.0, 0.01);
    assert_eq!(v.passed, 1);
}

#[test]
fn check_relative_pass_within_tol() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_relative("close", 1.005, 1.0, 0.01);
    assert_eq!(v.passed, 1);
}

#[test]
fn check_relative_fail_outside_tol() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_relative("far", 1.05, 1.0, 0.01);
    assert_eq!(v.failed, 1);
}

#[test]
fn check_relative_zero_expected() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_relative("zero_ok", 0.005, 0.0, 0.01);
    assert_eq!(v.passed, 1);
    v.check_relative("zero_bad", 0.05, 0.0, 0.01);
    assert_eq!(v.failed, 1);
}

// ── check_abs_or_rel tests ──

#[test]
fn check_abs_or_rel_pass_by_abs() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_abs_or_rel("abs_ok", 0.001, 0.0, 0.01, 0.000_01);
    assert_eq!(v.passed, 1);
}

#[test]
fn check_abs_or_rel_pass_by_rel() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_abs_or_rel("rel_ok", 100.5, 100.0, 0.001, 0.01);
    assert_eq!(v.passed, 1);
}

#[test]
fn check_abs_or_rel_fail_both() {
    let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
    v.check_abs_or_rel("both_bad", 200.0, 100.0, 0.01, 0.01);
    assert_eq!(v.failed, 1);
}

// ── NdjsonSink tests ──

#[test]
fn ndjson_sink_emits_valid_json() {
    let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
    let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
    sink.on_check(CheckOutcome::Pass, "test_check", "all good");

    let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(parsed["outcome"], "pass");
    assert_eq!(parsed["name"], "test_check");
}

#[test]
fn ndjson_sink_section_emits_json() {
    let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
    let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
    sink.section("IPC health");

    let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(parsed["section"], "IPC health");
}

#[test]
fn ndjson_sink_summary_emits_json() {
    let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
    let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
    sink.write_summary(10, 2, 3);

    let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(parsed["summary"]["passed"], 10);
    assert_eq!(parsed["summary"]["failed"], 2);
    assert_eq!(parsed["summary"]["skipped"], 3);
}

#[test]
fn ndjson_sink_with_validation_result() {
    let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
    let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
    let mut v = ValidationResult::new("ndjson_test").with_sink(Arc::new(sink));
    v.check_bool("a", true, "pass");
    v.check_bool("b", false, "fail");
    v.check_skip("c", "skipped");

    let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert_eq!(output.trim().lines().count(), 3);
    assert_eq!(v.passed, 1);
    assert_eq!(v.failed, 1);
    assert_eq!(v.skipped, 1);
}

/// Helper writer that delegates to a shared `Vec<u8>` behind a Mutex.
#[derive(Debug, Clone)]
struct CursorWriter(Arc<std::sync::Mutex<Vec<u8>>>);

impl std::io::Write for CursorWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
