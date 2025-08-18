//! Minimaler, auskommentierter Skeleton der künftigen Safe-API.
//! Greift nur auf die Zustandsmarker aus `buffer::state` zurück und
//! kollidiert nicht mit dem bestehenden `buffer`-Code.

#![allow(dead_code, unused_imports)]

use crate::buffer::state::{Empty, InFlight, Ready, State};
use core::marker::PhantomData;
// Falls der Typ in deinem Code anders heißt, passe das Alias hier an:
use crate::buffer::GpuBuffer as LlBuffer;
use opencl3::event::Event as ClEvent;

/// Root-Lebensraum. In echt: OpenCL-Context/Devices/Programme.
pub struct Context<'id> {
    _opaque: (),
    _id: core::marker::PhantomData<&'id ()>,
}

/// Befehlswarteschlange, an `Context` gebunden.
/// Lifetime erzwingt: Queue lebt nicht länger als der Context.
pub struct Queue<'id, 'ctx> {
    _ctx: &'ctx Context<'id>,
}

/// Ein typisierter GPU-Buffer im Host-API-Layer.
///
/// # Must Use
/// Nicht stumm verwerfen – ein nicht verwendeter Buffer kann auf ein Leck hindeuten
/// oder einen nicht vollzogenen State-Hop.
#[must_use]
pub struct DeviceBuffer<'id, 'ctx, T, S: State> {
    inner: LlBuffer<S>,
    len_elems: usize,
    _ty: PhantomData<T>,
    _ctx: PhantomData<&'ctx Context<'id>>,
}

/// Kernel-Handle, an Queue/Context gebunden.
pub struct Kernel<'id, 'ctx> {
    _q: PhantomData<&'ctx Queue<'id, 'ctx>>,
}

/// Ein Event-Token als Resultat eines GPU-Befehls.
///
/// # Must Use
/// Muss von [`Queue::wait`] oder einem Äquivalent konsumiert werden.
/// Nicht verwendete Events bedeuten, dass ein Kommando evtl. nie abgeschlossen wurde.
#[cfg(hpc_core_dev)]
#[must_use]
#[derive(Debug)]
pub struct EventToken<'ctx> {
    // Im Dev-Stub KEIN echtes OpenCL-Event
    _q: core::marker::PhantomData<&'ctx ()>,
}

#[cfg(not(hpc_core_dev))]
#[must_use]
#[derive(Debug)]
pub struct EventToken<'ctx> {
    // Echtes Event nur im Normalbetrieb
    evt: ClEvent,
    _q: core::marker::PhantomData<&'ctx ()>,
}

#[cfg(not(hpc_core_dev))]
impl<'ctx> EventToken<'ctx> {
    /// Konstruiere aus realem OpenCL-Event
    #[inline]
    pub fn from_event(evt: ClEvent) -> Self {
        Self {
            evt,
            _q: core::marker::PhantomData,
        }
    }

    /// Event extrahieren (konsumiert den Token)
    #[inline]
    #[must_use]
    pub fn into_event(self) -> ClEvent {
        self.evt
    }
}

#[cfg(hpc_core_dev)]
impl<'ctx> EventToken<'ctx> {
    /// Dev-Stub: Token ohne echtes Event
    #[inline]
    pub fn dev() -> Self {
        Self {
            _q: core::marker::PhantomData,
        }
    }
}

// --- Signaturen (nur Skeleton) ------------------------------------------------
impl<'id> Context<'id> {
    #[cfg(hpc_core_dev)]
    pub fn new() -> Self {
        Self { 
            _opaque: (),
            _id: core::marker::PhantomData,
        }
    }

    #[allow(clippy::new_without_default)]
    #[cfg(not(hpc_core_dev))]
    pub fn new() -> Self {
        unimplemented!()
    }

    #[cfg(hpc_core_dev)]
    pub fn queue(&self) -> Queue<'id, '_> {
        Queue { _ctx: self }
    }

    #[cfg(not(hpc_core_dev))]
    pub fn queue(&self) -> Queue<'id, '_> {
        unimplemented!()
    }

