# hpc-core — Safe Host↔GPU Synchronisation

[![CI](https://github.com/TheBuccaneer/type_sec/actions/workflows/ci.yml/badge.svg)](https://github.com/TheBuccaneer/type_sec/actions/workflows/ci.yml)

Dieses Crate implementiert ein **Type-State-Modell** für OpenCL-Puffer.  
Ziel: Fehler in der Host-API (falsches Warten, doppeltes Warten, verfrühter Host-Zugriff, überlappende Writes) werden bereits **zur Compile-Zeit** verhindert.

## Spezifikation & Beweise

- **SPEC.md**: Formale Regeln S1–S3 (Exklusivität, lineares Warten, Ready ⇒ Sichtbarkeit).  
- **SPEC-tests-map.md**: Mapping von SPEC-Regeln auf `trybuild`-Tests.  
-  **Tests**: `RUSTFLAGS="--cfg hpc_core_dev" cargo test -p hpc-core` zeigt, dass Verstöße *nicht kompilieren*, der Happy Path aber *kompiliert*.

Damit ist die zentrale Aussage überprüfbar: **Compile-Time Safety statt Runtime-Heisenbugs.**

## Garantien (SPEC S1–S3)

- **S1 (Exklusivität während InFlight):** Solange ein Buffer `InFlight` ist, können keine widersprüchlichen Kommandos enqueued werden.  
- **S2 (Lineares Warten):** `EventToken` ist linear (`#[must_use]`, `!Copy`), „double wait“ und „vergessenes wait“ sind ausgeschlossen.  
- **S3 (Ready ⇒ Sichtbarkeit):** Host-Zugriffe sind nur in `Ready` erlaubt oder bei blockierenden Host-Kommandos.

## Tests

Dieses Projekt nutzt [`trybuild`](https://crates.io/crates/trybuild), um die Regeln S1–S3 explizit mit **compile-fail** und **compile-pass** Tests zu belegen.

### Alle Tests laufen lassen
```bash
RUSTFLAGS="--cfg hpc_core_dev" cargo test -p hpc-core --tests -- --nocapture
```

### Clippy (Lint-Check für Paper/CI)
```bash
RUSTFLAGS="--cfg hpc_core_dev" cargo clippy -p hpc-core --lib --tests -- -D warnings
```

## Continuous Integration

Das Repo enthält ein [CI-Workflow](../.github/workflows/ci.yml), der bei jedem Push/PR
- die Tests mit `cargo test` ausführt und
- `cargo clippy` für lib+tests erzwingt (keine Warnungen erlaubt).

So ist gewährleistet, dass die SPEC-Regeln jederzeit maschinell überprüfbar sind.