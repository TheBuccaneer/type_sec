use hpc_core::api::{Context, DeviceBuffer, Queue, Kernel};
fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    let buf: DeviceBuffer<u32, _> = q.create_buffer_elems(4);

    let k = Kernel::new(&q, "dummy");

    // compile-fail: versuche Kernel-Launch direkt auf Empty-Buffer (ohne Write → Ready)
    let _ = q.enqueue_kernel(&k, buf);
}
