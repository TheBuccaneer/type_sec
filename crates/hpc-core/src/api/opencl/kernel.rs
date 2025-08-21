// src/api/kernel.rs

use opencl3::{
    kernel::Kernel as CLKernel,
    program::Program as CLProgram,
};

use crate::error::Result;
use super::Context;
use crate::{DeviceBuffer, Ready};  // Beide auf einmal         // Dein DeviceBuffer Typ
  // Ready State

//=============================================================================
// KERNEL
//=============================================================================

#[must_use]
#[derive(Debug)]
pub struct Kernel<'q> {
    inner: CLKernel,
    #[allow(dead_code)]
    program: CLProgram,
    _marker: std::marker::PhantomData<&'q ()>, // Kernel hängt an Context/Queue
}




impl<'q> Kernel<'q> {
    pub fn from_source(ctx: &Context, src: &str, name: &str) -> Result<Self> {
        let program = CLProgram::create_and_build_from_source(ctx.raw(), src, "")?;
        let inner = CLKernel::create(&program, name)?;
        Ok(Self {
            inner,
            program,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn raw(&self) -> &CLKernel {
        &self.inner
    }

    pub fn set_arg_buffer<T>(&self, index: u32, buf: &DeviceBuffer<'_, T, Ready>) -> Result<()> {
        // ✅ Über das inner Feld zugreifen:
        self.inner.set_arg(index, buf.inner.raw())?;
        Ok(())
    }

    /// Scalar-Argument: nur zugelassene POD-Typen
    pub fn set_arg_scalar<S: KernelScalar>(&self, index: u32, val: &S) -> Result<()> {
        unsafe { self.inner.set_arg(index, val)?; }
        Ok(())
    }
}


/// Marker-Trait für erlaubte Skalare (keine &Vec/&[T]/Pointer etc.)
pub trait KernelScalar {}
impl KernelScalar for u8 {}
impl KernelScalar for i32 {}
impl KernelScalar for u32 {}
impl KernelScalar for f32 {}
impl KernelScalar for f64 {}
impl KernelScalar for u64 {} 
