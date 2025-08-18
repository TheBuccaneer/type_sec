fn spec_map_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/api_double_wait.rs");
    t.compile_fail("tests/ui/api_inflight_read.rs");
    // Weitere Fälle (F2,F4..F8) folgen.
}
