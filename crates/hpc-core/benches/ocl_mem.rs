use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

/// CPU-Fallback: simuliert H↔D Kopien. Später hier Raw-OpenCL & API einklinken.
fn baseline_memcpy(size: usize) {
    // TODO: Ersetze dies durch *direkten* OpenCL-Transfer (H→D oder D→H)
    let src = vec![0u8; size];
    let mut dst = vec![0u8; size];
    // simulierte "Kopie"
    dst.copy_from_slice(&src);
    black_box(dst);
}

fn api_memcpy(size: usize) {
    // TODO: Ersetze dies durch denselben Transfer über *deine Typstate-API*
    let src = vec![1u8; size];
    let mut dst = vec![0u8; size];
    dst.copy_from_slice(&src);
    black_box(dst);
}

fn bench_memcpy(c: &mut Criterion) {
    // Größenstaffel: 1 KiB, 64 KiB, 1 MiB, 16 MiB
    const SIZES: &[usize] = &[1 * 1024, 64 * 1024, 1 * 1024 * 1024, 16 * 1024 * 1024];

    let mut group = c.benchmark_group("ocl_mem");
    for &size in SIZES {
        group.throughput(Throughput::Bytes(size as u64)); // Bytes/s anzeigen lassen

        group.bench_with_input(
            BenchmarkId::new("baseline_raw_opencl_memcpy", size),
            &size,
            |b, &sz| {
                b.iter(|| baseline_memcpy(black_box(sz)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("api_typestate_memcpy", size),
            &size,
            |b, &sz| {
                b.iter(|| api_memcpy(black_box(sz)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_memcpy);
criterion_main!(benches);
