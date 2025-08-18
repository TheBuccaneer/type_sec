use hpc_core::api::{Context, DeviceBuffer, Queue, Kernel};

fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    let buf: DeviceBuffer<u32, _> = q.create_buffer_elems(4);
    let buf = q.enqueue_write(buf, &[1, 2, 3, 4]);

    let k = Kernel::new(&q, "dummy");
    let (buf_inflight, evt) = q.enqueue_kernel(&k, buf);

    // Erstes wait → legal
    let buf_ready = q.wait(evt, buf_inflight);

    // compile-fail: versuche nochmal wait() auf bereits Ready-Buffer
    // (Misc-Sync Fehlerklasse: wait() darf nur InFlight konsumieren).
    let _ = q.wait(evt, buf_ready);
}
