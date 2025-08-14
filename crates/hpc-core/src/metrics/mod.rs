#![cfg(feature = "metrics")]

mod recorder;
pub use recorder::*;

mod run_id;
pub use run_id::current_run_id;

pub mod schema;



use chrono::Local;
use fs2::FileExt;               // für lock_exclusive / unlock
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

// ------------------------------------------------------------
// Public counters
// ------------------------------------------------------------
pub static ALLOCS: AtomicU64 = AtomicU64::new(0);
pub static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

// ------------------------------------------------------------
// Rotation/Retention Parameter
// ------------------------------------------------------------
fn max_bytes() -> u64 {
    std::env::var("HPC_MAX_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5 * 1024 * 1024)
}
fn keep_files() -> usize {
    std::env::var("HPC_KEEP_FILES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7)
}

// ------------------------------------------------------------
#[derive(Debug)]
pub struct RunLog {
    pub example: &'static str,
    pub n: usize,
}

// Für read_last_line (ignoriert zusätzliche Felder im JSON)
#[derive(Debug, Deserialize, Serialize)]
pub struct RunLineOwned {
    pub example: String,
    pub n: usize,
    pub allocs: u64,
    pub alloc_bytes: u64,
    pub ts: String,
    pub schema_version: u8,
    pub pid: u32,
}

pub fn read_last_line(path: &Path) -> io::Result<Option<RunLineOwned>> {
    let s = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };

    if let Some(line) = s.lines().rev().find(|l| !l.trim().is_empty()) {
        let v: RunLineOwned =
            serde_json::from_str(line).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Some(v))
    } else {
        Ok(None)
    }
}

// ------------------------------------------------------------
// Results-Verzeichnis (optional genutzt)
// ------------------------------------------------------------
fn results_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("HPC_RESULTS_DIR") {
        return PathBuf::from(dir);
    }
    let date = Local::now().format("%Y-%m-%d").to_string();
    PathBuf::from("results").join(date)
}

// ------------------------------------------------------------
// Provenienz (einmal pro Prozess)
// ------------------------------------------------------------
#[derive(Serialize)]
struct Provenance {
    exe: String,
    cwd: String,
    argv: Vec<String>,
    hostname: String,
    os: &'static str,
    arch: &'static str,
    version: &'static str,
    build_profile: &'static str,
    git_commit: Option<&'static str>,
    git_branch: Option<&'static str>,
    git_dirty: Option<&'static str>,
}

static PROV: OnceCell<Provenance> = OnceCell::new();

fn provenance() -> &'static Provenance {
    PROV.get_or_init(|| {
        let exe = std::env::current_exe()
            .ok()
            .and_then(|p| p.into_os_string().into_string().ok())
            .unwrap_or_default();
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|p| p.into_os_string().into_string().ok())
            .unwrap_or_default();
        let argv_raw: Vec<String> = std::env::args().collect();
        let argv = mask_args(argv_raw);

        let hostname = get_hostname();
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let version = env!("CARGO_PKG_VERSION");
        let build_profile = option_env!("PROFILE").unwrap_or("release");

        let git_commit = option_env!("GIT_SHA");
        let git_branch = option_env!("GIT_BRANCH");
        let git_dirty = option_env!("GIT_DIRTY");

        Provenance {
            exe,
            cwd,
            argv,
            hostname,
            os,
            arch,
            version,
            build_profile,
            git_commit,
            git_branch,
            git_dirty,
        }
    })
}

fn get_hostname() -> String {
    // minimaler, portabler Ansatz (requires `hostname` crate)
    hostname::get()
        .ok()
        .and_then(|s| s.into_string().ok())
        .unwrap_or_default()
}

fn mask_args(mut args: Vec<String>) -> Vec<String> {
    // simple Redaktion – nach Bedarf verbessern
    let patterns = &["token=", "secret=", "password=", "pwd="];
    for a in &mut args {
        let lower = a.to_lowercase();
        for p in patterns {
            if let Some(pos) = lower.find(p) {
                if let Some(eq) = a[pos..].find('=') {
                    a.replace_range(pos + eq + 1.., "***");
                }
            }
        }
        if a.len() > 256 {
            a.truncate(256);
            a.push_str("…");
        }
    }
    if args.len() > 64 {
        args.truncate(64);
        args.push("…truncated".into());
    }
    args
}

