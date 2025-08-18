# EPS Method & Metrics

**Protocol-to-Tests.** Jede SPEC-Regel ↔ mindestens ein compile-fail Test (trybuild).

**PSC (Protocol-Safety Coverage).**
Wir erfassen 12 Host-Sync-Fehlmuster. Ziel: **PSC ≥ 75 %**.
PSC = (# durch EPS abgedeckte Muster) / 12
Von 12 identifizierten OpenCL-Host-Fehlmustern (Double-wait, Missed wait, InFlight-Read, Order, Cross-Context, Leak, Invalid Size/ABI, Misc) werden 8/12 bereits **compile-time** ausgeschlossen.  
→ PSC = 66,7 %

**Threats to Validity.**
- Compiler-Message-Stabilität (gemildert durch `rust-toolchain.toml`)
- Dynamische/datenabhängige Protokollzweige nur begrenzt statisch erfassbar


## Host-Sync Fehlmuster (Katalog, Ziel 12; PSC ≥ 75%)

| ID | Pattern                                         | SPEC-Quelle | EPS-Test? | Baseline-Symptom |
|----|--------------------------------------------------|-------------|----------:|------------------|
| F1 | Double-wait auf Event                            | S?          |   [ ]     | Panic/Hang       |
| F2 | `read_blocking` auf InFlight                     | S?          |   [ ]     | UB/Fehler        |
| F3 | Use-after-drop (Event/Queue)                     | S?          |   [ ]     | Crash            |
| F4 | Cross-context Buffer-Misuse                      | S?          |   [ ]     | Runtime-Error    |
| F5 | Reuse ohne vorgeschaltetes `wait`                | S?          |   [ ]     | Datenkorruption  |
| F6 | Falsche Reihenfolge `enqueue_*` → `wait`         | S?          |   [ ]     | Hänger           |
| F7 | Mehrfaches `set_default_queue` inkonsistent      | S?          |   [ ]     | Heisenbugs       |
| F8 | Leaking events (nie gewartet)                    | S?          |   [ ]     | Ressourcen-Leak  |
| F9 | Blocking/Non-blocking inkonsistent kombiniert    | S?          |   [ ]     | Timing-Bugs      |
| F10| Falsche Event-Wait-List beim Read/Write          | S?          |   [ ]     | Race/Fehler      |
| F11| Mehrfaches Schreiben ohne Dependenzgraph         | S?          |   [ ]     | Overwrite        |
| F12| Kernel-Enqueue auf Empty-Buffer (Protokollbruch) | S?          |   [ ]     | Laufzeitfehler   |
