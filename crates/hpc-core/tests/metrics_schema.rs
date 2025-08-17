#![cfg(feature = "metrics")]

use hpc_core::metrics::schema;
use hpc_core::metrics::{RunLog, log_run_to};
use serde_json::json;
use std::fs;

/*
#[test]
fn schema_v1_accepts_current_writer_output() {
    let base = std::env::temp_dir().join("hpc-core-test-schema-ok");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    // ein paar Writes erzeugen
    let p = log_run_to(&RunLog { example: "vec_add", n: 1 }, &base).unwrap();
    let _ = log_run_to(&RunLog { example: "vec_add", n: 2 }, &base).unwrap();
    let _ = log_run_to(&RunLog { example: "vec_add", n: 3 }, &base).unwrap();

    // ganze Datei prüfen
    schema::validate_jsonl_file(&p).unwrap();
}
    */

#[cfg_attr(windows, ignore)]
#[test]
fn schema_v1_accepts_current_writer_output() {
    // Einzigartiges Temp-Verzeichnis pro Lauf (PID + Zeitstempel)
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let base = std::env::temp_dir().join(format!(
        "hpc-core-test-schema-ok-{}-{}",
        std::process::id(),
        nonce
    ));

    fs::create_dir_all(&base).unwrap();

    let p = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 1,
        },
        &base,
    )
    .unwrap();
    let _ = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 2,
        },
        &base,
    )
    .unwrap();
    let _ = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 3,
        },
        &base,
    )
    .unwrap();

    // ganze Datei prüfen
    hpc_core::metrics::schema::validate_jsonl_file(&p).unwrap();
}

#[test]
fn schema_v1_rejects_missing_required_field() {
    // minimal gültige Zeile konstruieren, dann ein Pflichtfeld entfernen
    let mut v = json!({
        "schema_version": schema::SCHEMA_VERSION as u64,
        "ts": "2025-01-01T00:00:00Z",
        "pid": 1234u64,
        "run_id": "20250101-000000-ABCD-1234",
        "example": "vec_add",
        "n": 1u64,
        "allocs": 0u64,
        "alloc_bytes": 0u64
    });

    // Pflichtfeld raus
    v.as_object_mut().unwrap().remove("pid");

    let err = hpc_core::metrics::schema::validate_value(&v).unwrap_err();
    assert!(
        err.contains("pid"),
        "expected error mentioning pid, got: {err}"
    );
}

#[test]
fn schema_v1_allows_unknown_fields() {
    let mut v = json!({
        "schema_version": schema::SCHEMA_VERSION as u64,
        "ts": "2025-01-01T00:00:00Z",
        "pid": 1234u64,
        "run_id": "20250101-000000-ABCD-1234",
        "example": "vec_add",
        "n": 1u64,
        "allocs": 0u64,
        "alloc_bytes": 0u64
    });

    // neues Feld hinzufügen (zukunftige Erweiterung)
    v.as_object_mut()
        .unwrap()
        .insert("new_future_field".into(), json!("ok"));
    // darf valide bleiben
    hpc_core::metrics::schema::validate_value(&v).unwrap();
}

#[test]
fn schema_v1_rejects_wrong_version() {
    let v = json!({
        "schema_version": 999u64, // absichtlich falsch
        "ts": "2025-01-01T00:00:00Z",
        "pid": 1234u64,
        "run_id": "20250101-000000-ABCD-1234",
        "example": "vec_add",
        "n": 1u64,
        "allocs": 0u64,
        "alloc_bytes": 0u64
    });

    let err = hpc_core::metrics::schema::validate_value(&v).unwrap_err();
    assert!(err.contains("schema_version mismatch"), "got: {err}");
}
