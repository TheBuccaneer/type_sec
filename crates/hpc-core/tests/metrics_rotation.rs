#![cfg(feature = "metrics")]

use std::{fs, path::PathBuf};
use hpc_core::metrics::{RunLog, log_run_to};

#[cfg_attr(windows, ignore)]
#[test]
fn rotates_when_file_too_large() {
    let base = std::env::temp_dir().join("hpc-core-test-rotation");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    // mehrfach schreiben, um Rotation zu erzwingen
    for _ in 0..20_000 {
    let _ = log_run_to(&RunLog { example: "vec_add", n: 42 }, &base).unwrap();
}

    // alle JSONL-Dateien einsammeln
    let files: Vec<PathBuf> = fs::read_dir(&base)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "jsonl").unwrap_or(false))
        .collect();

    assert!(files.len() > 1, "rotation did not produce extra files");

    // Jede Datei muss mit \n enden
    for f in &files {
        let bytes = fs::read(f).unwrap();
        assert!(bytes.ends_with(b"\n"), "{:?} does not end with newline", f);
    }
}
