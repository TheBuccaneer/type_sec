#![doc = include_str!("../README.md")]

/// Vollständige Spezifikation (S1–S3) direkt in der Doku.
/// Quelle: `crates/hpc-core/SPEC.md`.
#[doc = include_str!("../SPEC.md")]
pub mod spec {}

#[cfg(feature = "bloat-probe")]
pub mod bloat_typestates_probe {
    use crate::buffer::{
        GpuBuffer,
        state::{Empty, InFlight, Ready},
    };
    use core::hint::black_box;
    use core::mem::MaybeUninit;

    // Reine Typ-Übergänge: keine Reads/Writes, nur "Carry" des uninit-Handles.
    #[inline(always)]
    pub fn to_ready(_buf: MaybeUninit<GpuBuffer<Empty>>) -> MaybeUninit<GpuBuffer<Ready>> {
        // kein Code nötig – der Compiler kann das als NOP sehen
        unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<Ready>>>(_buf) }
    }

    #[must_use]
    pub struct EventToken(core::marker::PhantomData<&'static mut ()>);

    #[inline(always)]
    pub fn enqueue_kernel(
        _buf: MaybeUninit<GpuBuffer<Ready>>,
    ) -> (MaybeUninit<GpuBuffer<InFlight>>, EventToken) {
        let next = unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<InFlight>>>(_buf) };
        (next, EventToken(core::marker::PhantomData))
    }

    #[inline(always)]
    pub fn wait(
        _tok: EventToken,
        _buf: MaybeUninit<GpuBuffer<InFlight>>,
    ) -> MaybeUninit<GpuBuffer<Ready>> {
        unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<Ready>>>(_buf) }
    }

    /// Zweite ASM-Probe: zeigt Empty→Ready→InFlight→Ready ohne Backend/UB.
    #[inline(never)]
    pub extern "C" fn bloat_hotpath_typestates_entry() {
        let empty = MaybeUninit::<GpuBuffer<Empty>>::uninit();
        let ready = to_ready(empty);
        let (inflight, tok) = enqueue_kernel(ready);
        let ready2 = wait(tok, inflight);
        black_box(&ready2);
    }
}

#[cfg(feature = "bloat-probe")]
pub mod bloat_probe {
    use crate::buffer::{GpuBuffer, state::Empty};
    use core::hint::black_box;
    use core::mem::MaybeUninit;

    /// Minimaler ASM-Probe-Einstiegspunkt für Zero-Cost-Nachweis.
    /// Konstruktion wird absichtlich vermieden (uninit), damit kein Drop/UB entsteht.
    /// Minimaler Hotpath für den Zero-Cost-Nachweis (feldgenau).
    #[inline(never)]
    pub extern "C" fn bloat_hotpath_probe_entry() {
        let buf = MaybeUninit::<GpuBuffer<Empty>>::uninit();
        black_box(&buf);
    }
}

#[cfg(feature = "bloat-probe")]
pub use bloat_probe::bloat_hotpath_probe_entry;

// HPC-Core: High-Performance Computing utilities for OpenCL
//
// This crate provides safe wrappers and utilities for GPU computing.

// Core modules (always available)
pub mod api;
mod buffer;
mod error;

// Re-export core types
pub use buffer::state::{InFlight, Queued, Ready, State};
pub use buffer::{GpuBuffer, GpuEventGuard};
pub use error::{ClError, Result};

// Feature-gated modules
#[cfg(feature = "metrics")]
pub mod metrics;

#[cfg(feature = "memtrace")]
pub mod memtracer;
#[cfg(feature = "memtrace")]
pub use memtracer::{
    AbortEvent, AbortTokenGuard, CopyToken, Dir, Operation, TracingScope, clear_abort_token,
    disable_auto_trace, enable_auto_trace, flush_csv, is_auto_trace_enabled, log_abort,
    log_transfer, now_us, reset, set_abort_token, start,
};

// FFI callback for memtrace
#[cfg(feature = "memtrace")]
use std::ffi::c_void;

#[cfg(feature = "memtrace")]
pub extern "C" fn memtrace_callback(
    _record: *const core::ffi::c_void,
    _user_data: *mut core::ffi::c_void,
) {
    // noop stub

    let ctx = crate::Context::dummy(); // ggf. anpassen
    let q = Queue::new(&ctx);

    // minimaler Hotpath: Empty -> Ready -> InFlight -> Wait
    let buf: GpuBuffer<Empty> = GpuBuffer::empty(&ctx, 1024);
    let buf = buf.enqueue_kernel(&q, "vec_add"); // dummy kernel
    let buf = buf.wait(); // blockiert bis fertig

    core::hint::black_box(buf);
}

