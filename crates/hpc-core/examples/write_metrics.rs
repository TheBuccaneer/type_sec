// Schreibt eine JSONL-Zeile in <base>/run.jsonl (append).
// Aufruf: cargo run -p hpc-core --example write_metrics --features metrics -- [<base> [example n]]

#[cfg(feature = "metrics")]
fn main() -> std::io::Result<()> {
    use hpc_core::metrics::{RunLog, log_run_to};
    use std::{env, path::PathBuf};

    let base = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        PathBuf::from("results").join(date)
    });

    // Beispielname & n aus CLI, mit Defaults
    let example = env::args().nth(2).unwrap_or_else(|| "vec_add".to_string());
    let n: usize = env::args()
        .nth(3)
        .unwrap_or_else(|| "256".into())
        .parse()
        .unwrap_or(256);

    // RunLog erwartet &'static str → für das kurze Process-Leben ist Box::leak ok
    let ex_static: &'static str = Box::leak(example.into_boxed_str());
    let path = log_run_to(
        &RunLog {
            example: ex_static,
            n,
        },
        &base,
    )?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

#[cfg(not(feature = "metrics"))]
fn main() {
    eprintln!("Build with: --features metrics");
}
