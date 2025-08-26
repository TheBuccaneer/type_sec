//! High-level wrapper for OpenCL kernels.
//! Provides a safe API around cl_kernel handles, bound to a specific
//! - Lifetime branding (`'q`) to prevent cross-queue mixing.
//!
use opencl3::{kernel::Kernel as CLKernel, program::Program as CLProgram};

use super::Context;
use crate::DeviceBuffer;
use crate::buffer::state::Written;
use crate::error::Result;
use std::marker::PhantomData;

#[must_use]
#[derive(Debug)]
pub struct Kernel<'brand> {
    inner: CLKernel,
    #[allow(dead_code)]
    program: CLProgram,
    _brand: PhantomData<&'brand ()>,
}

impl<'brand> Kernel<'brand> {
    pub fn from_source(ctx: &'brand Context<'brand>, src: &str, name: &str) -> Result<Self> {
        let program = CLProgram::create_and_build_from_source(ctx.raw(), src, "")?;
        let inner = CLKernel::create(&program, name)?;
        Ok(Self {
            inner,
            program,
            _brand: PhantomData,
        })
    }

    pub fn raw(&self) -> &CLKernel {
        &self.inner
    }

    /// Set a buffer argument (only ready buffers with the same brand are allowed)
    pub fn set_arg_buffer<T>(
        &self,
        index: u32,
        buf: &DeviceBuffer<'brand, T, Written>,
    ) -> Result<()> {
        self.inner.set_arg(index, buf.inner.raw())?;
        Ok(())
    }

    /// Scalar argument: only allowed POD types
    pub fn set_arg_scalar<S: KernelScalar>(&self, index: u32, val: &S) -> Result<()> {
        self.inner.set_arg(index, val)?; // unsafe entfernt - nicht nötig
        Ok(())
    }
}

/// Marker trait for allowed scalars (no &Vec/&[T]/Pointer etc.)
pub trait KernelScalar: bytemuck::Pod {}
impl KernelScalar for u8 {}
impl KernelScalar for i32 {}
impl KernelScalar for u32 {}
impl KernelScalar for f32 {}
impl KernelScalar for f64 {}
impl KernelScalar for u64 {}