// ------------------------------------------------------------
// Rotation (unter gehaltenem Lock aufrufen!)
// ------------------------------------------------------------
fn rotate_if_needed(path: &Path, max_bytes: u64, keep: usize) -> io::Result<()> {
    use std::fs::{self, OpenOptions};

    if let Ok(md) = fs::metadata(path) {
        if md.len() > max_bytes {
            let dir = path.parent().unwrap();
            let ts = Local::now().format("%Y%m%d-%H%M%S");
            let mut dst = dir.join(format!("run-{ts}.jsonl"));
            let mut suffix = 1;
            while dst.exists() {
                dst = dir.join(format!("run-{ts}-{suffix}.jsonl"));
                suffix += 1;
            }
            fs::rename(path, &dst)?; // atomar (gleiche Partition)
            let _ = OpenOptions::new().create(true).append(true).open(path)?;

            // Retention
            let mut files: Vec<_> = fs::read_dir(dir)?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.starts_with("run-") && s.ends_with(".jsonl"))
                        .unwrap_or(false)
                })
                .collect();

            files.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());
            if files.len() > keep {
                for p in files.drain(0..files.len() - keep) {
                    let _ = std::fs::remove_file(p);
                }
            }
        }
    }
    Ok(())
}

// ------------------------------------------------------------
// Hauptfunktion: JSONL-Write mit Lock, Rotation, Fsync, Provenienz
// ------------------------------------------------------------
pub fn log_run_to<P: AsRef<Path>>(r: &RunLog, base: P) -> io::Result<PathBuf> {
    use std::fs::{self, OpenOptions};

    let dir = base.as_ref().to_path_buf();
    fs::create_dir_all(&dir)?;
    let path = dir.join("run.jsonl");

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    // exklusiver Lock bis nach flush/fsync
    file.lock_exclusive()?;

    // Rotation unter Lock
    rotate_if_needed(&path, max_bytes(), keep_files())?;

    // Provenienz ermitteln
    let p = provenance();

    // JSON-Zeile
    let line = serde_json::json!({
        "schema_version": schema::SCHEMA_VERSION,
        "ts": Local::now().to_rfc3339(),
        "pid": std::process::id(),
        "run_id": current_run_id(),

        "example": r.example,
        "n": r.n as u64,
        "allocs": ALLOCS.load(Ordering::Relaxed),
        "alloc_bytes": ALLOC_BYTES.load(Ordering::Relaxed),

        "exe": p.exe,
        "cwd": p.cwd,
        "argv": p.argv,
        "hostname": p.hostname,
        "os": p.os,
        "arch": p.arch,
        "version": p.version,
        "build_profile": p.build_profile,
        "git": {
            "commit": p.git_commit,
            "branch": p.git_branch,
            "dirty":  p.git_dirty,
        }
    });

    let mut s = serde_json::to_string(&line).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    s.push('\n');

    file.write_all(s.as_bytes())?;
    file.sync_data()?; // Haltbarkeit
    fs2::FileExt::unlock(&file)?;

    Ok(path)
}

pub fn log_run(r: &RunLog) -> io::Result<PathBuf> {
    let base = results_dir();
    log_run_to(r, base)
}

// ------------------------------------------------------------
// Kleine Laufzeit-Zusammenfassung (wie gehabt)
// ------------------------------------------------------------
pub fn summary() {
    // Zeiten gruppieren
    let mut map: HashMap<&str, Vec<u128>> = HashMap::new();
    {
        let mut times = TIMES.lock().unwrap();
        for (name, us) in times.drain(..) {
            map.entry(name).or_default().push(us);
        }
    }

    eprintln!("── metrics summary ──");
    for (name, mut v) in map {
        v.sort_unstable();
        let mean = if v.is_empty() { 0 } else { v.iter().sum::<u128>() / (v.len() as u128) };
        let p95 = if v.is_empty() { 0 } else { v[((v.len() * 95) / 100).saturating_sub(1)] };
        eprintln!("{:<18} mean={:>5} µs   p95={:>5} µs", name, mean, p95);

        if name == "enqueue_write" {
            let total_us: u128 = v.iter().sum();
            if total_us > 0 {
                let gbps = (ALLOC_BYTES.load(Ordering::Relaxed) as f64) / (total_us as f64) / 1e3;
                eprintln!("    ↳ throughput ≈ {:.2} GiB/s", gbps);
            }
        }
    }

    // Allokations-Zähler
    let allocs = ALLOCS.load(Ordering::Relaxed);
    let bytes = ALLOC_BYTES.load(Ordering::Relaxed);
    eprintln!("GPU allocations: {}   ({} MiB)", allocs, bytes / 1024 / 1024);
}
