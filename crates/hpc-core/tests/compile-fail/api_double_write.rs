use hpc_core::api::{Context, Kernel};

fn main() {
    let ctx = Context::new();
    let q = ctx.queue();

    let buf = q.create_buffer_elems::<u32>(16);
    let ready = q.enqueue_write(buf, &[0u32; 16]);

    let k = Kernel::new(&q, "dummy");
    let (inflight, ev) = q.enqueue_kernel(&k, ready);

    let ready_again = q.wait(ev, inflight);

    // ❌ compile-fail: `ev` wurde schon in `wait` verbraucht (use of moved value)
    let _still_ready = q.wait(ev, ready_again);
}
