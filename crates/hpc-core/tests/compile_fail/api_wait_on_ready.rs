use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Buffer ist bereits Ready
    let buf = ctx
        .create_buffer::<u8>(16).unwrap()
        .enqueue_write(&queue, &[0u8; 16]).unwrap(); // → Ready

    // ❌ Das sollte NICHT kompilieren
    // Ready-Buffer hat keine wait()-Methode!
    let _illegal = buf.wait(); 
    //                ^^^^ should not exist on Ready state
}