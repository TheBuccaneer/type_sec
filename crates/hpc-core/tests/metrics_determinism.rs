// crates/hpc-core/tests/metrics_determinism.rs
// cargo test -p hpc-core --features metrics -- metrics_determinism

#![cfg(feature = "metrics")]

use std::{fs, thread::sleep, time::Duration};
use serde_json::Value;
use hpc_core::metrics::{RunLog, log_run_to};

#[cfg_attr(windows, ignore)]
#[test]
fn metrics_determinism_guard() {
    // Eigener Temp-Ordner → keine Kollisionen mit anderen Tests
    let base = std::env::temp_dir().join("hpc-core-test-det-guard");

    // Zwei identische Writes (gleiches Input), minimale Pause für andere ts
    let p1 = log_run_to(&RunLog { example: "vec_add", n: 256 }, &base)
        .expect("first write ok");
    sleep(Duration::from_millis(2));
    let p2 = log_run_to(&RunLog { example: "vec_add", n: 256 }, &base)
        .expect("second write ok");

    // Beide Writes landen in derselben Datei (run.jsonl)
    assert_eq!(p1, p2, "writes should append to the same run.jsonl");

    // Letzte zwei **nicht-leeren** Zeilen lesen
    let s = fs::read_to_string(&p1).expect("read run.jsonl");
    let mut lines = s.lines().rev().filter(|l| !l.trim().is_empty());
    let last   = lines.next().expect("has last line");
    let before = lines.next().expect("has previous line");

    let v_last:   Value = serde_json::from_str(last).expect("parse last");
    let v_before: Value = serde_json::from_str(before).expect("parse prev");

    // Erlaubte Abweichung: nur der Timestamp
    for key in ["example","n","allocs","alloc_bytes","schema_version","pid"] {
        assert_eq!(v_last[ key ], v_before[ key ], "field `{key}` drifted");
    }

    // Timestamp muss sich unterscheiden (neue Messung)
    assert_ne!(v_last["ts"], v_before["ts"], "expected `ts` to differ");

    // Sanity: `ts` sieht aus wie RFC3339 (einfacher Check)
    let ts_ok = v_last["ts"].as_str().map(|s| s.contains('T')).unwrap_or(false);
    assert!(ts_ok, "`ts` should look like RFC3339 (contain 'T')");

    for key in ["example","n","allocs","alloc_bytes","schema_version","pid","run_id"] {
    assert_eq!(v_last[key], v_before[key], "field `{key}` drifted");
}
}


use regex::Regex;

#[cfg_attr(windows, ignore)]
#[test]
fn run_id_is_stable_and_well_formed() {
    let base = std::env::temp_dir().join("hpc-core-test-run-id");

    let p1 = log_run_to(&RunLog { example: "vec_add", n: 16 }, &base).unwrap();
    sleep(Duration::from_millis(2));
    let p2 = log_run_to(&RunLog { example: "vec_add", n: 16 }, &base).unwrap();

    assert_eq!(p1, p2, "expected same JSONL file");

    let s = fs::read_to_string(&p1).unwrap();
    let mut lines = s.lines().rev().filter(|l| !l.trim().is_empty());
    let last   = lines.next().unwrap();
    let before = lines.next().unwrap();

    let v_last:   Value = serde_json::from_str(last).unwrap();
    let v_before: Value = serde_json::from_str(before).unwrap();

    let r1 = v_last["run_id"].as_str().expect("run_id present");
    let r0 = v_before["run_id"].as_str().expect("run_id present");

    // Stabil innerhalb eines Prozesses
    assert_eq!(r1, r0, "run_id must remain stable within the same process");

    // Format: YYYYMMDD-HHMMSS-XXXX-PID
    let re = Regex::new(r"^\d{8}-\d{6}-[0-9A-F]{4}-\d+$").unwrap();
    assert!(re.is_match(r1), "run_id format unexpected: {r1}");
}