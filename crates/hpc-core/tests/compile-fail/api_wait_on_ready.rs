use hpc_core::api::{Context, Kernel};

fn main() {
    let ctx = Context::new();
    let q = ctx.queue();

    let buf = q.create_buffer_elems::<u32>(16);
    let ready = q.enqueue_write(buf, &[0u32; 16]);

    // Kein Kernel gestartet → Buffer ist Ready, nicht InFlight.
    // ❌ sollte NICHT kompilieren: wait erwartet EventToken + InFlight-Buffer
    let ev = unsafe { core::mem::transmute::<usize, hpc_core::api::EventToken<'_>>(0) }; // nur damit’s parsed; wird eh nicht gelinkt
    let _back_to_ready = q.wait(ev, ready);
}
