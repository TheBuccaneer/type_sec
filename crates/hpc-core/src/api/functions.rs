// src/api/functions.rs

use crate::buffer::{GpuBuffer, state::Empty};
use crate::error::Result;
use super::{Context, DeviceBuffer};

//=============================================================================
// BUFFER CREATION
//=============================================================================

/// Erstellt einen neuen Buffer auf Nutzerebene!
/// 
/// # Beispiel
/// ```rust
/// let (ctx, queue) = Context::new_first_gpu()?;
/// let buffer = create_buffer::<f32>(&ctx, 1000)?;  // 1000 f32 Elemente
/// ```
pub fn create_buffer<T>(ctx: &Context, n_elems: usize) -> Result<DeviceBuffer<'static, T, Empty>> {
    let inner: GpuBuffer<Empty> = GpuBuffer::<Empty>::create_uninit_elems::<T>(ctx.raw(), n_elems)?;
    Ok(DeviceBuffer::from_inner(inner, n_elems))
}