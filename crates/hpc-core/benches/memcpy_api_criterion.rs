use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use hpc_core::api::Context;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context as CLContext;
use opencl3::device::get_all_devices;
use opencl3::memory::{Buffer as CLBuffer, CL_MEM_READ_WRITE};
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};
use std::ptr;

const SIZES: &[usize] = &[
    1 * 1024,          // 1 KiB
    64 * 1024,         // 64 KiB
    1 * 1024 * 1024,   // 1 MiB
    16 * 1024 * 1024,  // 16 MiB
    100 * 1024 * 1024, // 100 MiB
];

fn api_memcpy_bench(c: &mut Criterion) {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    let mut group = c.benchmark_group("api_memcpy_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        // Daten vorbereiten
        let src: Vec<u8> = vec![1; nbytes];
        let mut dst: Vec<u8> = vec![0; nbytes];

        // Buffer einmal anlegen und initialisieren
        let buf = ctx.create_empty_buffer::<u8>(nbytes).unwrap(); // Empty
        let mut buf = buf.write_block(&queue, &src).unwrap(); // Ready

        group.bench_function(format!("copy_bytes_{}", nbytes), |b| {
            b.iter(|| {
                buf.overwrite_blocking_for_bench(&queue, black_box(&src)).unwrap(); //InFlight
                buf.read_blocking(&queue, black_box(&mut dst))
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn memcpy_opencl3_bench(c: &mut Criterion) {
    let device_ids =
        get_all_devices(opencl3::device::CL_DEVICE_TYPE_GPU).expect("Kein GPU gefunden");
    let ctx = CLContext::from_devices(&device_ids, &[], None, ptr::null_mut()).unwrap();
    let queue = CommandQueue::create(&ctx, device_ids[0], 0).unwrap();

    let mut group = c.benchmark_group("memcpy_opencl3_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        let src: Vec<u8> = vec![1; nbytes];
        let mut dst: Vec<u8> = vec![0; src.len()];
        let mut buf: CLBuffer<u8> =
            CLBuffer::create(&ctx, CL_MEM_READ_WRITE, src.len(), ptr::null_mut()).unwrap();

        group.bench_function(format!("copy_only_{}", nbytes), |b| {
            b.iter(|| {
                queue
                    .enqueue_write_buffer(&mut buf, CL_BLOCKING, 0, black_box(&src), &[])
                    .unwrap();
                queue
                    .enqueue_read_buffer(&buf, CL_BLOCKING, 0, black_box(&mut dst), &[])
                    .unwrap();
            });
        });
    }

    group.finish();
}


criterion_group!(
    benches,
    memcpy_opencl3_bench,
    api_memcpy_bench,

);
criterion_main!(benches);
