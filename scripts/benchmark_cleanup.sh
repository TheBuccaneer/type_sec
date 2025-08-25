#!/bin/bash
# Cleanup: restore system settings after benchmarking

echo "=========================================="
echo "Benchmark Cleanup Script"
echo "=========================================="

# --- GPU (NVIDIA) ---------------------------------------------------------
if command -v nvidia-smi &>/dev/null; then
  echo "1) Resetting NVIDIA GPU application clocks & power limit..."
  # Reset application clocks to default
  sudo nvidia-smi -rac >/dev/null 2>&1
  # Reset power limit to default
  DEF_PL=$(nvidia-smi -q -d POWER | awk -F: '/Default Power Limit/{gsub(/[ W]/,"",$2); print int($2)}' | head -1)
  if [[ -n "$DEF_PL" ]]; then
    sudo nvidia-smi -pl "$DEF_PL" >/dev/null 2>&1
  fi
  # Disable persistence mode
  sudo nvidia-smi -pm DISABLED >/dev/null 2>&1
fi

# --- CPU Frequenz/Governor ------------------------------------------------
echo "2) Restoring CPU frequency governor..."
for gov in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
  if [[ -w "$gov" ]]; then
    # Prefer schedutil if available, otherwise ondemand
    if grep -qw schedutil "$gov"; then
      echo schedutil | sudo tee "$gov" >/dev/null
    else
      echo ondemand | sudo tee "$gov" >/dev/null
    fi
  fi
done
if command -v cpupower >/dev/null 2>&1; then
  if cpupower frequency-info | grep -q "Available cpufreq governors.*schedutil"; then
    sudo cpupower frequency-set -g schedutil >/dev/null 2>&1
  else
    sudo cpupower frequency-set -g ondemand  >/dev/null 2>&1
  fi
fi

# --- Turbo/Boost ----------------------------------------------------------
echo "3) Re-enabling CPU turbo/boost where applicable..."
# Intel
if [[ -w /sys/devices/system/cpu/intel_pstate/no_turbo ]]; then
  echo 0 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo >/dev/null
fi
# AMD (Threadripper etc.)
if [[ -w /sys/devices/system/cpu/cpufreq/boost ]]; then
  echo 1 | sudo tee /sys/devices/system/cpu/cpufreq/boost >/dev/null
fi

# --- NUMA Balancing -------------------------------------------------------
echo "4) Re-enabling NUMA auto-balancing..."
if [[ -w /proc/sys/kernel/numa_balancing ]]; then
  echo 1 | sudo tee /proc/sys/kernel/numa_balancing >/dev/null
fi

# --- CPU C-States ---------------------------------------------------------
echo "5) Re-enabling CPU C-states..."
for f in /sys/devices/system/cpu/cpu*/cpuidle/state*/disable; do
  [[ -w "$f" ]] && echo 0 | sudo tee "$f" >/dev/null
done

# --- PCIe ASPM ------------------------------------------------------------
echo "6) Restoring PCIe ASPM policy..."
if [[ -w /sys/module/pcie_aspm/parameters/policy ]]; then
  OPTS=$(cat /sys/module/pcie_aspm/parameters/policy 2>/dev/null)
  if echo "$OPTS" | grep -qw default; then VAL=default
  elif echo "$OPTS" | grep -qw powersave; then VAL=powersave
  else VAL=performance
  fi
  echo "$VAL" | sudo tee /sys/module/pcie_aspm/parameters/policy >/dev/null
fi

# --- I/O Scheduler --------------------------------------------------------
echo "7) Restoring I/O schedulers on SCSI/SATA disks..."
for sch in /sys/block/sd*/queue/scheduler; do
  if [[ -w "$sch" ]]; then
    OPTS=$(cat "$sch")
    # choose a sane default present in the list
    if echo "$OPTS" | grep -qw none; then DEF=none
    elif echo "$OPTS" | grep -qw mq-deadline; then DEF=mq-deadline
    elif echo "$OPTS" | grep -qw cfq; then DEF=cfq
    else DEF=deadline
    fi
    echo "$DEF" | sudo tee "$sch" >/dev/null
  fi
done

# --- VM dirty ratios ------------------------------------------------------
echo "8) Restoring vm.dirty ratios..."
echo 10 | sudo tee /proc/sys/vm/dirty_background_ratio >/dev/null
echo 20 | sudo tee /proc/sys/vm/dirty_ratio >/dev/null

echo "=========================================="
echo "Cleanup done. System restored to sane defaults."
echo "=========================================="
