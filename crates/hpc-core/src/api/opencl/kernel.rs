// src/api/kernel.rs

use opencl3::{kernel::Kernel as CLKernel, program::Program as CLProgram};

use super::Context;
use crate::error::Result;
use crate::{DeviceBuffer, Ready};
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
    pub fn from_source(ctx: &Context<'brand>, src: &str, name: &str) -> Result<Self> {
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

    /// Setze ein Buffer-Argument (nur Ready-Buffer mit gleicher Brand erlaubt)
    pub fn set_arg_buffer<T>(
        &self,
        index: u32,
        buf: &DeviceBuffer<'brand, T, Ready>,
    ) -> Result<()> {
        self.inner.set_arg(index, buf.inner.raw())?;
        Ok(())
    }

    /// Scalar-Argument: nur zugelassene POD-Typen
    pub fn set_arg_scalar<S: KernelScalar>(&self, index: u32, val: &S) -> Result<()> {
        self.inner.set_arg(index, val)?; // unsafe entfernt - nicht nötig
        Ok(())
    }
}

/// Marker-Trait für erlaubte Skalare (keine &Vec/&[T]/Pointer etc.)
pub trait KernelScalar: bytemuck::Pod {}
impl KernelScalar for u8 {}
impl KernelScalar for i32 {}
impl KernelScalar for u32 {}
impl KernelScalar for f32 {}
impl KernelScalar for f64 {}
impl KernelScalar for u64 {}
