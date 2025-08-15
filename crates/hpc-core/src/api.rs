//! Minimaler, auskommentierter Skeleton der künftigen Safe-API.
//! Greift nur auf die Zustandsmarker aus `buffer::state` zurück und
//! kollidiert nicht mit dem bestehenden `buffer`-Code.

#![allow(dead_code, unused_imports)]

use core::marker::PhantomData;
use crate::buffer::state::{State, Empty, Ready, InFlight};
// Falls der Typ in deinem Code anders heißt, passe das Alias hier an:
use crate::buffer::GpuBuffer as LlBuffer;

/// Root-Lebensraum. In echt: OpenCL-Context/Devices/Programme.
pub struct Context {
    _opaque: (),
}

/// Befehlswarteschlange, an `Context` gebunden.
/// Lifetime erzwingt: Queue lebt nicht länger als der Context.
pub struct Queue<'ctx> {
    _ctx: &'ctx Context,
}

/// Ein typisierter GPU-Buffer im Host-API-Layer.
/// 
/// # Must Use
/// Nicht stumm verwerfen – ein nicht verwendeter Buffer kann auf ein Leck hindeuten
/// oder einen nicht vollzogenen State-Hop.
#[must_use]
pub struct DeviceBuffer<T, S: State> {
    inner: LlBuffer<S>,
    len_elems: usize,
    _ty: PhantomData<T>,
}

/// Kernel-Handle, an Queue/Context gebunden.
pub struct Kernel<'ctx> {
    _q: PhantomData<&'ctx ()>,
}

/// Ein Event-Token als Resultat eines GPU-Befehls.
/// 
/// # Must Use
/// Muss von [`Queue::wait`] oder einem Äquivalent konsumiert werden.
/// Nicht verwendete Events bedeuten, dass ein Kommando evtl. nie abgeschlossen wurde.
#[must_use]
#[derive(Debug)]
pub struct EventToken<'ctx> {
    _q: PhantomData<&'ctx ()>,
}



// --- Signaturen (nur Skeleton) ------------------------------------------------
impl Context {
    #[cfg(feature = "api-dev")]
    pub fn new() -> Self { Self { _opaque: () } }

    #[allow(clippy::new_without_default)]
    #[cfg(not(feature = "api-dev"))]
    pub fn new() -> Self { unimplemented!() }

    #[cfg(feature = "api-dev")]
    pub fn queue(&self) -> Queue<'_> { Queue { _ctx: self } }

    #[cfg(not(feature = "api-dev"))]
    pub fn queue(&self) -> Queue<'_> { unimplemented!() }


    pub fn create_queue(&self) -> Queue<'_> {
        Queue { _ctx: self }
    }
    
}

impl<'ctx> Queue<'ctx> {

    /// Device → Host (blocking):
    /// Liest den Inhalt eines `DeviceBuffer<T, Ready>` in `out` und blockiert,
    /// bis der Transfer abgeschlossen ist. Der Buffer bleibt `Ready`.
    #[inline]
    pub fn read_blocking<T>(
        &'ctx self,
        _buf: &DeviceBuffer<T, Ready>,
        _out: &mut [T],
    ) {
        // später: low-level clEnqueueReadBuffer + clFinish/Wait
        // Größenprüfung: out.len() == buf.len_elems()
        unimplemented!("Queue::read_blocking<T>: typed blocking read from device")
    }

    #[inline]
    pub fn create_buffer_elems<T>(&'ctx self, _n: usize) -> DeviceBuffer<T, Empty> {
        unimplemented!("Queue::create_buffer_elems: allocate n * size_of::<T>() bytes and wrap")
    }

    /// Lege einen noch leeren Gerätepuffer an.
    pub fn create_buffer<T>(&'ctx self) -> DeviceBuffer<T, Empty> {
        unimplemented!()
    }

/// Host→Device Write: Empty -> Ready.
///
/// # Examples
/// ```no_run
/// use hpc_core::api::Context;
/// let ctx = Context::new();
/// let q = ctx.queue();
/// let buf = q.create_buffer_elems::<u32>(16);
/// let _ready = q.enqueue_write(buf, &[0u32; 16]);
/// ```
///
/// ```compile_fail
/// // Write auf bereits Ready → sollte nicht kompilieren.
/// use hpc_core::api::Context;
/// let ctx = Context::new();
/// let q = ctx.queue();
/// let ready = {
///     let b = q.create_buffer_elems::<u32>(16);
///     q.enqueue_write(b, &[0u32; 16])
/// };
/// let _illegal = q.enqueue_write(ready, &[1u32; 16]);
/// ```
/// 
#[inline]
#[cfg(feature = "api-dev")]
pub fn enqueue_write<T>(
    &'ctx self,
    buf: DeviceBuffer<T, Empty>,
    _data: &[T],
) -> DeviceBuffer<T, Ready> {
    // Dev-Stitching: nur State-Übergang, keine echte Kopie.
    let inner_empty = buf.into_inner();
    // Low-Level: State-Hop über Helper (kein transmute im API-Layer).
    let inner_ready = unsafe { inner_empty.assume_state::<Ready>() };
    DeviceBuffer::from_inner_unchecked(inner_ready)
}

#[inline]
#[cfg(not(feature = "api-dev"))]
pub fn enqueue_write<T>(
    &'ctx self,
    _buf: DeviceBuffer<T, Empty>,
    _data: &[T],
) -> DeviceBuffer<T, Ready> {
    unimplemented!("Queue::enqueue_write<T>: typed write to device buffer")
}

