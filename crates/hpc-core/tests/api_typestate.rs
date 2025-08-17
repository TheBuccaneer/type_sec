#[test]
fn api_compile_fail_and_pass() {
    use std::path::PathBuf;

    // compile-fail: absoluter, Windows-sicherer Glob
    let cf_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("compile-fail");
    assert!(
        cf_dir.is_dir(),
        "compile-fail directory not found: {:?}",
        cf_dir
    );
    let cf_pattern = format!("{}/{}", cf_dir.to_string_lossy().replace('\\', "/"), "*.rs");

    // compile-pass: absoluter, Windows-sicherer Glob
    let cp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("compile-pass");
    let _ = std::fs::create_dir_all(&cp_dir); // falls Ordner noch nicht existiert
    let cp_pattern = format!("{}/{}", cp_dir.to_string_lossy().replace('\\', "/"), "*.rs");

    let t = trybuild::TestCases::new();
    t.compile_fail(&cf_pattern);
    t.pass(&cp_pattern);
}
