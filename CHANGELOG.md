# Changelog

Alle wesentlichen Änderungen dieses Projekts werden hier dokumentiert.  
Formatiert nach [Keep a Changelog](https://keepachangelog.com), Semantic Versioning angewendet.

## [Unreleased]
### Hinzugefügt
- Einführung des **EPS-Frameworks** (Executable Protocol Specifications).
- Konkrete **PSC-Metrik** (Protocol-Safety Coverage ≥ 75 % über 12 Host-Sync-Fehlmuster).
- **Microbenchmarks** (Memcpy, VecAdd, Event-Ketten) mit Criterion integriert.
- **Zero-Cost-Nachweise** via `cargo-bloat` & Assembly-Spot-Checks (`cargo-show-asm`).
- **Cross-Mapping-Skizze** für SYCL (Accessoren) und CUDA Graphs.

### Geändert
- Skript `scripts/run_bench.sh` um GPU- und CPU-Stabilisierung ergänzt (Persistence, Clocks, Governor).
- Toolchain-Pinning via `rust-toolchain.toml` zur Versionsstabilisierung.
- Umstrukturierung: Workspace-Layout mit separatem `crates/`, `paper/`, `docs/`, `scripts/`, etc.

### Tests & Specs
- Integration von trybuild-Based **compile-fail-Tests** für 8/12 Fehlermuster.
- `SPEC.md` und `SPEC-tests-map.md` mit präziser Invariantenzuordnung aktualisiert.

### Reproduzierbarkeit
- Neue `docs/reproduce.md` für Reproduktionsschritte.
- DOI-ready Release-Files und Struktur vorbereitet.

## [0.1.0] – 2025-08-25
### Hinzugefügt
- Erstveröffentlichung:
  - Typstate-basierte Host-API für sichere OpenCL-Nutzung.
  - trybuild-Tests für Invarianten (S1–S3).
  - Basis-Microbench-Harness (Stubfiles).
  - Artifact-Ready Repo-Struktur mit DOI/Badge-Support.
