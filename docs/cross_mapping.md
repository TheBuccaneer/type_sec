
Ziel: kompakte Gegenüberstellung von **Scope**, **Guarantees**, **Evidence**, **Kostenmodell** für die drei Ansätze. (vgl. Arbeitsplan).  

## Vergleichstabelle

| System           | Scope (Abstraktion)                         | Guarantees (Sicherheit)                    | Evidence (Belege)                     | Kostenmodell / Overhead            |
|------------------|---------------------------------------------|--------------------------------------------|---------------------------------------|------------------------------------|
| SYCL (Accessor)  | Puffer + Accessor-Objekte mit Access-Modes (read/write); Kommandogruppen | Statische Datenzugriffs-Disziplin (Access-Modes), Dependenzen via Runtime-Scheduler; keine formalen Host-Transitions | Spec- | SYCL (Accessor)  |                                            |                                            |                                       |                                    | Runtime-Belege; Beispiele im Paper; Abgleich mit unseren Misuse-Klassen | Overhead abhängig vom Scheduler; Host-API abstrahiert, aber vergleichbar low; unsere Zero-Cost-Schicht bleibt erhalten |
| SYCL (USM)       | Unified Shared Memory (malloc/free, Pointer-Semantik) | Keine statischen Garantien; Safety analog zu C++-Pointern, nur Runtime-Errors möglich | SYCL Spec, Demos; keine compile-time Barrieren | Overhead minimal (direkte Pointer), aber Host-Misuse sehr leicht möglich; unser Typstate-Ansatz schließt diese Klasse explizit aus |
| CUDA Graphs      | Host baut Launch-Graph (Nodes, Dependencies), dann 1 Call | Formale Dependenzstruktur im Graph; trotzdem keine Host-Transitions/Typsicherheit | NVIDIA Graph-Model + Beispiele; Paper-Notizen als Evidence | Overhead meist geringer durch gebündelte Launches; Sicherheitsgarantien schwächer als unser Compile-Fail-Ansatz |
| OpenCL (Host)    | Host-API (Queues, Buffers, Events); manuelle Sequenzierung am Host | Keine statischen Garantien; Misuse möglich (Double-wait, Read während InFlight, Reihenfolgefehler) | Spez-Anker: blocking/non-blocking, event wait list; unsere SPEC/trybuild-Belege (compile-fail) | Roh-Kosten minimal; unsere Typstate-Schicht bleibt zero-cost (ASM/bloat) |

## Notizen (Stichpunkte für Paper)
- Terminologie angleichen (Queue/Stream, Event/Graph-Node, Accessor/USM).
- „Einziger legaler Übergang“ analog zu unseren Typ-States markieren (Empty→Ready→InFlight→wait).
- Wo existieren Host-seitige Misuse-Klassen? (Double-wait, Read während InFlight, …)
- Verweise in `paper/related-work.md` setzen.

## ToDo
- 4–6 Quellen sammeln (Related Work Abschnitt).
- Beispiele/Code-Snippets minimal (je 3–5 Zeilen) für jeden Ansatz.
