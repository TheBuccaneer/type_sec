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


## F1–F8 Übersicht (Compile‑Fail Coverage)

| ID | Fehlklasse (kurz)               | Erwartung      | Status |
|----|----------------------------------|----------------|--------|
| F1 | Double‑wait                      | compile‑fail   | ☑     |
| F2 | Missed wait                      | compile‑fail   | ☑     |
| F3 | Read während InFlight            | compile‑fail   | ☑     |
| F4 | Falsche Reihenfolge (Order)      | compile‑fail   | ☑     |
| F5 | Cross‑Context‑Mixing             | compile‑fail   | ☑     |
| F6 | Leak (Ressourcen nicht freigegeben) | compile‑fail | ☑     |
| F7 | Ungültige Größe/ABI              | compile‑fail   | ☑     |
| F8 | Sonstige Host‑Sync‑Fehler        | compile‑fail   | ☑     |

**PSC (prevented spec cases): 8/12 = 66.7%

*Stand: 2025-08-18 10:15*

*Stand: 2025-08-18 10:11*

*Stand: 2025-08-18 10:07*
