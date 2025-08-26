//! benchmark file for criterion benchmarks.
//! One file benches all

#![allow(warnings)]

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use hpc_core::api::Context;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context as CLContext;
use opencl3::device::get_all_devices;
use opencl3::memory::{Buffer as CLBuffer, CL_MEM_READ_WRITE};
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};
use std::ptr;

const SIZES: &[usize] = &[
    1 * 1024,          // 1 KB
    64 * 1024,         // 64 KB
    1 * 1024 * 1024,   // 1 MB
    16 * 1024 * 1024,  // 16 MB
    100 * 1024 * 1024, // 100 MB
];

fn api_buffer_bench(c: &mut Criterion) {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    let mut group = c.benchmark_group("api_buffer_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        // Daten vorbereiten
        let src: Vec<u8> = vec![1; nbytes];
        let mut dst: Vec<u8> = vec![0; nbytes];

        // Buffer einmal anlegen und initialisieren

        group.bench_function(format!("copy_bytes_{}", nbytes), |b| {
            b.iter(|| {
                let buf = ctx.create_empty_buffer::<u8>(nbytes).unwrap(); // Empty
                let buf = buf.write_block(&queue, &src).unwrap(); // Ready
                buf.read_blocking(&queue, black_box(&mut dst)).unwrap();
            });
        });
    }

    group.finish();
}

