use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Buffer → Ready
    let buf = ctx
        .create_empty_buffer::<u8>(16).unwrap()
        .write_block(&queue, &[0u8; 16]).unwrap();

    // Kernel, das einen Buffer + eine Zahl erwartet
    let kernel = Kernel::from_source(
        &ctx,
        "kernel void dummy(__global uchar* buf, uint n) {}",
        "dummy"
    ).unwrap();

    // wrong argument. Only scalar
    let _illegal = kernel.set_arg_scalar(1, &buf).unwrap();
}
