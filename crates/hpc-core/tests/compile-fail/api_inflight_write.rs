// Passe den Crate-Namen an, falls euer Paket nicht "hpc-core" heißt.
// Cargo macht aus "hpc-core" -> "hpc_core".
use hpc_core::api::Kernel;
use hpc_core::api::Context;
use hpc_core::api::Queue;

fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    let buf = q.create_buffer_elems::<u32>(16);
    let ready = q.enqueue_write(buf, &[0u32; 16]);

    let k = Kernel::new(&q, "dummy");
    let (inflight, _ev) = q.enqueue_kernel(&k, ready);

    // illegal: während InFlight erneut schreiben -> sollte NICHT kompilieren
    let _bad = q.enqueue_write(inflight, &[1u32; 16]);
}
