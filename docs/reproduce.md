# Reproduction Guide

This document describes how to reproduce the benchmarks for the RUST IT project.

## Environment Setup

For reproducible performance, CPU and GPU clocks must be stabilized before benchmarking.  
Scripts are provided in the `scripts/` folder.

Run **before** starting the benchmarks:

```bash
./scripts/benchmark-setup.sh
```

This script will:
- set the CPU governor to `performance` and disable turbo/boost,
- fix NVIDIA GPU clocks, enable persistence mode, and pin a NUMA node (Threadripper),
- disable NUMA auto-balancing and C-states,
- flush caches and set I/O scheduler for stable I/O timing.

**Note:**: In our measurements the GPU link was pinned to PCIe Gen3,
since higher link speeds caused Bandwidth stability issues on our platform.

Bandwidth checked with `cuda_benchmark.cu` and `opencl_benchmark.c` in `scripts/`

## Running the Benchmarks

To run the memcpy benchmarks (Criterion):

```bash
cargo bench -p hpc-core --benches
```

This will execute for example:

- `crates/hpc-core/benches/memcpy_api_criterion.rs`  
  (measures `api_buffer_bench` and `raw_buffer_bench` for buffer create + read + write pipe with varying sizes)
  (measures `api_read_bench` and `raw_read_bench` for pure reading performance with varying sizes)
  (measures `api_write_bench` and `raw_write_bench` for pure writing performance with varying sizes)
  (measures `api_full_bench` and `raw_full_bench` for full pipline buffer create + write + execute kernel + read with varying sizes)

Criterion outputs throughput numbers (`MiB/s` or `GiB/s`) and plots.

## Cleanup

After benchmarking, restore the system defaults:

```bash
./scripts/benchmark-cleanup.sh
```

This will reset:
- CPU governor back to `schedutil`/`ondemand`,
- Turbo/Boost re-enabled,
- NUMA balancing and C-states back on,
- GPU clocks and power limit reset to defaults,
- I/O scheduler and vm.dirty ratios restored.
