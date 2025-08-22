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
    // Zwei separate Marker:
    pub(crate) _brand: PhantomData<fn(&'brand ()) -> &'brand ()>, // invariant für Context
    pub(crate) _type: PhantomData<T>, // kovariant für Typ (das ist OK)
}

impl<'ctx, T, S: State> DeviceBuffer<'ctx, T, S> {
    pub(crate) fn from_inner(inner: GpuBuffer<S>, len_elems: usize) -> Self {
        Self {
            inner,
            len: len_elems,
            _brand: PhantomData,  // ← für Context-Branding
            _type: PhantomData,   // ← für Typ-Parameter
        }
    }
}