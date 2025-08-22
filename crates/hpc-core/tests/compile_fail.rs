#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    // Alle .rs Dateien im Unterordner ausführen
    t.compile_fail("tests/compile_fail/*.rs");
}