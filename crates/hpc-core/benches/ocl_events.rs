use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::atomic::{Ordering, fence};

/// CPU-Fallback: simuliert eine Event-Wait-Kette.
/// In echt: N Enqueues + Event-Wait-List / clWaitForEvents o.ä.
fn baseline_event_chain(len: usize) {
    // TODO: Ersetze durch *direkte* OpenCL-Event-Kette (Baseline)
    let mut token: u64 = 0;
    for _ in 0..len {
        // Simuliere "Arbeit" + Sequenzbarriere
        token = token.wrapping_add(1);
        fence(Ordering::SeqCst);
    }
    black_box(token);
}

fn api_event_chain(len: usize) {
    // TODO: Ersetze durch denselben Ablauf via *deine Typstate-API* (InFlight -> wait -> Ready)
    let mut token: u64 = 0;
    for _ in 0..len {
        token = token.wrapping_add(1);
        fence(Ordering::SeqCst);
    }
    black_box(token);
}

fn bench_events(c: &mut Criterion) {
    // Kettenlängen: 1, 4, 16
    const LENS: &[usize] = &[1, 4, 16];

    let mut group = c.benchmark_group("ocl_events");
    for &len in LENS {
        group.throughput(Throughput::Elements(len as u64)); // "Events"/s (Kettenlänge als Maß)

        group.bench_with_input(
            BenchmarkId::new("baseline_raw_opencl_events", len),
            &len,
            |b, &ll| {
                b.iter(|| baseline_event_chain(black_box(ll)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("api_typestate_events", len),
            &len,
            |b, &ll| {
                b.iter(|| api_event_chain(black_box(ll)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_events);
criterion_main!(benches);
