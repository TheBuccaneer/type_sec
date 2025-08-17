#![cfg(feature = "metrics")]

use hpc_core::metrics::{RunLog, log_run_to};
use serde_json::Value;
use std::{fs, sync::Arc, thread, time::Duration};

#[cfg_attr(windows, ignore)]
#[test]
fn concurrent_appends_are_consistent() {
    let base =
        std::env::temp_dir().join(format!("hpc-core-test-concurrency-{}", std::process::id()));
    let threads = 6usize;
    let per = 50usize;

    let base = Arc::new(base);
    let mut handles = Vec::new();

    for t in 0..threads {
        let base = Arc::clone(&base);
        handles.push(thread::spawn(move || {
            for i in 0..per {
                if (t + i) % 7 == 0 {
                    thread::sleep(Duration::from_millis(1));
                }
                // Fix 1: n als usize
                let _ = log_run_to(
                    &RunLog {
                        example: "vec_add",
                        n: i as usize,
                    },
                    &*base,
                )
                .unwrap();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let path = base.join("run.jsonl");
    let bytes = fs::read(&path).expect("read run.jsonl");
    assert!(bytes.ends_with(b"\n"), "file should end with newline");

    // Zeilen zählen (ohne leere am Ende)
    let lines: Vec<&[u8]> = bytes
        .split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(lines.len(), threads * per, "unexpected number of lines");

    // Jede Zeile ist valides JSON und hat eine run_id
    let mut run_id: Option<String> = None;
    for l in &lines {
        let v: Value = serde_json::from_slice(l).expect("valid JSONL line");
        let rid = v
            .get("run_id")
            .and_then(|x| x.as_str())
            .expect("run_id present");

        if let Some(r0) = &run_id {
            assert_eq!(rid, r0, "run_id must be stable per process");
        } else {
            run_id = Some(rid.to_string());
        }
    }
}

#[cfg_attr(windows, ignore)]
#[test]
fn last_byte_is_newline() {
    let base = std::env::temp_dir().join("hpc-core-test-newline");
    let p = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 1,
        },
        &base,
    )
    .unwrap();
    let bytes = std::fs::read(p).unwrap();
    assert!(bytes.ends_with(b"\n"));
}
