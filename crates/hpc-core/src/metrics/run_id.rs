use once_cell::sync::OnceCell;

static RUN_ID: OnceCell<String> = OnceCell::new();

/// Stabile Lauf-ID für den gesamten Prozess.
/// Format: YYYYMMDD-HHMMSS-<SEED_HEX>-<PID>
pub fn current_run_id() -> &'static str {
    RUN_ID.get_or_init(|| {
        use time::{OffsetDateTime, macros::format_description};
        let now = OffsetDateTime::now_utc();
        let fmt = format_description!("[year][month][day]-[hour][minute][second]");
        let ts = now.format(&fmt).unwrap_or_else(|_| "00000000-000000".to_string());
        let seed = fastrand::u16(0..=u16::MAX);
        let pid = std::process::id();
        format!("{ts}-{:04X}-{pid}", seed)
    })
}