#[cfg(feature = "bloat-probe")]
pub mod bloat_api_probe {
    use crate::buffer::{
        GpuBuffer,
        state::{Empty, InFlight, Ready},
    };
    use core::hint::black_box;
    use core::mem::MaybeUninit;

    // "API-Schicht" – ruft intern die Raw-Operationen auf (alles inline(always))
    #[inline(always)]
    fn api_to_ready(buf: MaybeUninit<GpuBuffer<Empty>>) -> MaybeUninit<GpuBuffer<Ready>> {
        raw_to_ready(buf)
    }

    #[inline(always)]
    fn api_enqueue_kernel(
        buf: MaybeUninit<GpuBuffer<Ready>>,
    ) -> (MaybeUninit<GpuBuffer<InFlight>>, ApiEventToken) {
        let (b, _t) = raw_enqueue_kernel(buf);
        (b, ApiEventToken)
    }

    #[inline(always)]
    fn api_wait(
        _tok: ApiEventToken,
        buf: MaybeUninit<GpuBuffer<InFlight>>,
    ) -> MaybeUninit<GpuBuffer<Ready>> {
        raw_wait(buf)
    }

    #[must_use]
    struct ApiEventToken;

    // „Raw“ Helfer innerhalb des Moduls – exakt wie in bloat_raw_probe
    #[inline(always)]
    fn raw_to_ready(buf: MaybeUninit<GpuBuffer<Empty>>) -> MaybeUninit<GpuBuffer<Ready>> {
        unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<Ready>>>(buf) }
    }
    #[inline(always)]
    fn raw_enqueue_kernel(
        buf: MaybeUninit<GpuBuffer<Ready>>,
    ) -> (MaybeUninit<GpuBuffer<InFlight>>, ()) {
        let next = unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<InFlight>>>(buf) };
        (next, ())
    }
    #[inline(always)]
    fn raw_wait(buf: MaybeUninit<GpuBuffer<InFlight>>) -> MaybeUninit<GpuBuffer<Ready>> {
        unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<Ready>>>(buf) }
    }

    /// Entry: identisch zum Raw-Pfad, aber via API-Fassade.
    #[inline(never)]
    pub extern "C" fn bloat_hotpath_api_entry() {
        let empty = MaybeUninit::<GpuBuffer<Empty>>::uninit();
        let ready = api_to_ready(empty);
        let (inflight, tok) = api_enqueue_kernel(ready);
        let ready2 = api_wait(tok, inflight);
        black_box(&ready2);
    }
}

#[cfg(feature = "bloat-probe")]
pub mod bloat_raw_probe {
    use crate::buffer::{
        GpuBuffer,
        state::{Empty, InFlight, Ready},
    };
    use core::hint::black_box;
    use core::mem::MaybeUninit;

    #[inline(always)]
    fn to_ready(buf: MaybeUninit<GpuBuffer<Empty>>) -> MaybeUninit<GpuBuffer<Ready>> {
        // Nur Phantom-State wechselt → repr-identisch
        unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<Ready>>>(buf) }
    }

    #[must_use]
    struct EventToken(core::marker::PhantomData<&'static mut ()>);

    #[inline(always)]
    fn enqueue_kernel(
        buf: MaybeUninit<GpuBuffer<Ready>>,
    ) -> (MaybeUninit<GpuBuffer<InFlight>>, EventToken) {
        let next = unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<InFlight>>>(buf) };
        (next, EventToken(core::marker::PhantomData))
    }

    #[inline(always)]
    fn wait(
        _tok: EventToken,
        buf: MaybeUninit<GpuBuffer<InFlight>>,
    ) -> MaybeUninit<GpuBuffer<Ready>> {
        unsafe { core::mem::transmute::<_, MaybeUninit<GpuBuffer<Ready>>>(buf) }
    }

    /// Entry: Empty → Ready → InFlight → Ready, ohne Backend.
    #[inline(never)]
    pub extern "C" fn bloat_hotpath_raw_entry() {
        let empty = MaybeUninit::<GpuBuffer<Empty>>::uninit();
        let ready = to_ready(empty);
        let (inflight, tok) = enqueue_kernel(ready);
        let ready2 = wait(tok, inflight);
        black_box(&ready2);
    }
}
