# Session Types – Host-side Protocol (RUST IT)

This note summarizes the host-side protocol we enforce at *compile time* using type states in the `Api` layer of the codebase.

> **Goal**: one-pager that the paper can quote. It explains states, transitions, lifetimes/branding, and the safety box backed by compile-fail tests.

---

## States (type markers)

The high-level API uses *type states* for `DeviceBuffer<'brand, T, S>` where `S` encodes the protocol phase (see `src/buffer/state.rs`). The relevant states for the public API in `src/api` are:

- `Empty` – freshly allocated, not yet initialized for device use.
- `Written` – stable/synchronized w.r.t. host <-> device; usable as kernel arg.
- `Mapped` – buffer is mapped to host memory; device must not access it.
- `InFlight` – part of an outstanding async op (kernel, read, write).

Low-level also defines `Synchronized` but the user-facing `Api` consolidates steady state as `Written`.

---

## State machine (host view)

```
Empty ──(write_* / map_for_write_block)──▶ Written ──(enqueue_kernel / read_non_block / write_non_block)──▶ InFlight
InFlight ── wait(event) ──▶ Written
Mapped ── unmap(token) ──▶ Written
```

**Only exit from `InFlight`** is `wait(event)` (or the helpers that consume the token).

---

## Lifetimes & Branding

`Context<'brand>`, `Queue<'brand>`, `Kernel<'brand>`, `DeviceBuffer<'brand, T, S>` and `EventToken<'brand>` share the *branding lifetime* `'brand` to prevent cross-queue/context mixing at the type level. The brand appears as:

```rust
pub struct Queue<'brand> { /* ... _brand: PhantomData<fn(&'brand ()) -> &'brand ()> */ }
pub struct Kernel<'brand> { /* ... */ }
pub struct Context<'brand> { /* ... */ }
pub struct EventToken<'brand> { /* ... */ }
pub struct DeviceBuffer<'brand, T, S> { /* ... */ }
```

This ensures that a token or buffer from one branded queue cannot be “waited” or used with another queue/kernel by mistake.

---

## Transition reference (selected API surface)

**Creation / Initialization (Empty → …)**
- `map_for_write_block(self, &Queue<'brand>) -> (DeviceBuffer<_, Mapped>, MapToken<'brand>)`  
  → prepares host-side initialization via mapping.
- `write_block(self, &Queue<'brand>, &[T]) -> DeviceBuffer<_, Written>`  
  → blocking write, ends in `Written`.

**Mapped path**
- `MapToken::unmap(self, DeviceBuffer<_, Mapped>) -> Result<DeviceBuffer<_, Written>>`.

**Compute (Written → InFlight)**
- `enqueue_kernel(self, &Queue<'brand>, &Kernel<'brand>, global) -> (DeviceBuffer<_, InFlight>, EventToken<'brand>)`.

**I/O from Written**
- *Reads*  
  - `read_blocking(&self, &Queue<'brand>, out: &mut [T]) -> Result<()>` (stays `Written`).  
  - `read_non_blocking(&self, &Queue<'brand>, out: &mut [T]) -> Result<ReadGuard<'_, 'brand, T>>` → returns a guard + token; buffer enters `InFlight` until waited.
- *Writes*  
  - `write_blocking(&mut self, &Queue<'brand>, &[T]) -> Result<()>` (stays `Written`).  
  - `write_non_blocking(self, &Queue<'brand>, &[T]) -> Result<(DeviceBuffer<_, InFlight>, EventToken<'brand>)>`.

**Synchronisation (InFlight → Written)**
- `EventToken::wait(self, DeviceBuffer<_, InFlight>) -> DeviceBuffer<_, Written>`  
  - `ReadGuard::wait(self, DeviceBuffer<_, InFlight>) -> DeviceBuffer<_, Written>` (consumes the guard & token).

*(Names reflect the `src/api` modules: `device_buffer/empty.rs`, `device_buffer/written/*`, `device_buffer/mapped.rs`, and `util/*`.)*

---

## Safety Box (compile-time guarantees)

- **No Host Access in `InFlight`**: while a buffer is `InFlight`, it cannot be read/written or passed to another kernel; only `wait(event, buf)` is available.
- **Linear Events**: `EventToken<'brand>` and `ReadGuard` are `#[must_use]` and get **consumed** by `wait(…)`; double-wait is prevented by the type system.
- **No Cross-Queue Mixing**: branding `'brand` ties buffers, kernels, queues, and tokens to the same context/queue lineage.
- **Host I/O only from `Written`**: read/write APIs require the buffer not to be `InFlight`.

These are backed by compile-fail tests in the project and the `must_use` annotations in `util/event_token.rs` and companions.

---

## Mini typed trace (end-to-end)

```rust
// Allocate Empty
let buf_e: DeviceBuffer<'brand, f32, Empty> = ctx.create_buffer_elems(1<<20)?;

// Initialize (blocking)
let buf_w: DeviceBuffer<_, f32, Written> = buf_e.write_block(&q, &host_data)?;

// Launch kernel (async)
let (buf_f, evt) = buf_w.enqueue_kernel(&q, &k, global)?; // Written → InFlight

// Synchronize
let buf_w = evt.wait(buf_f); // InFlight → Written

// Read back (blocking) – stays Written
buf_w.read_blocking(&q, &mut out)?;
```

---

## Non-goals (explicitly out-of-scope here)

- Multi-queue/multi-device hazards and cross-queue dependencies.
- Zero-copy/pinning and advanced mapping strategies.
- Global progress guarantees beyond “eventually call `wait` for each token”.

---

**Cite in the paper**: “We encode the host–device protocol as type states (`Empty`, `Written`, `Mapped`, `InFlight`) on `DeviceBuffer<T, S>` with a branding lifetime tying buffers, queues, kernels, and events. Illegal interleavings become *type errors*, enforced by the compiler.”