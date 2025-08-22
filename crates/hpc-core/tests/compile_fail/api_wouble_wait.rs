use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();
    
    // Buffer → Ready
    let buf = ctx
        .create_buffer::<u8>(16).unwrap()
        .enqueue_write(&queue, &[0u8; 16]).unwrap();
    
    // Non-blocking write statt kernel (einfacher für Test)
    let (inflight, evt) = buf.overwrite_non_blocking(&queue, &[1u8; 16]).unwrap();
    
    // Erster Wait (legal)
    let ready = evt.wait(inflight);
    
    // Zweiter Wait (illegal, evt wurde schon konsumiert)
    let _illegal = evt.wait(ready);
    //             ^^^ use of moved value `evt`
}
