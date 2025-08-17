//! metrics_tail.rs — CLI utility to “tail” the most recent JSONL lines from
//! results/<YYYY-MM-DD>/run.jsonl. Supports line count (-n), pretty-printing,
//! example-based filtering, and optional on-the-fly schema validation.
//!
//! Why so many `#[cfg(feature = "metrics")]`?
//! - This tool and its dependencies (`clap`, `chrono`, `serde_json`, etc.) are
//!   only needed when working with metrics. Gating them keeps the default build
//!   slim and fast.
//! - Some Cargo/test setups also compile binaries during `cargo test`. With the
//!   feature gate, the code is effectively disabled unless `--features metrics`
//!   is set — avoiding unused-import warnings or “main not found” surprises.
//! - Results in reproducible builds across platforms: no metrics I/O code is
//!   compiled unless explicitly requested by enabling the feature.


#[cfg(feature = "metrics")]
use std::{collections::VecDeque, fs, io::{self, BufRead}, path::PathBuf, thread, time::Duration};
#[cfg(feature = "metrics")]
use clap::Parser;
#[cfg(feature = "metrics")]
use anyhow::{Result, Context, anyhow};
#[cfg(feature = "metrics")]
use serde_json::Value;

#[cfg(feature = "metrics")]
#[derive(Parser, Debug)]
#[command(name="metrics_tail", about="Tailt die letzten JSONL-Zeilen aus run.jsonl")]
struct Opt {
    /// Pfad zur JSONL-Datei (Standard: results/<YYYY-MM-DD>/run.jsonl)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Anzahl der Zeilen
    #[arg(short = 'n', long, default_value_t = 20)]
    lines: usize,

    /// Nur Zeilen mit example == … anzeigen
    #[arg(long)]
    example: Option<String>,

    /// JSON hübsch formatieren
    #[arg(long)]
    pretty: bool,

    /// Jede Zeile gegen das aktuelle Schema validieren
    #[arg(long)]
    validate: bool,
}

#[cfg(feature = "metrics")]
fn default_results_file() -> PathBuf {
    use chrono::Local;
    let date = Local::now().format("%Y-%m-%d").to_string();
    PathBuf::from("results").join(date).join("run.jsonl")
}

#[cfg(feature = "metrics")]
fn main() -> Result<()> {
    let opt = Opt::parse();
    let path = opt.file.unwrap_or_else(default_results_file);

    // Schnelles Tail: nicht die ganze Datei halten
    let file = fs::File::open(&path)
        .or_else(|_| { // Windows-Retry beim Open
            thread::sleep(Duration::from_millis(20));
            fs::File::open(&path)
        })
        .with_context(|| format!("open {}", path.display()))?;
    let reader = io::BufReader::new(file);

    let mut buf: VecDeque<String> = VecDeque::with_capacity(opt.lines + 1);
    for line in reader.lines() {
        let l = line?;
        if l.trim().is_empty() { continue; }
        if let Some(ex) = &opt.example {
            // billiger Vorfilter (bevor JSON parse)
            if !l.contains("\"example\"") { continue; }
            if !l.contains(ex) { continue; }
        }
        if buf.len() == opt.lines { buf.pop_front(); }
        buf.push_back(l);
    }

    for l in buf {
        let v: Value = serde_json::from_str(&l)
            .with_context(|| "invalid JSON line")?;

        if opt.validate {
    hpc_core::metrics::schema::validate_value(&v)
        .map_err(|e| anyhow!("schema validation failed: {e}"))?;
}

        if opt.pretty {
            println!("{}", serde_json::to_string_pretty(&v)?);
        } else {
            println!("{}", serde_json::to_string(&v)?);
        }
    }

    Ok(())
}


#[cfg(not(feature = "metrics"))]
fn main() {}