use hpc_core::api::{Context, DeviceBuffer, Queue, Kernel};

fn main() {
    // Setup: Context + Queue
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    // Buffer anlegen und beschreiben → Ready
    let buf: DeviceBuffer<u32, _> = q.create_buffer_elems(4);
    let buf = q.enqueue_write(buf, &[1, 2, 3, 4]);

    // Kernel starten → InFlight
    let k = Kernel::new(&q, "dummy");
    let (buf_inflight, _evt) = q.enqueue_kernel(&k, buf);

    // Fehler: Read auf InFlight-Buffer (sollte compile-fail sein)
    let mut out = [0u32; 4];
    q.read_blocking(&buf_inflight, &mut out);
}
