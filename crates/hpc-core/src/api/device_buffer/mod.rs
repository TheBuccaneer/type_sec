// src/api/device_buffer/mod.rs

use crate::buffer::{GpuBuffer, state::State};
use std::marker::PhantomData;

// State-specific implementations
mod empty;
mod inflight;
mod ready;

//=============================================================================
// STRUCT DEFINITION
//=============================================================================

/// High-Level typisierter Buffer. Delegiert an den GPUBuffer
///
/// User API: Buffers können erstellt, gelesen, geschrieben und für Kernels genutzt werden.
/// Die interne Struktur ist versteckt - Type-State Pattern sorgt für Compile-Time Safety.
#[derive(Debug)]
pub struct DeviceBuffer<'brand, T, S: State> {
    pub(crate) inner: GpuBuffer<S>,
    pub(crate) len: usize,
    pub(crate) _marker: PhantomData<&'brand T>,
}

//=============================================================================
// SHARED IMPLEMENTATIONS
//=============================================================================

impl<'ctx, T, S: State> DeviceBuffer<'ctx, T, S> {
    /// Hilfsfunktion, die intern einfach das neue Objekt weitergibt
    pub(crate) fn from_inner(inner: GpuBuffer<S>, len_elems: usize) -> Self {
        Self {
            inner,          // der u8-Buffer
            len: len_elems, // Element-Länge für Nutzer
            _marker: PhantomData,
        }
    }
}
