// examples/simple_vector_add.rs

use hpc_core::*;

#[test]
fn main() -> Result<()> {
    // 1. OpenCL Context und Queue erstellen (neue API)
    let ctx = Context::create_context()?;
    let queue = ctx.create_queue()?;

    // 2. Test-Daten vorbereiten (u8)
    let size = 1024;
    let a: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    let b: Vec<u8> = (0..size).map(|i| ((i * 2) % 256) as u8).collect();
    let mut result: Vec<u8> = vec![0; size];

    // 3. Kernel-Source (angepasst für u8)
    let kernel_source = r#"
        __kernel void vector_add(
            __global const uchar* a,
            __global const uchar* b, 
            __global uchar* result,
            const unsigned int size
        ) {
            int gid = get_global_id(0);
            if (gid < size) {
                result[gid] = a[gid] + b[gid];
            }
        }
    "#;

    // 4. Buffer erstellen und Daten hochladen (neue Context-API)
    let buffer_a = ctx.create_buffer::<u8>(size)?.enqueue_write(&queue, &a)?;
    let buffer_b = ctx.create_buffer::<u8>(size)?.enqueue_write(&queue, &b)?;
    let buffer_result = ctx.create_buffer::<u8>(size)?.enqueue_write(&queue, &result)?;

    // 5. Kernel kompilieren
    let kernel = Kernel::from_source(&ctx, kernel_source, "vector_add")?;

    // 6. ✅ Kernel-Argumente setzen (type-safe branded API)
    kernel.set_arg_buffer(0, &buffer_a)?;      // Input A
    kernel.set_arg_buffer(1, &buffer_b)?;      // Input B  
    kernel.set_arg_buffer(2, &buffer_result)?; // Output
    kernel.set_arg_scalar(3, &(size as u32))?; // Size parameter

    // 7. Kernel ausführen (buffer-zentriert)
    let (result_inflight, event) = buffer_result.enqueue_kernel(&queue, &kernel, size)?;

    // 8. Zurück zu Ready (nach Kernel-Completion)
    let result_ready = event.wait(result_inflight);

    // 9. Ergebnis zurück lesen
    result_ready.enqueue_read_blocking(&queue, &mut result)?;

    // 10. Ergebnis prüfen
    println!("Erste 10 Ergebnisse:");
    for i in 0..10 {
        println!("{}:  {} + {} = {}", i, a[i], b[i], result[i]);
    }

    // 11. Verify (mit u8 Overflow-Check)
    for i in 0..size {
        let expected = (a[i] as u16 + b[i] as u16) as u8; // Handle overflow
        assert_eq!(result[i], expected, "Mismatch at index {}", i);
    }

    println!("✅ Vector addition erfolgreich!");
    Ok(())
}