      // Ready -> InFlight + EventToken
    #[cfg(feature = "api-dev")]
    #[inline]
    pub fn enqueue_kernel<T>(
        &'ctx self,
        _k: &Kernel<'ctx>,
        buf: DeviceBuffer<T, Ready>,
    ) -> (DeviceBuffer<T, InFlight>, EventToken<'ctx>) {
        // reiner State-Übergang; ersetzt du später durch echten enqueue
        let inner_ready = buf.into_inner();
        let inner_inflight: LlBuffer<InFlight> = unsafe { inner_ready.assume_state() };
        (DeviceBuffer::from_inner_unchecked(inner_inflight), EventToken { _q: core::marker::PhantomData })
    }

        /// Enqueued einen Kernel und liefert ein [`EventToken`].
    ///
    /// # State-Hop
    /// Eingänge: *Pending* → nach erfolgreichem Lauf: *Ready*.
    ///
    /// # Bemerkung
    /// Diese Funktion **führt** den Kernel irgendwann aus (Scheduling),
    /// das eigentliche Warten passiert über [`Queue::wait`].
    #[cfg(not(feature = "api-dev"))]
    #[inline]
    pub fn enqueue_kernel<T>(
        &'ctx self,
        _k: &Kernel<'ctx>,
        _buf: DeviceBuffer<T, Ready>,
    ) -> (DeviceBuffer<T, InFlight>, EventToken<'ctx>) {
        unimplemented!("Queue::enqueue_kernel<T>: start kernel, return InFlight + EventToken")
    }

   // InFlight -> Ready (Token wird konsumiert)
    #[cfg(feature = "api-dev")]
    #[inline]
    pub fn wait<T>(
        &'ctx self,
        _ev: EventToken<'ctx>,
        buf: DeviceBuffer<T, InFlight>,
    ) -> DeviceBuffer<T, Ready> {
        let inner_inflight = buf.into_inner();
        let inner_ready: LlBuffer<Ready> = unsafe { inner_inflight.assume_state() };
        DeviceBuffer::from_inner_unchecked(inner_ready)
    }

    /// Wartet/blockiert auf ein Event und vollzieht den State-Hop.
    ///
    /// # State-Hop
    /// *Pending* → *Ready*. Auf bereits *Ready* sollte **nicht** gewartet werden
    /// (siehe compile-fail-Tests).
    #[cfg(not(feature = "api-dev"))]
    #[inline]
    pub fn wait<T>(
        &'ctx self,
        _ev: EventToken<'ctx>,
        _buf: DeviceBuffer<T, InFlight>,
    ) -> DeviceBuffer<T, Ready> {
        unimplemented!("Queue::wait<T>: block on event, transition InFlight->Ready")
    }
}

impl<'ctx> Kernel<'ctx> {
    // dev: simpler Platzhalter
    #[cfg(feature = "api-dev")]
    pub fn new(_q: &'ctx Queue<'ctx>, _name: &str) -> Self {
        Self { _q: core::marker::PhantomData }
    }

        /// Erstellt ein Kernel-Handle für die gegebene Queue und Kernel-Bezeichnung.
    ///
    /// # Zweck
    /// *Nur Handle, kein Dispatch.* Der eigentliche Start passiert über
    /// [`Queue::enqueue_kernel`].
    ///
    /// # State-Hop
    /// Vorbereitung -> ausführbar mit `enqueue_kernel`.
    #[inline]
    #[cfg(not(feature = "api-dev"))]
    pub fn new(_q: &'ctx Queue<'ctx>, _name: &str) -> Self { unimplemented!() }
}


impl<T, S: State> core::fmt::Debug for DeviceBuffer<T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DeviceBuffer")
            .field("len_elems", &self.len_elems)
            .finish_non_exhaustive()
    }
}

impl<T, S: State> DeviceBuffer<T, S> {
    /// Interne Konstruktion aus Low-Level-Buffer + Elementlänge.
    /// (Sichtbar im Crate; die High-Level-APIs stellen das später sicher her.)
    pub(crate) fn from_inner(inner: LlBuffer<S>, len_elems: usize) -> Self {
        Self { inner, len_elems, _ty: PhantomData }
    }

    /// Gibt den Low-Level-Buffer wieder zurück (z. B. für interne Delegation).
    pub(crate) fn into_inner(self) -> LlBuffer<S> {
        self.inner
    }

    /// Anzahl Elemente (nicht Bytes).
    #[inline] pub fn len_elems(&self) -> usize { self.len_elems }

    /// Byte-Länge, abgeleitet aus `T`.
    #[inline] pub fn len_bytes(&self) -> usize { self.len_elems * core::mem::size_of::<T>() }

    pub(crate) fn from_inner_unchecked(inner: LlBuffer<S>) -> Self {
    let bytes = inner.len_bytes();
    let sz = core::mem::size_of::<T>();
    debug_assert!(
        bytes % sz == 0,
        "DeviceBuffer<{}>: bytes ({}) nicht durch size_of T ({}) teilbar",
        core::any::type_name::<T>(),
        bytes,
        sz
    );
    let len_elems = bytes / sz;
    Self { inner, len_elems, _ty: PhantomData }
}

}
