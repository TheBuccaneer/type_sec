# Methods

## 1. Protocol Specification & EPS Framework
**Was:** Deployment der SPEC.md-Invarianten als EPS (Executable Protocol Specifications) über `trybuild`-Compile-Fail-Tests.  
**Ziel:** Protocol-Safety Coverage (PSC) über 12 Host-Sync-Fehlmuster (Ziel: ≥ 75 %).

## 2. Fault Pattern Taxonomy & Test Mapping
- Tabelle der 12 Fehlermuster (IDs F1–F12 mit kurzer Beschreibung).
- Wie die `SPEC-tests-map.md` die Invarianten abbildet.

## 3. Benchmark Setup
We designed three microbenchmarks to evaluate both performance overhead and protocol safety:

- **Memcpy (Host↔Device)**: Measures pure data transfer throughput.  
  - `ocl_mem.rs` (Criterion benchmark).  
  - Transfers of 1 MB–512 MB with pinned memory.  
  - Baseline: raw OpenCL vs. wrapped API.

- **Vector Add (Host-Kernel)**: Measures kernel dispatch overhead.  
  - `ocl_vecadd.rs`.  
  - Vectors of 10⁶–10⁸ elements.  
  - Measures execution time and throughput.

- **Event Chain Wait**: Evaluates synchronization overhead.  
  - `ocl_events.rs`.  
  - Chains of 10–100 dependent kernels.  
  - Baseline: raw OpenCL `clWaitForEvents` vs. type-safe wrapper.

  ### Tooling & Execution Environment

- **Criterion** (`cargo bench`) with 10 warmup runs and 100 measurement runs.  
- **System stabilization** before benchmarks:
  - GPU Persistence Mode (`nvidia-smi -pm 1`).  
  - Fixed GPU clocks via `nvidia-smi -lgc`.  
  - Constant power cap (`nvidia-smi -pl`).  
  - CPU governor set to performance (`cpupower frequency-set -g performance`).  
  - Benchmarks pinned to dedicated CPU cores (`taskset`).  
- Automation via `scripts/run_bench.sh` and Justfile targets.

### Output & Data Collection

- Criterion automatically generates:
  - JSON reports (`target/criterion/**/raw.json`).  
  - HTML reports (`target/criterion/report/index.html`).  
- Raw results exported to `results/YYYY-MM-DD/`.  
- Collected metrics: mean runtime, standard deviation, throughput.

## 4. Zero-Cost Validation

### Tools
- **cargo-bloat** (Binärgrößen & Top-Symbole)
- **cargo show-asm** (ASM-Spotchecks ausgewählter Hot-Path-Funktionen)

### Ziel
Belegen, dass die Typstate-Schicht keinen messbaren Hot-Path-Overhead verursacht.

### Vorgehen
1) **Binärgröße pro Bench-Target**:
   - `cargo bloat --release --bench ocl_mem -n 30`
   - `cargo bloat --release --bench ocl_vecadd -n 30`
   - `cargo bloat --release --bench ocl_events -n 30`
   Die Reports werden in `docs/zero_cost.md` zusammengefasst (Datum, Commit-Hash).

2) **ASM-Spotchecks**:
   - Für 1–2 kritische Funktionen (Wrapper/Hot-Path) den generierten ASM-Code ausgeben:
     - `cargo show-asm --bench ocl_mem --rust <vollqualifizierter::pfad::zu::fn>`
   - Erwartung: Monomorphisierung/Inlining, **keine** zusätzlichen Branches/Checks im Hot-Path.

### Metriken & Akzeptanzkriterien
- **ΔGröße** zwischen Bench-Targets ohne/mit Wrappercode ≲ **2 %**
- **ASM-Spotcheck** zeigt keine extra Sprünge/Checks im kritischen Pfad

## 5. Cross-API Mapping & Limitations
- Skizze: SYCL Accessors vs. our EPS, CUDA Graphs Abhängigkeitsmodell vs. Protokollsicherheit.  
- Grenzen der EPS-Ansatzes (z. B. dynamische Protokolle, Compiler Message Stability).

## 6. Reproducibility & Artifact Support
- `run_bench.sh`, `docs/reproduce.md`.  
- Criterion-Reports, JSON, CSV, HTML.  
- Zenodo/CI-Badge-Ready Artefaktstruktur.
