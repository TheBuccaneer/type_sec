use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

/// CPU-Fallback: Vektoraddition C = A + B.
/// Später: Raw-OpenCL-Kernel (Baseline) vs. Typstate-API (Treatment).
fn baseline_vecadd(n: usize) {
    // TODO: Ersetze durch *direkten* OpenCL-Kernel-Launch (VecAdd)
    let a = vec![1.0f32; n];
    let b = vec![2.0f32; n];
    let mut c = vec![0.0f32; n];
    for i in 0..n {
        c[i] = a[i] + b[i];
    }
    black_box(c);
}

fn api_vecadd(n: usize) {
    // TODO: Ersetze durch denselben Kernel via *deine Typstate-API*
    let a = vec![1.0f32; n];
    let b = vec![2.0f32; n];
    let mut c = vec![0.0f32; n];
    for i in 0..n {
        c[i] = a[i] + b[i];
    }
    black_box(c);
}

fn bench_vecadd(c: &mut Criterion) {
    // Größenstaffel (Elemente): 2^16, 2^20, 2^24
    const NS: &[usize] = &[1 << 16, 1 << 20, 1 << 24];

    let mut group = c.benchmark_group("ocl_vecadd");
    for &n in NS {
        group.throughput(Throughput::Elements(n as u64)); // Elemente/s

        group.bench_with_input(
            BenchmarkId::new("baseline_raw_opencl_vecadd", n),
            &n,
            |b, &nn| {
                b.iter(|| baseline_vecadd(black_box(nn)));
            },
        );

        group.bench_with_input(BenchmarkId::new("api_typestate_vecadd", n), &n, |b, &nn| {
            b.iter(|| api_vecadd(black_box(nn)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_vecadd);
criterion_main!(benches);
