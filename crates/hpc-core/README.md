# Type-Sec Project (EPS)

## Überblick
Dieses Repo enthält das „Type-Sec“-Projekt mit typzustandsbasierter Host-API für OpenCL. Mit den EPS (Executable Protocol Specifications) sichern wir API-Invarianten zur Compile-Zeit.

## Schnellstart
- **Platzierung der Quellen**:
  - `crates/hpc-core/`: Implementierung & Benchmarks  
  - `docs/`: Reproduktionsanleitung & Background-Notizen  
  - `scripts/`: Bench-Stabilisierungsskript  
  - `paper/`: Papierdokumentation (Draft, Related Work, etc.)

- **Reproduzierbare Benchmarks**:
  ```bash
  SET_PERF_CPU=1 CPU_CORES=2-11 ./scripts/run_bench.sh cargo bench -p hpc-core
