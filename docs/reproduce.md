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

nvidia-smi -q | grep -A 5 "PCI"
    PCI
        Bus                               : 0x01
        Device                            : 0x00
        Domain                            : 0x0000
        Base Classcode                    : 0x3
        Sub Classcode                     : 0x0
--
            PCIe Generation
                Max                       : 3
                Current                   : 1
                Device Current            : 1
                Device Max                : 4
                Host Max                  : 3
--
            SRAM PCIE                     : N/A
            SRAM Other                    : N/A
    Retired Pages
        Single Bit ECC                    : N/A
        Double Bit ECC                    : N/A
        Pending Page Blacklist            : N/A


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

### Results Archive
The raw outputs of our 5 benchmark runs are stored in:

results/2025-08-25/criterion/run1
results/2025-08-25/criterion/run2
results/2025-08-25/criterion/run3
results/2025-08-25/criterion/run4
results/2025-08-25/criterion/run5

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
