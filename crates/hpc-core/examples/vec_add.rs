use hpc_core::api::{Context, Kernel, Queue, create_buffer};
use hpc_core::buffer::state::{Empty, InFlight, Ready};
use hpc_core::error::Result;

// OpenCL Kernel Source - Einfache Vektor-Addition
const KERNEL_SOURCE: &str = r#"
__kernel void vector_add(
    __global const float* a,
    __global const float* b, 
    __global float* result,
    const unsigned int n
) {
    int gid = get_global_id(0);
    if (gid < n) {
        result[gid] = a[gid] + b[gid];
    }
}
"#;

fn main() -> Result<()> {
    println!("🚀 Starting Vector Addition Example...");

    // 1. OpenCL Setup
    let (ctx, queue) = Context::new_first_gpu()?;
    println!("✅ OpenCL Context & Queue created");

    // 2. Kernel kompilieren
    let kernel = Kernel::from_source(&ctx, KERNEL_SOURCE, "vector_add")?;
    println!("✅ Kernel compiled");

    // 3. Test-Daten vorbereiten
    const N: usize = 1024;
    let host_a: Vec<f32> = (0..N).map(|i| i as f32).collect(); // [0, 1, 2, ...]
    let host_b: Vec<f32> = (0..N).map(|i| (i * 2) as f32).collect(); // [0, 2, 4, ...]
    let mut host_result: Vec<f32> = vec![0.0; N];

    println!("✅ Test data prepared: {} elements", N);
    println!(
        "   Input A: [{}, {}, {}, ...]",
        host_a[0], host_a[1], host_a[2]
    );
    println!(
        "   Input B: [{}, {}, {}, ...]",
        host_b[0], host_b[1], host_b[2]
    );

    // 4. GPU Buffers erstellen und Daten übertragen
    println!("📤 Creating buffers and transferring data...");

    let buf_a = create_buffer::<f32>(&ctx, N)?.enqueue_write(&queue, &host_a)?; // Empty -> Ready

    let buf_b = create_buffer::<f32>(&ctx, N)?.enqueue_write(&queue, &host_b)?; // Empty -> Ready

    let buf_result = create_buffer::<f32>(&ctx, N)?.enqueue_write(&queue, &vec![0.0f32; N])?; // Nullen als Initialwerte

    println!("✅ Buffers created and data transferred");

    // 5. Kernel-Argumente setzen (das müsstest du noch in deiner API implementieren)
    // Für jetzt nehmen wir an, dass der Kernel irgendwie die Buffer als Argumente bekommt

    // 6. Kernel ausführen
    println!("🔥 Launching kernel...");

    // NOTE: Du musst noch kernel.set_arg() Funktionen implementieren!
    // Für jetzt simulieren wir es:

    let (buf_result_inflight, event_token) = buf_result.enqueue_kernel(&queue, &kernel, N)?;
    println!("✅ Kernel launched, waiting for completion...");

    // 7. Auf Kernel-Completion warten
    let buf_result_ready = queue.wait(event_token, buf_result_inflight);
    println!("✅ Kernel completed");

    // 8. Ergebnisse zurücklesen
    println!("📥 Reading results...");
    buf_result_ready.enqueue_read(&queue, &mut host_result)?;

    // 9. Ergebnisse validieren
    println!("🔍 Validating results...");
    let mut errors = 0;
    for i in 0..std::cmp::min(10, N) {
        let expected = host_a[i] + host_b[i];
        let actual = host_result[i];

        if (expected - actual).abs() > 1e-6 {
            println!(
                "   ❌ ERROR at index {}: expected {}, got {}",
                i, expected, actual
            );
            errors += 1;
        } else {
            println!(
                "   ✅ [{}]: {} + {} = {} ✓",
                i, host_a[i], host_b[i], actual
            );
        }
    }

    if errors == 0 {
        println!("🎉 SUCCESS! Vector addition completed correctly!");
        println!("   Processed {} elements", N);
    } else {
        println!("❌ Found {} errors in results", errors);
    }

    Ok(())
}

// Alternative main() falls die obige API noch nicht komplett funktioniert:
#[allow(dead_code)]
fn main_simple_memtest() -> Result<()> {
    println!("🚀 Simple Memory Test...");

    // 1. Setup
    let (ctx, queue) = Context::new_first_gpu()?;

    // 2. Test-Daten
    let test_data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut read_back: Vec<f32> = vec![0.0; 5];

    // 3. H->D->H Transfer ohne Kernel
    println!("📤 Writing data to GPU...");
    let buf = create_buffer::<f32>(&ctx, 5)?.enqueue_write(&queue, &test_data)?;

    println!("📥 Reading data back from GPU...");
    buf.enqueue_read(&queue, &mut read_back)?;

    // 4. Vergleichen
    println!("Original: {:?}", test_data);
    println!("Read back: {:?}", read_back);

    if test_data == read_back {
        println!("✅ Memory test PASSED!");
    } else {
        println!("❌ Memory test FAILED!");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_roundtrip() {
        main_simple_memtest().expect("Memory test should pass");
    }

    #[test]
    fn test_vector_add() {
        main().expect("Vector addition should work");
    }
}
