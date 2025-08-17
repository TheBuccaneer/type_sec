use chrono::DateTime;
use serde_json::Value;
use std::{io, path::Path};

/// Aktuelle Schema-Version für Logzeilen
pub const SCHEMA_VERSION: u8 = 1;

/// Prüft eine einzelne JSON-Zeile gegen das V1-Minimalschema.
/// Erlaubt zusätzliche (unbekannte) Felder.
pub fn validate_value(v: &Value) -> Result<(), String> {
    // 1) Pflichtfelder
    let sv = v
        .get("schema_version")
        .and_then(|x| x.as_u64())
        .ok_or("missing or non-integer `schema_version`")?;
    if sv != SCHEMA_VERSION as u64 {
        return Err(format!(
            "schema_version mismatch: got {sv}, expected {}",
            SCHEMA_VERSION
        ));
    }

    let ts = v.get("ts").and_then(|x| x.as_str()).ok_or("missing `ts`")?;
    // RFC3339
    DateTime::parse_from_rfc3339(ts).map_err(|e| format!("invalid `ts` RFC3339: {e}"))?;

    // ints
    v.get("pid")
        .and_then(|x| x.as_u64())
        .ok_or("missing or non-integer `pid`")?;
    v.get("n")
        .and_then(|x| x.as_u64())
        .ok_or("missing or non-integer `n`")?;
    v.get("allocs")
        .and_then(|x| x.as_u64())
        .ok_or("missing or non-integer `allocs`")?;
    v.get("alloc_bytes")
        .and_then(|x| x.as_u64())
        .ok_or("missing or non-integer `alloc_bytes`")?;

    // strings
    let example = v
        .get("example")
        .and_then(|x| x.as_str())
        .ok_or("missing `example`")?;
    if example.is_empty() {
        return Err("`example` must be non-empty".into());
    }
    let run_id = v
        .get("run_id")
        .and_then(|x| x.as_str())
        .ok_or("missing `run_id`")?;
    if run_id.is_empty() {
        return Err("`run_id` must be non-empty".into());
    }

    // 2) Alles andere ignorieren (Forward-Compatibility)
    Ok(())
}

/// Validiert alle nicht-leeren Zeilen einer JSONL-Datei.
pub fn validate_jsonl_file(path: &Path) -> io::Result<()> {
    let s = std::fs::read_to_string(path)?;
    for (i, line) in s.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let v: Value = serde_json::from_str(line).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("line {i}: invalid json: {e}"))
        })?;
        if let Err(msg) = validate_value(&v) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("line {i}: {msg}"),
            ));
        }
    }
    Ok(())
}
