// crates/hpc-core/tests/metrics_log_smoke.rs

#[cfg(feature = "metrics")]
fn unique_results_dir(suffix: &str) -> std::path::PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("hpc-core-test-{}-{}", suffix, ts))
}

#[cfg(feature = "metrics")]
fn read_nonempty_lines(p: &std::path::Path) -> Vec<String> {
    use std::{fs, io::Read};
    let mut s = String::new();
    if let Ok(mut f) = fs::File::open(p) {
        let _ = f.read_to_string(&mut s);
    }
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect()
}

#[cfg(feature = "metrics")]
#[test]
fn jsonl_writer_appends_one_line() {
    use hpc_core::metrics::{RunLog, log_run_to};
    use std::{fs, io::Read};

    let base = unique_results_dir("append");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    // 1. Write
    let p1 = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 256,
        },
        &base,
    )
    .expect("first write failed");
    // 2. Write (Append, gleicher Pfad)
    let p2 = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 512,
        },
        &base,
    )
    .expect("second write failed");
    assert_eq!(p1, p2, "wrote to different files");

    let mut s = String::new();
    std::fs::File::open(&p1)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let lines: Vec<_> = s
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines.len(),
        2,
        "expected exactly 2 JSONL lines after two writes"
    );

    for (i, line) in lines.iter().enumerate() {
        let v: serde_json::Value =
            serde_json::from_str(line).expect(&format!("line {} not valid JSON", i));
        assert!(v.get("example").is_some());
        assert!(v.get("n").is_some());
        assert!(v.get("allocs").is_some());
        assert!(v.get("alloc_bytes").is_some());
    }
}

#[cfg(feature = "metrics")]
#[test]
fn jsonl_line_has_counters() {
    use hpc_core::metrics::{RunLog, log_run_to};
    use std::{fs, io::Read};

    let base = unique_results_dir("parse");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let p = log_run_to(
        &RunLog {
            example: "vec_add",
            n: 1,
        },
        &base,
    )
    .expect("write failed");

    let mut s = String::new();
    std::fs::File::open(&p)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let lines = read_nonempty_lines(&p);
    assert_eq!(lines.len(), 1, "expected 1 line");

    let v: serde_json::Value = serde_json::from_str(&lines[0]).unwrap();
    assert!(v["allocs"].is_u64() || v["allocs"].is_i64() || v["allocs"].is_number());
    assert!(v["alloc_bytes"].is_u64() || v["alloc_bytes"].is_i64() || v["alloc_bytes"].is_number());
}
