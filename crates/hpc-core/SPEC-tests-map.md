# SPEC ↔ Tests (trybuild-Mapping)

| Testfall                    | Fehlgebrauch (Kurz)                     | Erwartung         | Invariante |
|----------------------------|-----------------------------------------|-------------------|------------|
| inflight_write.rs          | Enqueue auf Buffer noch `InFlight`      | compile-fail      | S1         |
| double_write.rs            | Zweites Write/Kernel ohne Sync          | compile-fail      | S1         |
| wait_on_ready.rs           | `wait()` auf `Ready`                    | compile-fail      | S2, S3     |
| double_wait.rs             | `EventToken` zweimal genutzt            | compile-fail      | S2         |
| ready_cannot_wait.rs       | `wait()` existiert nicht für `Ready`    | compile-fail      | S2, S3     |
| compile_pass_happy.rs      | Richtige Sequenz incl. Wait             | compile-pass      | S1–S3      |
