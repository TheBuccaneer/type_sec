# Reproduce Benchmark Results

1. Stelle sicher, dass du die richtige Toolchain nutzt:
   ```bash
   rustup show

SET_PERF_CPU=1 CPU_CORES=2-11 ./scripts/run_bench.sh

cargo bench -p hpc-core
