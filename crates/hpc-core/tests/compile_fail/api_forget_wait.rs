#![deny(unused_must_use)]
use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();
    let buf = ctx
        .create_buffer::<u8>(16).unwrap()
        .enqueue_write(&queue, &[0u8; 16]).unwrap();
    let kernel = Kernel::from_source(&ctx, "kernel void dummy(__global uchar* buf) {}", "dummy").unwrap();

    // Komplettes Ignorieren – löst must_use-Fehler aus:
    buf.enqueue_kernel(&queue, &kernel, 16).unwrap();
}
