//! examples/simple_vector_add.rs
//! happy path example. Fully working
//!
//!
use hpc_core::*;

fn main() -> Result<()> {
    // 1. we need context and queue
    let ctx = Context::create_context()?;
    let queue = ctx.create_queue()?;

    // 2. test data size times input data
    let size = 1024;
    let a: Vec<u32> = vec![1; size]; // Alle Elemente = 1
    let b: Vec<u32> = vec![1; size]; // Alle Elemente = 1
    let mut result: Vec<u32> = vec![0; size];

    println!("Preparing: {} elements", size);

    // 3. Kernel-Source für u32 vector addition (FIXED: uchar -> uint)
    let kernel_source = r#"
        __kernel void vector_add(
            __global const uint* a,
            __global const uint* b, 
            __global uint* result,
            const unsigned int size
        ) {
            int gid = get_global_id(0);
            if (gid < size) {
                result[gid] = a[gid] + b[gid];
            }
        }
    "#;

    // 4. Buffer erstellen und Daten hochladen
    let buffer_a = ctx
        .create_empty_buffer::<u32>(size)?
        .write_block(&queue, &a)?; // Empty → Written

    let buffer_b = ctx
        .create_empty_buffer::<u32>(size)?
        .write_block(&queue, &b)?; // Empty → Written

    let buffer_result = ctx
        .create_empty_buffer::<u32>(size)?
        .write_block(&queue, &result)?; // Empty → Written (mit Nullen)

    println!("Buffers created and initialized");

    // 5. Kernel kompilieren
    let kernel = Kernel::from_source(&ctx, kernel_source, "vector_add")?;
    println!("Kernel compiled");

    // 6. Kernel-Argumente setzen
    kernel.set_arg_buffer(0, &buffer_a)?; // Input A
    kernel.set_arg_buffer(1, &buffer_b)?; // Input B  
    kernel.set_arg_buffer(2, &buffer_result)?; // Output
    kernel.set_arg_scalar(3, &(size as u32))?; // Size parameter

    println!("Kernel arguments set");

    // 7. Kernel ausführen
    let (inflight_buffer, event) = buffer_result.enqueue_kernel(&queue, &kernel, size)?;
    println!("Kernel started");

    // 8. Warten bis Kernel fertig ist und zurück zu Written
    let result_buffer = event.wait(inflight_buffer); // InFlight → Written (kein ?)
    println!("Kernel completed");

    // 9. Ergebnis zurück lesen
    result_buffer.read_blocking(&queue, &mut result)?;
    println!("Results read back");

    // 10. Erste 100 Ergebnisse anzeigen
    println!("\nFirst 100 results of vector addition:");
    for i in 0..100.min(size) {
        println!("{:3}: {:3} + {:3} = {:3}", i, a[i], b[i], result[i]);
    }

    // 11. Vollständige Verifikation (sollte überall 2 sein)
    let mut errors = 0;
    for i in 0..size {
        let expected = 2u32; // 1 + 1 = 2
        if result[i] != expected {
            if errors < 5 {
                // Only show first 5 errors
                println!(
                    "Error at index {}: expected {}, got {}",
                    i, expected, result[i]
                );
            }
            errors += 1;
        }
    }

    if errors == 0 {
        println!(
            "\nVector addition successful! All {} results correct.",
            size
        );
    } else {
        println!("\n{} errors found in {} elements", errors, size);
        return Err("Verification failed".into());
    }

    Ok(())
}
