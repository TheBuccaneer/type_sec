
use hpc_core::api::Result;  
use hpc_core::api::Context;
use hpc_core::api::create_buffer;

#[test]
fn memcpy_roundtrip() -> Result<()> {
    // 1. Context + Queue
    let (ctx, queue) = Context::new_first_gpu()?;

    // 2. Hostdaten vorbereiten
    let input: Vec<f32> = vec![1.0; 16];
    let mut output: Vec<f32> = vec![0.0; 16];

    // 3. Buffer erstellen (Empty)
    let buf = create_buffer::<f32>(&ctx, input.len())?;

    // 4. Host→Device (Ready)
    let buf = buf.enqueue_write(&queue, &input)?;

    // 5. Device→Host (Ready bleibt Ready)
    buf.enqueue_read(&queue, &mut output)?;

    // 6. Prüfen
    assert_eq!(input, output);

    Ok(())
}