#![allow(unused_variables, unreachable_code)]

// Happy-Path-Skeleton: soll kompilieren, aber NICHT laufen.
// trybuild führt pass-Tests aus; deshalb packen wir alles in `if false { ... }`.

use hpc_core::api::{Context, Kernel};

fn main() {
    if false {
        let ctx = Context::new();
        let q = ctx.queue();

        let buf = q.create_buffer_elems::<f32>(8);
        let ready = q.enqueue_write(buf, &[0.0; 8]);

        let k = Kernel::new(&q, "vec_add");
        let (inflight, ev) = q.enqueue_kernel(&k, ready);
        let ready2 = q.wait(ev, inflight);

        let mut out = [0.0f32; 8];
        q.read_blocking(&ready2, &mut out);
    }
}
