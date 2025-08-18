# SPEC ↔ Tests Map (F1–F8)

| ID | OpenCL-Regel (Host)                     | Baseline (erwartet)                      | API-Test (Datei)                         | Erwartung | Status |
|----|-----------------------------------------|-------------------------------------------|------------------------------------------|-----------|--------|
| F1 | Event nur 1× warten (kein Double-wait)  | Runtime-Fehler / UB möglich               | crates/hpc-core/tests/ui/api_double_wait.rs   | compile-fail | ☐ |
| F2 | wait() nicht vergessen                  | Hängt/Leak möglich                        | crates/hpc-core/tests/ui/api_miss_wait.rs     | compile-fail | ☐ |
| F3 | Kein Read während InFlight              | Runtime-Fehler / falsche Daten möglich    | crates/hpc-core/tests/ui/api_inflight_read.rs | compile-fail | ☐ |
| F4 | Befehlsreihenfolge korrekt              | Race/Fehlverhalten                        | crates/hpc-core/tests/ui/api_order.rs         | compile-fail | ☐ |
| F5 | Kein Cross-Context-Mixing               | Runtime-Error                             | crates/hpc-core/tests/ui/api_cross_ctx.rs     | compile-fail | ☐ |
| F6 | Ressourcen korrekt freigeben (Leak=0)   | Leaks                                     | crates/hpc-core/tests/ui/api_leak.rs          | compile-fail | ☐ |
| F7 | Gültige Größen/Layouts (ABI)            | Runtime-Error                             | crates/hpc-core/tests/ui/api_invalid_size.rs  | compile-fail | ☐ |
| F8 | Sonstige Host-Sync-Fehler               | Diverse                                   | crates/hpc-core/tests/ui/api_misc.rs          | compile-fail | ☐ |

_Notizen:_ Die API erzwingt die Regeln compile-time via Typstates/EventToken. Baseline zeigt typisches Fehlverhalten.
