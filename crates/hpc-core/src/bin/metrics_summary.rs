use std::{collections::HashMap, fs, io::{self, BufRead}, path::PathBuf, thread, time::Duration};
use clap::Parser;
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(name="metrics_summary", about="Kleine Zusammenfassung aus run.jsonl")]
struct Opt {
    /// Pfad zur JSONL-Datei (Standard: results/<YYYY-MM-DD>/run.jsonl)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Schema prüfen (langsamer)
    #[arg(long)]
    validate: bool,
}

fn default_results_file() -> PathBuf {
    use chrono::Local;
    let date = Local::now().format("%Y-%m-%d").to_string();
    PathBuf::from("results").join(date).join("run.jsonl")
}

fn open_retry(path: &PathBuf) -> io::Result<fs::File> {
    for _ in 0..20 {
        match fs::File::open(path) {
            Ok(f) => return Ok(f),
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                thread::sleep(Duration::from_millis(15));
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(io::ErrorKind::PermissionDenied, "retry timeout"))
}

#[derive(Default)]
struct Agg {
    count: u64,
    allocs: u128,
    bytes: u128,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let path = opt.file.unwrap_or_else(default_results_file);

    let file = open_retry(&path).with_context(|| format!("open {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let mut total: u64 = 0;
    let mut last_ts: Option<String> = None;
    let mut run_ids: std::collections::BTreeSet<String> = Default::default();
    let mut per_example: HashMap<String, Agg> = HashMap::new();

    for line in reader.lines() {
        let l = line?;
        if l.trim().is_empty() { continue; }
        let v: Value = match serde_json::from_str(&l) {
            Ok(v) => v,
            Err(_) => continue, // kaputte Zeile überspringen
        };

        if opt.validate {
            if let Err(e) = hpc_core::metrics::schema::validate_value(&v) {
                eprintln!("skip invalid line: {e}");
                continue;
            }
        }

        total += 1;

        if let Some(ts) = v.get("ts").and_then(|x| x.as_str()) {
            last_ts = Some(ts.to_string());
        }
        if let Some(rid) = v.get("run_id").and_then(|x| x.as_str()) {
            run_ids.insert(rid.to_string());
        }

        let ex = v.get("example").and_then(|x| x.as_str()).unwrap_or("<unknown>").to_string();
        let allocs = v.get("allocs").and_then(|x| x.as_u64()).unwrap_or(0) as u128;
        let bytes  = v.get("alloc_bytes").and_then(|x| x.as_u64()).unwrap_or(0) as u128;

        let e = per_example.entry(ex).or_default();
        e.count += 1;
        e.allocs += allocs;
        e.bytes  += bytes;
    }

    println!("== metrics summary ==");
    println!("file: {}", path.display());
    println!("lines: {}", total);
    println!("unique run_id(s): {}", run_ids.len());
    if let Some(ts) = last_ts { println!("last ts: {ts}"); }

    println!("\nper example:");
    let mut keys: Vec<_> = per_example.keys().cloned().collect();
    keys.sort();
    for k in keys {
        let a = &per_example[&k];
        let avg_allocs = if a.count > 0 { a.allocs as f64 / a.count as f64 } else { 0.0 };
        let avg_bytes  = if a.count > 0 { a.bytes  as f64 / a.count as f64 } else { 0.0 };
        println!(
            "  - {:<16} count={:<6} avg_allocs={:<10.2} avg_bytes={:<12.2}",
            k, a.count, avg_allocs, avg_bytes
        );
    }

    Ok(())
}
