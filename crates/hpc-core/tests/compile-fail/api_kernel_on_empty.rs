use hpc_core::api::{Context, Kernel};
use hpc_core::api::Queue;

fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    // Buffer bleibt Empty, kein Write
    let buf = q.create_buffer_elems::<u32>(16);

    let k = Kernel::new(&q, "dummy");

    // compile-fail: enqueue_kernel erwartet Ready-Buffer, bekommt Empty
    let (_inflight, _ev) = q.enqueue_kernel(&k, buf);
}
