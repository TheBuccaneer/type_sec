# SPEC ↔ Tests (trybuild-Mapping)

| Testfall               | Fehlgebrauch (Kurz)                      | Erwartung     | Invariante |
|-------------------------|------------------------------------------|---------------|------------|
| api_inflight_write.rs   | Enqueue auf Buffer noch `InFlight`       | compile-fail  | S1         |
| api_double_write.rs     | Zweites Write/Kernel ohne Sync           | compile-fail  | S1         |
| api_kernel_on_empty.rs  | Kernel-Start auf `Empty`-Buffer          | compile-fail  | S1         |
| api_wait_on_ready.rs    | `wait()` auf `Ready`                     | compile-fail  | S2, S3     |
| ready_cannot_wait.rs    | `wait()` ist für `Ready` nicht definiert | compile-fail  | S2, S3     |
| api_double_wait.rs      | `EventToken` zweimal genutzt             | compile-fail  | S2         |
| api_happy_path.rs       | Richtige Sequenz inkl. `wait()`          | compile-pass  | S1–S3      |
