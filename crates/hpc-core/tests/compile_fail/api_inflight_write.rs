use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Buffer erzeugen und beschreiben → Ready
    let buf = ctx.create_buffer::<u8>(16).unwrap()
        .enqueue_write(&queue, &[0u8; 16]).unwrap();

    // Kernel starten → Result wird InFlight
    let (inflight, _evt) = buf.enqueue_kernel(&queue, &unimplemented!(), 16).unwrap();

    // Das hier darf nicht gehen: Write auf InFlight
    inflight.overwrite_blocking(&queue, &[1u8; 16]).unwrap();
}