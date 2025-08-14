// crates/hpc-core/examples/tail_metrics.rs
// Aufruf: cargo run -p hpc-core --example tail_metrics --features metrics -- results/YYYY-MM-DD

#[cfg(feature = "metrics")]
fn main() -> std::io::Result<()> {
    use std::{env, path::PathBuf};
    use hpc_core::metrics::read_last_line;

    let base = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        PathBuf::from("results").join(date)
    });
    let path = base.join("run.jsonl");

    match read_last_line(&path)? {
        Some(line) => {
            // als JSON zurück ausgeben (schön maschinenlesbar)
            println!("{}", serde_json::to_string(&line).unwrap());
        }
        None => {
            eprintln!("(no lines) {}", path.display());
        }
    }
    Ok(())
}

#[cfg(not(feature = "metrics"))]
fn main() {
    eprintln!("Build with --features metrics");
}
