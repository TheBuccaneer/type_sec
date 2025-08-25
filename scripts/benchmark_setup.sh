#!/bin/bash
# GPU/CPU Performance Stabilization Script for Benchmarking
# Run this before executing benchmarks for consistent results

echo "=========================================="
echo "Benchmark Performance Setup Script"
echo "=========================================="

# [TR] Optional: NUMA-Node für Threadripper auswählbar (Default: 0)
BENCH_NUMA_NODE="${BENCH_NUMA_NODE:-0}"

# Check if running as root (required for some operations)
if [[ $EUID -eq 0 ]]; then
    echo "Warning: Running as root. Some operations might not be necessary."
fi

echo "1. Setting CPU Governor to Performance..."
# Set all CPU cores to performance mode
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    if [ -w "$cpu" ]; then
        echo performance | sudo tee "$cpu" > /dev/null
        echo "  Set $(basename $(dirname $cpu)) to performance"
    fi
done
# [TR] Sicherstellen, dass wirklich der Performance-Governor aktiv ist (Zen/amd_pstate)
if command -v cpupower >/dev/null 2>&1; then
    sudo cpupower frequency-set -g performance >/dev/null 2>&1
fi

echo "2. Disabling CPU Turbo Boost for consistency..."
# Disable Intel Turbo Boost for consistent timing
if [ -w /sys/devices/system/cpu/intel_pstate/no_turbo ]; then
    echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo > /dev/null
    echo "  Intel Turbo Boost disabled"
elif [ -w /sys/devices/system/cpu/cpufreq/boost ]; then
    echo 0 | sudo tee /sys/devices/system/cpu/cpufreq/boost > /dev/null
    echo "  AMD Boost disabled"
fi

echo "3. Setting GPU to Maximum Performance..."
# NVIDIA GPU Performance Mode
if command -v nvidia-smi &> /dev/null; then
    # Set persistence mode (prevents driver unload)
    sudo nvidia-smi -pm ENABLED > /dev/null 2>&1
    
    # Set maximum power limit (if supported)
    sudo nvidia-smi -pl $(nvidia-smi -q -d POWER | grep "Max Power Limit" | cut -d: -f2 | cut -d. -f1 | tr -d ' ') > /dev/null 2>&1
    
    # Set application clocks to maximum (if supported)
    sudo nvidia-smi -ac $(nvidia-smi -q -d SUPPORTED_CLOCKS | grep "Memory" | tail -1 | cut -d: -f2 | cut -d, -f1 | tr -d ' '),$(nvidia-smi -q -d SUPPORTED_CLOCKS | grep "Graphics" | head -1 | cut -d: -f2 | tr -d ' ') > /dev/null 2>&1
    
    echo "  NVIDIA GPU set to maximum performance"
else
    echo "  nvidia-smi not found, skipping GPU setup"
fi

echo "4. Optimizing System Settings..."
# Disable CPU frequency scaling
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor > /dev/null 2>&1

# [TR] NUMA-Autobalancing deaktivieren (stabilere Latenzen auf Multi-Die-CPUs)
if [ -w /proc/sys/kernel/numa_balancing ]; then
    echo 0 | sudo tee /proc/sys/kernel/numa_balancing > /dev/null
fi

# Set I/O scheduler to deadline for SSDs (better for benchmark consistency)
for disk in /sys/block/sd*/queue/scheduler; do
    if [ -w "$disk" ]; then
        echo deadline | sudo tee "$disk" > /dev/null
    fi
done

# Increase VM dirty ratio for better I/O performance
echo 15 | sudo tee /proc/sys/vm/dirty_background_ratio > /dev/null
echo 30 | sudo tee /proc/sys/vm/dirty_ratio > /dev/null

echo "5. Memory and Process Optimization..."
# Drop caches to ensure clean memory state
sudo sync
echo 3 | sudo tee /proc/sys/vm/drop_caches > /dev/null

# Set high priority and CPU affinity for benchmark process (optional)
# This will be done when running the actual benchmark

echo "6. Disabling Power Management Features..."
# Disable CPU C-states (keeps CPU at full speed)
if [ -w /sys/devices/system/cpu/cpu0/cpuidle/state1/disable ]; then
    for state in /sys/devices/system/cpu/cpu*/cpuidle/state*/disable; do
        echo 1 | sudo tee "$state" > /dev/null 2>&1
    done
    echo "  CPU C-states disabled"
fi

# Disable PCIe ASPM (Active State Power Management)
if [ -w /sys/module/pcie_aspm/parameters/policy ]; then
    echo performance | sudo tee /sys/module/pcie_aspm/parameters/policy > /dev/null 2>&1
    echo "  PCIe ASPM set to performance"
fi

echo "7. Final System Status Check..."
# Show current CPU governor
echo "  CPU Governor: $(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null)"

# [TR] Anzeigen, ob NUMA-Autobalancing aus ist
if [ -r /proc/sys/kernel/numa_balancing ]; then
    echo "  NUMA balancing: $(cat /proc/sys/kernel/numa_balancing)"
fi

# Show current GPU status
if command -v nvidia-smi &> /dev/null; then
    echo "  GPU Status:"
    nvidia-smi --query-gpu=name,power.draw,clocks.gr,clocks.mem,temperature.gpu --format=csv,noheader,nounits | head -1 | awk -F, '{printf "    %s: %sW, %sMHz/%sMHz, %s°C\n", $1, $2, $3, $4, $5}'
fi

echo ""
echo "=========================================="
echo "System optimized for benchmarking!"
echo "=========================================="
echo ""
echo "Now run your benchmark with:"
# [TR] Empfehlung: auf einen NUMA-Knoten pinnen (Threadripper)
if command -v numactl >/dev/null 2>&1; then
  echo "  numactl --cpunodebind=${BENCH_NUMA_NODE} --membind=${BENCH_NUMA_NODE} cargo bench"
else
  echo "  cargo bench    # (tip: install 'numactl' and pin to a NUMA node for Threadripper)"
fi
echo ""
echo "After benchmarking, run: ./benchmark-cleanup.sh"
echo "to restore normal system settings."