fn raw_buffer_bench(c: &mut Criterion) {
    let device_ids =
        get_all_devices(opencl3::device::CL_DEVICE_TYPE_GPU).expect("Kein GPU gefunden");
    let ctx = CLContext::from_devices(&device_ids, &[], None, ptr::null_mut()).unwrap();
    let queue = CommandQueue::create(&ctx, device_ids[0], 0).unwrap();

    let mut group = c.benchmark_group("raw_buffer_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        let src: Vec<u8> = vec![1; nbytes];
        let mut dst: Vec<u8> = vec![0; src.len()];

        group.bench_function(format!("copy_only_{}", nbytes), |b| {
            b.iter(|| {
                let mut buf: CLBuffer<u8> =
                    CLBuffer::create(&ctx, CL_MEM_READ_WRITE, src.len(), ptr::null_mut()).unwrap();
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

fn api_read_bench(c: &mut Criterion) {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    let mut group = c.benchmark_group("api_read_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        // Daten vorbereiten
        let src: Vec<u8> = vec![1; nbytes];
        let mut dst: Vec<u8> = vec![0; nbytes];

        // Buffer einmal anlegen und initialisieren
        let buf = ctx.create_empty_buffer::<u8>(nbytes).unwrap(); // Empty
        let buf = buf.write_block(&queue, &src).unwrap(); // Ready

        group.bench_function(format!("copy_bytes_{}", nbytes), |b| {
            b.iter(|| {
                buf.read_blocking(&queue, black_box(&mut dst)).unwrap();
            });
        });
    }

    group.finish();
}

fn raw_read_bench(c: &mut Criterion) {
    let device_ids =
        get_all_devices(opencl3::device::CL_DEVICE_TYPE_GPU).expect("Kein GPU gefunden");
    let ctx = CLContext::from_devices(&device_ids, &[], None, ptr::null_mut()).unwrap();
    let queue = CommandQueue::create(&ctx, device_ids[0], 0).unwrap();

    let mut group = c.benchmark_group("raw_read_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        let src: Vec<u8> = vec![1; nbytes];
        let mut dst: Vec<u8> = vec![0; src.len()];
        let mut buf: CLBuffer<u8> =
            CLBuffer::create(&ctx, CL_MEM_READ_WRITE, src.len(), ptr::null_mut()).unwrap();
        queue
            .enqueue_write_buffer(&mut buf, CL_BLOCKING, 0, black_box(&src), &[])
            .unwrap();

        group.bench_function(format!("copy_only_{}", nbytes), |b| {
            b.iter(|| {
                queue
                    .enqueue_read_buffer(&buf, CL_BLOCKING, 0, black_box(&mut dst), &[])
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn api_write_bench(c: &mut Criterion) {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    let mut group = c.benchmark_group("api_write_bench");

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
                buf.overwrite_blocking_for_bench(&queue, black_box(&src))
                    .unwrap(); //InFlight
            });
        });
    }

    group.finish();
}

fn raw_write_bench(c: &mut Criterion) {
    let device_ids =
        get_all_devices(opencl3::device::CL_DEVICE_TYPE_GPU).expect("Kein GPU gefunden");
    let ctx = CLContext::from_devices(&device_ids, &[], None, ptr::null_mut()).unwrap();
    let queue = CommandQueue::create(&ctx, device_ids[0], 0).unwrap();

    let mut group = c.benchmark_group("raw_write_bench");

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
            });
        });
    }

    group.finish();
}

fn api_full_bench(c: &mut Criterion) {
    // Verschiedene Buffer-Größen
    const SIZES: &[usize] = &[1024, 4096, 16384, 65536, 262144, 1048576];

    // OpenCL Setup
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Kernel Source
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

    // Kernel kompilieren
    let kernel = hpc_core::api::Kernel::from_source(&ctx, kernel_source, "vector_add").unwrap();

    let mut group = c.benchmark_group("api_full_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        // Test-Daten vorbereiten
        let a: Vec<u8> = vec![1; nbytes];
        let b: Vec<u8> = vec![1; nbytes];
        let mut result: Vec<u8> = vec![0; nbytes];

        group.bench_function(format!("vector_add_{}", nbytes), |bench| {
            bench.iter(|| {
                // Das wird gemessen: Kompletter Vector Addition Workflow

                // 1. Buffer erstellen und Daten hochladen
                let buffer_a = ctx
                    .create_empty_buffer::<u8>(nbytes)
                    .unwrap()
                    .write_block(&queue, black_box(&a))
                    .unwrap();

                let buffer_b = ctx
                    .create_empty_buffer::<u8>(nbytes)
                    .unwrap()
                    .write_block(&queue, black_box(&b))
                    .unwrap();

                let buffer_result = ctx
                    .create_empty_buffer::<u8>(nbytes)
                    .unwrap()
                    .write_block(&queue, black_box(&result))
                    .unwrap();

                // 2. Kernel-Argumente setzen
                kernel.set_arg_buffer(0, &buffer_a).unwrap();
                kernel.set_arg_buffer(1, &buffer_b).unwrap();
                kernel.set_arg_buffer(2, &buffer_result).unwrap();
                kernel.set_arg_scalar(3, &(nbytes as u32)).unwrap();

                // 3. Kernel ausführen
                let (inflight_buffer, event) = buffer_result
                    .enqueue_kernel(&queue, &kernel, nbytes)
                    .unwrap();

                // 4. Warten und Ergebnis zurücklesen
                let result_buffer = event.wait(inflight_buffer);
                result_buffer
                    .read_blocking(&queue, black_box(&mut result))
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn raw_full_bench(c: &mut Criterion) {
    // Verschiedene Buffer-Größen
    const SIZES: &[usize] = &[1024, 4096, 16384, 65536, 262144, 1048576];

    // OpenCL Setup (raw)
    let device_ids = opencl3::device::get_all_devices(opencl3::device::CL_DEVICE_TYPE_GPU)
        .expect("Kein GPU gefunden");
    let ctx =
        opencl3::context::Context::from_devices(&device_ids, &[], None, ptr::null_mut()).unwrap();
    let queue = opencl3::command_queue::CommandQueue::create(&ctx, device_ids[0], 0).unwrap();

    // Kernel Source
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

    // Program und Kernel kompilieren (raw OpenCL) mit Error-Handling
    let mut program = opencl3::program::Program::create_from_source(&ctx, kernel_source).unwrap();

    // Build mit Error-Handling
    match program.build(&device_ids, "") {
        Ok(_) => {}
        Err(e) => {
            // Build-Log ausgeben bei Fehlern
            for &device_id in &device_ids {
                if let Ok(build_log) = program.get_build_log(device_id) {
                    eprintln!("Build Error für Device {:?}: {}", device_id, build_log);
                }
            }
            panic!("Program build failed: {:?}", e);
        }
    }

    let kernel = opencl3::kernel::Kernel::create(&program, "vector_add").unwrap();

    let mut group = c.benchmark_group("raw_full_bench");

    for &nbytes in SIZES {
        group.throughput(Throughput::Bytes(nbytes as u64));

        // Test-Daten vorbereiten
        let a: Vec<u8> = vec![1; nbytes];
        let b: Vec<u8> = vec![1; nbytes];
        let mut result: Vec<u8> = vec![0; nbytes];

        group.bench_function(format!("vector_add_{}", nbytes), |bench| {
            bench.iter(|| {
                // Das wird gemessen: Raw OpenCL Vector Addition

                // 1. Buffer erstellen (raw)
                let mut buffer_a = opencl3::memory::Buffer::<u8>::create(
                    &ctx,
                    opencl3::memory::CL_MEM_READ_WRITE as opencl3::types::cl_mem_flags,
                    nbytes,
                    ptr::null_mut(),
                )
                .unwrap();

                let mut buffer_b = opencl3::memory::Buffer::<u8>::create(
                    &ctx,
                    opencl3::memory::CL_MEM_READ_WRITE as opencl3::types::cl_mem_flags,
                    nbytes,
                    ptr::null_mut(),
                )
                .unwrap();

                let mut buffer_result = opencl3::memory::Buffer::<u8>::create(
                    &ctx,
                    opencl3::memory::CL_MEM_READ_WRITE as opencl3::types::cl_mem_flags,
                    nbytes,
                    ptr::null_mut(),
                )
                .unwrap();

                // 2. Daten hochladen (raw)
                queue
                    .enqueue_write_buffer(
                        &mut buffer_a,
                        opencl3::types::CL_BLOCKING,
                        0,
                        black_box(&a),
                        &[],
                    )
                    .unwrap();

                queue
                    .enqueue_write_buffer(
                        &mut buffer_b,
                        opencl3::types::CL_BLOCKING,
                        0,
                        black_box(&b),
                        &[],
                    )
                    .unwrap();

                queue
                    .enqueue_write_buffer(
                        &mut buffer_result,
                        opencl3::types::CL_BLOCKING,
                        0,
                        black_box(&result),
                        &[],
                    )
                    .unwrap();

                // 3. Kernel-Argumente setzen (raw)
                kernel.set_arg(0, &buffer_a).unwrap();
                kernel.set_arg(1, &buffer_b).unwrap();
                kernel.set_arg(2, &buffer_result).unwrap();
                kernel.set_arg(3, &(nbytes as u32)).unwrap();

                // 4. Kernel ausführen (raw)
                let _event = queue
                    .enqueue_nd_range_kernel(
                        kernel.get(),            // Raw pointer mit .get()
                        1,                       // work_dim
                        ptr::null(),             // global_work_offset
                        &nbytes as *const usize, // global_work_size
                        ptr::null(),             // local_work_size
                        &[],                     // event_wait_list
                    )
                    .unwrap();

                // 5. Warten und Ergebnis zurücklesen (raw)
                _event.wait().unwrap();

                queue
                    .enqueue_read_buffer(
                        &buffer_result,
                        opencl3::types::CL_BLOCKING,
                        0,
                        black_box(&mut result),
                        &[],
                    )
                    .unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    api_buffer_bench,
    raw_buffer_bench,
    api_read_bench,
    raw_read_bench,
    api_write_bench,
    raw_write_bench,
    api_full_bench,
    raw_full_bench,
);
criterion_main!(benches);
