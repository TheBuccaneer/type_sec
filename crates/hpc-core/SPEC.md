# SPEC — Host↔GPU-Synchronisationssicherheit via Typzustände

**Ziel.** Host-seitigen Fehlgebrauch von OpenCL-APIs (Reihenfolge/Synchronisation) zur **Compile-Zeit** ausschließen. Die Regeln leiten sich aus der Event-/Wait-List-Semantik ab und werden mit einem Type-State-Automat erzwungen.

## Threat Model

**Abgesichert (Scope).** Fehlgebrauch auf der Host-Seite:
- falsches/vergessenes Warten (double wait / wait auf falschem Zustand),
- überlappende Schreib-/Konflikt-Kommandos ohne Abhängigkeit,
- verfrühter Host-Zugriff auf Buffer vor Completion.

**Außerhalb (Non-Goals).** Kernel-seitige Datenrennen/Barrier-Divergenz; hierfür verweisen wir auf GPU-Seiten-Analysetools (z. B. GPUVerify).

## Zustandsautomat (Überblick)
Empty → InFlight → Ready

```text
               create_buffer
      ┌────────────────────────────────┐
      │                                ▼
   [ Empty ] --enqueue_write(blocking)--> [ Ready ]
      │                                     │
      │ enqueue_write(nonblocking)          │ read_blocking / map_for_read
      │ or enqueue_kernel(...)              │ (keine Zustandsänderung)
      ▼                                     │
   [ InFlight ] --wait(EventToken)--------> [ Ready ]
      ^               (consumes token)
      │
      └--- (weitere Kommandos können via Event-Weitergabe
            aneinandergekettet werden; kein Host-Zugriff)
```

**Intuition**
- **Empty**: (Re)allokiert/neu; keine ausstehenden Geräte-Kommandos.
- **InFlight**: mind. ein Kommando enqueued; zugehöriges Event ≠ `CL_COMPLETE`.
- **Ready**: alle betreffenden Kommandos abgeschlossen; Host-Sichtbarkeit garantiert (oder Kommando war blocking).

## API-Form (abgeleitet)

- Enqueue: `DeviceBuffer<T, Ready> -> (DeviceBuffer<T, InFlight>, EventToken)`  **(erzwingt S1/S2)**  
- Wait: `wait(EventToken, DeviceBuffer<T, InFlight>) -> DeviceBuffer<T, Ready>` **(erzwingt S2/S3)**  
- Host-Read/Map nur mit `&DeviceBuffer<T, Ready>` **(erzwingt S3)**  
- `#[must_use]` auf Übergangs-APIs & Token; Token **nicht** `Copy`.

## Invarianten (S1–S3)

**S1 — Exklusivität während InFlight**  
- **MUST NOT:** Neue Kommandos, die denselben Buffer lesen/schreiben, **ohne** Synchronisation enqueuen, solange er `InFlight` ist.  
- **MUST:** Abhängigkeiten explizit über das **Event** herstellen (Wait-List oder `wait()`).

**S2 — Korrektes Warten (linearer Token)**  
- **MUST:** `wait()` konsumiert einen **linearen** `EventToken` (`!Copy`, `#[must_use]`) und ist nur für `InFlight` definiert.  
- **MUST NOT:** zweites `wait()` auf dasselbe Event.

**S3 — Ready ⇒ Sichtbarkeit & Benutzbarkeit**  
- **MUST:** Host-Zugriffe (Read/Map/weitere Enqueues) erst in `Ready` **oder** bei blockierender Host-Operation.  
- **MUST NOT:** auf Ergebnisse zugreifen, bevor `CL_COMPLETE` erreicht **und** synchronisiert ist.

## Mapping zu Tests
Siehe [`SPEC-tests-map.md`](SPEC-tests-map.md).

## Referenzen

\[R1] OpenCL 3.0 Unified Spec — **Event Wait Lists** & Command Execution Model  
<https://registry.khronos.org/OpenCL/specs/3.0-unified/html/OpenCL_API.html>

\[R2] `clEnqueueReadBuffer` — blocking vs. non-blocking; Event-Rückgabe und Nutzbarkeit der Daten erst nach Completion  
<https://registry.khronos.org/OpenCL/sdk/3.0/docs/man/html/clEnqueueReadBuffer.html>

\[R3] `clGetEventInfo` — **kein** Synchronisationspunkt: `CL_COMPLETE` abfragen garantiert keine Sichtbarkeit für andere Kommandos  
<https://registry.khronos.org/OpenCL/sdk/3.0/docs/man/html/clGetEventInfo.html>

\[R4] OpenCL 3.0 Reference Guide (PDF) — Event Objects (`clWaitForEvents`, User Events)  
<https://www.khronos.org/files/opencl30-reference-guide.pdf>

\[R5] GPUVerify — statische Verifikation von Race-/Divergence-Freedom  
Paper: <https://www.doc.ic.ac.uk/~afd/papers/2012/OOPSLA.pdf>  
Repo:  <https://github.com/mc-imperial/gpuverify>

## Design Rationale (kurz)

Warum Typzustände statt Runtime-Checks?

1. **Früher Abbruch:** Fehlgebrauch wird bereits beim Kompilieren sichtbar (keine Heisen-Bugs zur Laufzeit).  
2. **Lineare Fähigkeiten:** Das Event wird als **linearer Token** modelliert; „double wait“/„forgot to wait“ sind typsystematisch ausgeschlossen.  
3. **API-Leitplanken statt Konventionen:** Nur erlaubte Übergänge sind überhaupt aufrufbar (`Ready → InFlight + EventToken`, `wait(EventToken, InFlight) → Ready`).  
4. **Kostenfrei im Hot-Path:** Der Overhead liegt im Typchecker, nicht im Kernel-Pfad.

## Glossar (Mini)

- **Event:** Sync-Objekt eines Kommandos; Status u. a. `CL_COMPLETE`.  
- **Event-Wait-List:** Abhängigkeitsliste, die den **Start** eines Kommandos steuert.  
- **Blocking/Non-blocking:** Blockierende Host-IO wartet auf Completion; nicht-blockierend liefert ein Event zur späteren Synchronisation.