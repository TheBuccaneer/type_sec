// Cargo macht aus "hpc-core" -> "hpc_core"
use hpc_core::api::{Context, Kernel};

fn main() {
    let ctx = Context::new();
    let q = ctx.queue();

    // typisierte Allokation + erster Write (Bodies sind egal, trybuild prüft nur Typen)
    let buf = q.create_buffer_elems::<u32>(16);
    let ready = q.enqueue_write(buf, &[0u32; 16]);

    let k = Kernel::new(&q, "dummy");

    // 1. Kernel → InFlight + Event
    let (inflight, ev) = q.enqueue_kernel(&k, ready);

    // 1. wait() verbraucht `ev`
    let ready_again = q.wait(ev, inflight);

    // 2. Kernel → neuer Event (ev2)
    let (inflight2, _ev2) = q.enqueue_kernel(&k, ready_again);

    // ❌ compile-fail: `ev` wurde bereits bewegt (use of moved value)
    let _illegal = q.wait(ev, inflight2);
}
