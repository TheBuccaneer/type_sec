# Reproduction Guide

This document describes how to reproduce the benchmarks for the RUST IT project.

## System requirements

- **OS**: Linux x86_64 (tested on Ubuntu 22.04).
- **GPU/Driver**: NVIDIA-SMI 570.169                Driver Version: 570.169        CUDA Version: 12.8  
- **OpenCL runtime**: OpenCL 3.0 CUDA 12.8.97
- **Rust**: version 1.89.0.
- **CPU**: AMD Ryzen Threadripper 3970X 32-Core Processor
- **GPU**: NVIDIA GeForce RTX 3090

## Environment Setup

For reproducible performance, CPU and GPU clocks must be stabilized before benchmarking.  
Scripts are provided in the scripts/ folder.

Run **before** starting the benchmarks:

```bash
./scripts/benchmark-setup.sh
```

This script will:
- set the CPU governor to performance and disable turbo/boost,
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


Bandwidth checked with 'cuda_benchmark.cu' and 'opencl_benchmark.c' in 'scripts/'

## Running Example

```bash
cargo run --example simple_vector_add
```

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

## Results Archive
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
- CPU governor back to schedutil/ondemand,
- Turbo/Boost re-enabled,
- NUMA balancing and C-states back on,
- GPU clocks and power limit reset to defaults,
- I/O scheduler and vm.dirty ratios restored.


## Compile-Fail Evidence

We additionally safeguard protocol errors through **Trybuild** tests. Test cases live in tests/compile_fail/*.rs alongside their expected
.stderr snapshots


Run:
```bash
TRYBUILD=overwrite cargo test -p hpc-core --tests
```

Each test contains a .rs file with a deliberately erroneous example as well as the expected compiler output (.stderr).

The complete snapshots of the test runs are available in the artifact at:

results/YYYY-MM-DD/trybuild_errors

In the paper, we document the cases covered in paper/evaluation.md.
The assignment of SPEC rule ↔ test case can be found in
SPEC-tests-map.md.

## Zero-Cost Abstraction (compile time)

To check that the abstraction does not add code-size overhead, we compare
our API against raw OpenCL using `cargo-bloat`.

```bash
# API (treatment)
cargo bloat -p hpc-core --release --example bloat_target -n 20 \
  > results/YYYY-MM-DD/bloat/top20_api.txt

# Raw OpenCL (baseline)
cargo bloat -p hpc-core --release --example bloat_target_opencl -n 20 \
  > results/YYYY-MM-DD/bloat/top20_base.txt

# Diff
diff -u results/YYYY-MM-DD/bloat/top20_base.txt \
       results/YYYY-MM-DD/bloat/top20_api.txt \
  > results/YYYY-MM-DD/bloat/diff_top20.txt
```

expect 0–2 % difference in top-20 functions.


## Assembly Spot Checks

To verify that type parameters and state markers compile away completely,
we inspect the generated assembly for selected hot-path functions.

Example:

```bash
cd crates/hpc-core
cargo asm --lib 'hpc_core::buffer::empty::<impl hpc_core::buffer::GpuBuffer<hpc_core::buffer::state::Empty>>::write_block'
  ```

  Inspect the .s file under results/2025-08-D21/asm/ to verify:
  no additional branches or loops are introduced
  only thin wrappers around calls remain
  PhantomData and state markers vanish completely.