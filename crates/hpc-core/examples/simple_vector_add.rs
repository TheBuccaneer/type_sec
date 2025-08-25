// examples/simple_vector_add.rs

use hpc_core::*;

fn main() -> Result<()> {
    // 1. OpenCL Context und Queue erstellen
    let ctx = Context::create_context()?;
    let queue = ctx.create_queue()?;

    // 2. Test-Daten vorbereiten - einfach mit 1en
    let size = 1024;
    let a: Vec<u8> = vec![1; size];  // Alle Elemente = 1
    let b: Vec<u8> = vec![1; size];  // Alle Elemente = 1
    let mut result: Vec<u8> = vec![0; size];

    println!("Vorbereitung: {} Elemente", size);

    // 3. Kernel-Source für u8 vector addition
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

    // 4. Buffer erstellen und Daten hochladen
    let buffer_a = ctx.create_empty_buffer::<u8>(size)?
        .write_block(&queue, &a)?; // Empty → Written

    let buffer_b = ctx.create_empty_buffer::<u8>(size)?
        .write_block(&queue, &b)?; // Empty → Written

    let buffer_result = ctx.create_empty_buffer::<u8>(size)?
        .write_block(&queue, &result)?; // Empty → Written (mit Nullen)

    println!("Buffer erstellt und initialisiert");

    // 5. Kernel kompilieren
    let kernel = Kernel::from_source(&ctx, kernel_source, "vector_add")?;
    println!("Kernel kompiliert");

    // 6. Kernel-Argumente setzen
    kernel.set_arg_buffer(0, &buffer_a)?; // Input A
    kernel.set_arg_buffer(1, &buffer_b)?; // Input B  
    kernel.set_arg_buffer(2, &buffer_result)?; // Output
    kernel.set_arg_scalar(3, &(size as u32))?; // Size parameter

    println!("Kernel-Argumente gesetzt");

    // 7. Kernel ausführen
    let (inflight_buffer, event) = buffer_result.enqueue_kernel(&queue, &kernel, size)?;
    println!("Kernel gestartet");

    // 8. Warten bis Kernel fertig ist und zurück zu Written
    let result_buffer = event.wait(inflight_buffer); // InFlight → Written (kein ?)
    println!("Kernel abgeschlossen");

    // 9. Ergebnis zurück lesen  
    result_buffer.read_blocking(&queue, &mut result)?;
    println!("Ergebnis gelesen");

    // 10. Erste 100 Ergebnisse anzeigen
    println!("\nErste 100 Ergebnisse der Vektor-Addition:");
    for i in 0..100.min(size) {
        println!("{:3}: {:3} + {:3} = {:3}", i, a[i], b[i], result[i]);
    }

    // 11. Vollständige Verifikation (sollte überall 2 sein)
    let mut errors = 0;
    for i in 0..size {
        let expected = 2u8; // 1 + 1 = 2
        if result[i] != expected {
            if errors < 5 { // Nur erste 5 Fehler anzeigen
                println!("Fehler bei Index {}: erwartet {}, bekommen {}", i, expected, result[i]);
            }
            errors += 1;
        }
    }

    if errors == 0 {
        println!("\nVector Addition erfolgreich! Alle {} Ergebnisse korrekt.", size);
    } else {
        println!("\n {} Fehler bei {} Elementen gefunden", errors, size);
        return Err("Verifikation fehlgeschlagen".into());
    }

    Ok(())
}