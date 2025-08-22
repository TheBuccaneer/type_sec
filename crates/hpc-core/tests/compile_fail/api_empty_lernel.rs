use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Buffer ist noch Empty
    let buf = ctx.create_buffer::<u8>(16).unwrap();

    // Kernel starten mit Empty ( illegal)
    let (_inflight, _evt) = buf.enqueue_kernel(&queue, &unimplemented!(), 16).unwrap();
}
