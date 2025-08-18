# API Audit — hpc-core

## Scope
Kurzcheck der Public-Surface bzgl. sicherer Zustandsübergänge und Zero-Cost-Eigenschaften.

## Findings

- **`assume_state` Sichtbarkeit**
  - Fundstellen: `api.rs` nutzt `unsafe { ...assume_state() }` intern.
  - Implementierung: `crates/hpc-core/src/buffer/mod.rs` deklariert  
    `pub(crate) unsafe fn assume_state<Target: state::State>(...)`.
  - **Bewertung:** Nur `pub(crate)` → keine Public-Umgehung der Typsicherheit. ✅

- **Einziger legaler Übergang aus `InFlight`**
  - Belege: `buffer/mod.rs` enthält `impl GpuBuffer<InFlight>` mit Kommentar  
    „S3: The only legal transition out of `InFlight`.“
  - API-Design sieht `wait()` als einzigen Pfad zurück nach `Ready` vor.
  - **Bewertung:** Übergang auf `wait()` beschränkt. ✅

- **PhantomData / Typzustände sind Zero-Cost**
  - Vorkommen: `api.rs`, `buffer/mod.rs` verwenden `PhantomData<...>` als Marker.
  - Keine runtime-tragenden Felder; dient nur dem Typensystem.
  - **Bewertung:** Rein compile-time, keine Laufzeitkosten. ✅

## Notes
- Interne `unsafe`-Nutzung (z. B. `assume_state`) ist gekapselt; Caller-seitig sicher.
- ASM-Probes zeigen identische Instruktionssequenzen zwischen Raw/API/Typestate.
- Weitere Arbeit: regelmäßiger Check bei API-Änderungen (pre-merge).

