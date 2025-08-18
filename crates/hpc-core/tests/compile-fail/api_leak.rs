use hpc_core::api::{Context, DeviceBuffer, Queue, Kernel};

fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    // Buffer anlegen + Write → Ready
    let buf: DeviceBuffer<u32, _> = q.create_buffer_elems(4);
    let buf = q.enqueue_write(buf, &[1, 2, 3, 4]);

    let k = Kernel::new(&q, "dummy");

    // Kernel starten → InFlight + Event
    let (buf_inflight, _evt) = q.enqueue_kernel(&k, buf);

    // compile-fail: Context droppt am Ende von main(), 
    // während Buffer noch InFlight ist → Leak/Use-after-free
    drop(ctx);
    let _ = buf_inflight; // Buffer überlebt den Context nicht
}
