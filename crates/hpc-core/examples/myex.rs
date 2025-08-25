use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Buffer → Ready
    let buf = ctx
        .create_empty_buffer::<u8>(16).unwrap()
        .write_block(&queue, &[0u8; 16]).unwrap();

    // Kernel starten → InFlight + Event
    let kernel = Kernel::from_source(&ctx, "kernel void dummy(__global uchar* buf) {}", "dummy").unwrap();
    let (inflight, _evt) = buf.enqueue_kernel(&queue, &kernel, 16).unwrap();

    let mut out = [0u8; 16];

    // Verboten: Read auf InFlight
    inflight.read_blocking(&queue, &mut out).unwrap();
}