    pub fn create_queue(&self) -> Queue<'id, '_> {
        Queue { _ctx: self }
    }
}

impl<'id> Default for Context<'id> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'id, 'ctx> Queue<'id, 'ctx> {
    /// Device → Host (blocking):
    /// Liest den Inhalt eines `DeviceBuffer<'id, 'ctx, T, Ready>` in `out` und blockiert,
    /// bis der Transfer abgeschlossen ist. Der Buffer bleibt `Ready`.
    #[inline]
    pub fn read_blocking<T>(&'ctx self, _buf: &DeviceBuffer<'id, 'ctx, T, Ready>, _out: &mut [T]) 
    where
        'ctx: 'id,
    {
        // später: low-level clEnqueueReadBuffer + clFinish/Wait
        // Größenprüfung: out.len() == buf.len_elems()
        unimplemented!("Queue::read_blocking<T>: typed blocking read from device")
    }

    pub fn new(ctx: &'ctx Context<'id>) -> Self {
        Queue { _ctx: ctx }
    }

    #[inline]
    pub fn create_buffer_elems<T>(&'ctx self, _n: usize) -> DeviceBuffer<'id, 'ctx, T, Empty> 
    where
        'ctx: 'id,
    {
        unimplemented!("Queue::create_buffer_elems: allocate n * size_of::<T>() bytes and wrap")
    }

    /// Lege einen noch leeren Gerätepuffer an.
    pub fn create_buffer<T>(&'ctx self) -> DeviceBuffer<'id, 'ctx, T, Empty> 
    where
        'ctx: 'id,
    {
        unimplemented!()
    }

    #[inline]
    #[cfg(hpc_core_dev)]
    pub fn enqueue_write<T>(
        &'ctx self,
        buf: DeviceBuffer<'id, 'ctx, T, Empty>,
        _data: &[T],
    ) -> DeviceBuffer<'id, 'ctx, T, Ready> 
    where
        'ctx: 'id,
    {
        // Dev stub: only type-state hop (no real copy)
        let inner_empty = buf.into_inner();
        let inner_ready: LlBuffer<Ready> = unsafe { inner_empty.assume_state::<Ready>() };
        DeviceBuffer::from_inner_unchecked(inner_ready)
    }

    #[inline]
    #[cfg(not(hpc_core_dev))]
    pub fn enqueue_write<T>(
        &'ctx self,
        _buf: DeviceBuffer<'id, 'ctx, T, Empty>,
        _data: &[T],
    ) -> DeviceBuffer<'id, 'ctx, T, Ready> 
    where
        'ctx: 'id,
    {
        unimplemented!("Queue::enqueue_write<T>: typed write to device buffer")
    }

    // Ready -> InFlight + EventToken
    #[cfg(hpc_core_dev)]
    #[inline]
    pub fn enqueue_kernel<T>(
        &'ctx self,
        _k: &Kernel<'id, 'ctx>,
        buf: DeviceBuffer<'id, 'ctx, T, Ready>,
    ) -> (DeviceBuffer<'id, 'ctx, T, InFlight>, EventToken<'ctx>) 
    where
        'ctx: 'id,
    {
        // Dev stub: only type-state hop (no real enqueue/event)
        let inner_ready = buf.into_inner();
        let inner_inflight: LlBuffer<InFlight> = unsafe { inner_ready.assume_state() };
        (
            DeviceBuffer::from_inner_unchecked(inner_inflight),
            // phantom token (dev stub has no real event)
            EventToken::dev(),
        )
    }

    #[cfg(not(hpc_core_dev))]
    #[inline]
    pub fn enqueue_kernel<T>(
        &'ctx self,
        _k: &Kernel<'id, 'ctx>,
        _buf: DeviceBuffer<'id, 'ctx, T, Ready>,
    ) -> (DeviceBuffer<'id, 'ctx, T, InFlight>, EventToken<'ctx>) 
    where
        'ctx: 'id,
    {
        unimplemented!("Queue::enqueue_kernel<T>: enqueue kernel and return InFlight + event token")
    }

    #[cfg(hpc_core_dev)]
    #[inline]
    pub fn wait<T>(
        &self,
        _ev: EventToken<'ctx>, // nur Marker im dev-Stub
        buf: DeviceBuffer<'id, 'ctx, T, InFlight>,
    ) -> DeviceBuffer<'id, 'ctx, T, Ready> {
        let inner_inflight = buf.into_inner();
        // dev-Stub: kein echtes Warten – nur Type-State-Hop
        let inner_ready: LlBuffer<Ready> = unsafe { inner_inflight.assume_state() };
        DeviceBuffer::from_inner_unchecked(inner_ready)
    }

    /// S3: Typed transition `InFlight -> Ready`.
    /// Consumes the event token and the in-flight buffer; after waiting, returns `Ready`.
    /// This prevents double-wait by taking the buffer by value.
    #[cfg(not(hpc_core_dev))]
    #[inline]
    pub fn wait<T>(
        &self,
        _ev: EventToken<'ctx>,
        _buf: DeviceBuffer<'id, 'ctx, T, InFlight>,
    ) -> DeviceBuffer<'id, 'ctx, T, Ready> {
        unimplemented!("Queue::wait<T>: block on event, transition InFlight->Ready")
    }

    /// Nur für Compile-Fail-Tests gedacht: liefert ein Dummy-Kernel.
    pub fn create_dummy_kernel(&self) -> Kernel<'id, 'ctx> {
        // Wird in den Tests nicht wirklich benutzt → darf unimplemented! sein.
        Kernel { _q: core::marker::PhantomData }
    }
}

impl<'id, 'ctx> Kernel<'id, 'ctx> {
    // dev: simpler Platzhalter
    #[cfg(hpc_core_dev)]
    pub fn new(_q: &'ctx Queue<'id, 'ctx>, _name: &str) -> Self {
        Self {
            _q: core::marker::PhantomData,
        }
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
    #[cfg(not(hpc_core_dev))]
    pub fn new(_q: &'ctx Queue<'id, 'ctx>, _name: &str) -> Self {
        unimplemented!()
    }
}

impl<'id, 'ctx, T, S: State> core::fmt::Debug for DeviceBuffer<'id, 'ctx, T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DeviceBuffer")
            .field("len_elems", &self.len_elems)
            .finish_non_exhaustive()
    }
}

impl<'id, 'ctx, T, S: State> DeviceBuffer<'id, 'ctx, T, S> {
    /// Interne Konstruktion aus Low-Level-Buffer + Elementlänge.
    /// (Sichtbar im Crate; die High-Level-APIs stellen das später sicher her.)
    pub(crate) fn from_inner(inner: LlBuffer<S>, len_elems: usize) -> Self {
        Self {
            inner,
            len_elems,
            _ty: PhantomData,
            _ctx: PhantomData,
        }
    }

    /// Gibt den Low-Level-Buffer wieder zurück (z. B. für interne Delegation).
    pub(crate) fn into_inner(self) -> LlBuffer<S> {
        self.inner
    }

    /// Anzahl Elemente (nicht Bytes).
    #[inline]
    pub fn len_elems(&self) -> usize {
        self.len_elems
    }

    /// Byte-Länge, abgeleitet aus `T`.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.len_elems * core::mem::size_of::<T>()
    }

    pub(crate) fn from_inner_unchecked(inner: LlBuffer<S>) -> Self {
        let bytes = inner.len_bytes();
        let sz = core::mem::size_of::<T>();
        debug_assert!(
            bytes % sz == 0,
            "DeviceBuffer<'id, 'ctx, {}>: bytes ({}) nicht durch size_of T ({}) teilbar",
            core::any::type_name::<T>(),
            bytes,
            sz
        );
        let len_elems = bytes / sz;
        Self {
            inner,
            len_elems,
            _ty: PhantomData,
            _ctx: PhantomData,
        }
    }
}