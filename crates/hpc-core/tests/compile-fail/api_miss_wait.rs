use hpc_core::api::{Context, DeviceBuffer, Queue, Kernel};

fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    let buf: DeviceBuffer<u32, _> = q.create_buffer_elems(4);
    let buf = q.enqueue_write(buf, &[1, 2, 3, 4]);

    let k = Kernel::new(&q, "dummy");

    // Kernel → InFlight
    let (buf_inflight, _evt) = q.enqueue_kernel(&k, buf);

    // compile-fail: versuche, Buffer ohne wait() erneut für Kernel zu nutzen
    let _ = q.enqueue_kernel(&k, buf_inflight);
}
