use hpc_core::*;

/*
trying to invoke empty buffer without data
 */


fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();


    let buf = ctx.create_empty_buffer::<u8>(16).unwrap();

    let kernel = Kernel::from_source(&ctx, "kernel void dummy(__global uchar* buf) {}", "dummy").unwrap();
    let (_inflight, _evt) = buf.enqueue_kernel(&queue, &kernel, 16).unwrap();
}
