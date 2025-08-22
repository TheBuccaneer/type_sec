use hpc_core::*; // ggf. Crate-Namen anpassen

#[test]
fn enqueue_write_same_queue_smoke() {
    let ctx = Context::create_context().expect("context");
    let q = ctx.create_queue().expect("queue");
    let buf = ctx.create_buffer::<u32>(4).expect("buffer");

    let data = [1u32, 2, 3, 4];
    let _evt = buf.enqueue_write(&q, &data).expect("enqueue_write should succeed");
}

#[test]
fn multiple_buffers_on_same_queue() {
    let ctx = Context::create_context().expect("context");
    let q = ctx.create_queue().expect("queue");

    let a = ctx.create_buffer::<u8>(8).expect("buf A");
    let b = ctx.create_buffer::<u8>(8).expect("buf B");

    let da = [0u8; 8];
    let db = [1u8; 8];

    let _ = a.enqueue_write(&q, &da).expect("A write");
    let _ = b.enqueue_write(&q, &db).expect("B write");
}

/// sehr leichter Test, der keine echte IO macht:
/// Er prüft, dass die API generisch über T funktioniert (Monomorphisierung).
#[test]
fn generic_over_element_type() {
    let ctx = Context::create_context().expect("context");
    let q = ctx.create_queue().expect("queue");

    let bu8 = ctx.create_buffer::<u8>(2).expect("u8 buf");
    let bu32 = ctx.create_buffer::<u32>(2).expect("u32 buf");

    let _ = bu8.enqueue_write(&q, &[7u8, 8u8]).expect("u8 write");
    let _ = bu32.enqueue_write(&q, &[7u32, 8u32]).expect("u32 write");
}
