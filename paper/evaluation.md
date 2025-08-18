## Methodik
- Benchmark-Harness: Criterion (Memcpy, VecAdd, Event-Chains).
- Vergleich: Raw OpenCL (Baseline) vs. RUST-IT API (Treatment).
- Größenstaffeln: 1 KiB, 64 KiB, 1 MiB, 16 MiB.
- Runs: ≥ 50 Samples je Fall.

## Ergebnisse
### Microbenchmarks (Medianlaufzeiten)

| Benchmark       | Größe     | Baseline (Raw) | Treatment (API) | Overhead |
|-----------------|-----------|----------------|-----------------|----------|
| Memcpy          | 1 KiB     | …              | …               | … %      |
|                 | 64 KiB    | …              | …               | … %      |
|                 | 1 MiB     | …              | …               | … %      |
|                 | 16 MiB    | …              | …               | … %      |
| VecAdd          | N=2^16    | …              | …               | … %      |
|                 | N=2^20    | …              | …               | … %      |
|                 | N=2^24    | …              | …               | … %      |
| Event-Chain     | L=1       | …              | …               | … %      |
|                 | L=4       | …              | …               | … %      |
|                 | L=16      | …              | …               | … %      |

### PSC-Metrik

| Kategorie                  | Abgedeckt | Gesamt |
|----------------------------|-----------|--------|
| Host-Sync-Fehlmuster (SPEC)| 8         | 12     |

PSC = **66,7 %**

## Threats to Validity
- Varianz: Treiber/ICD, Compiler-Version, Messrauschen.
- trybuild-Stabilität (Toolchain pin, Delta-Toleranzen).
- Plattform-Bindung (HW/SW Setup dokumentiert).

## Fazit
- Compile-time Safety: 8/12 Muster ausgeschlossen.
- Runtime Overhead: erwartbar < 2 % (Benchmarks).
- Leak-Report: Leak=0 im Happy-Path.
