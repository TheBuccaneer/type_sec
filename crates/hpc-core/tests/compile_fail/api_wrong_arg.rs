use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Buffer → Ready
    let buf = ctx
        .create_buffer::<u8>(16).unwrap()
        .enqueue_write(&queue, &[0u8; 16]).unwrap();

    // Kernel, das einen Buffer + eine Zahl erwartet
    let kernel = Kernel::from_source(
        &ctx,
        "kernel void dummy(__global uchar* buf, uint n) {}",
        "dummy"
    ).unwrap();

    // ❌ Illegal: falsche Arg-Art – API erwartet `u32`, bekommt aber `DeviceBuffer`
    let _illegal = kernel.set_arg_scalar(1, &buf).unwrap();
}
