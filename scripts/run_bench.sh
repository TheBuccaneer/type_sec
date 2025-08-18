#!/usr/bin/env bash
set -euo pipefail

# --- Defaults (clean) ---
GPU_IDS="${GPU_IDS:-0}"
LOCK_GC="${LOCK_GC:-}"
POWER_LIMIT="${POWER_LIMIT:-}"
CPU_CORES="${CPU_CORES:-}"
SET_PERF_CPU="${SET_PERF_CPU:-0}"
RESTORE_ON_EXIT=1

# --- Helpers --------------------------------------------------------------
require() { command -v "$1" >/dev/null 2>&1 || { echo "Missing: $1"; exit 1; }; }
is_root() { [ "${EUID:-$(id -u)}" -eq 0 ]; }
sudo_wrap() { if is_root; then "$@"; else sudo "$@"; fi; }

# --- Checks ----------------------------------------------------------------
require nvidia-smi
[ -z "${CPU_CORES}" ] || require taskset
if [ "${SET_PERF_CPU}" = "1" ]; then require cpupower; fi

# --- Snapshot aktuelle Settings (für Restore) ------------------------------
ORIG_PM=() ORIG_PL=() 
IFS=',' read -ra GPU_ARR <<<"${GPU_IDS}"

for gid in "${GPU_ARR[@]}"; do
  ORIG_PM[$gid]=$(nvidia-smi -i "$gid" --query-gpu=persistence_mode --format=csv,noheader 2>/dev/null || echo "N/A")
  ORIG_PL[$gid]=$(nvidia-smi -i "$gid" --query-gpu=power.limit --format=csv,noheader,nounits 2>/dev/null || echo "")
done

# CPU-Governors sichern (pro CPU)
TMP_GOV=$(mktemp)
if [ "${SET_PERF_CPU}" = "1" ]; then
  for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo "$(basename "$(dirname "$f")") $(cat "$f")" >> "$TMP_GOV" || true
  done
fi

restore() {
  # GPU reset
  for gid in "${GPU_ARR[@]}"; do
    # Clocks reset
    sudo_wrap nvidia-smi -i "$gid" -rgc || true
    # Power-Limit zurück
    if [ -n "${ORIG_PL[$gid]}" ]; then
      sudo_wrap nvidia-smi -i "$gid" -pl "${ORIG_PL[$gid]}" || true
    fi
    # Persistence Mode zurück
    case "${ORIG_PM[$gid]}" in
      Enabled)  sudo_wrap nvidia-smi -i "$gid" -pm 1 || true ;;
      Disabled) sudo_wrap nvidia-smi -i "$gid" -pm 0 || true ;;
    esac
  done

  # CPU-Governors zurück
  if [ -f "$TMP_GOV" ]; then
    while read -r cpu gov; do
      f="/sys/devices/system/cpu/${cpu}/cpufreq/scaling_governor"
      [ -w "$f" ] && echo "$gov" | sudo_wrap tee "$f" >/dev/null || true
    done < "$TMP_GOV"
    rm -f "$TMP_GOV"
  fi
}
if [ "$RESTORE_ON_EXIT" -eq 1 ]; then trap restore EXIT; fi

# --- Apply Stable Settings --------------------------------------------------
for gid in "${GPU_ARR[@]}"; do
  # Persistence Mode on
  sudo_wrap nvidia-smi -i "$gid" -pm 1

  # Optional: Power-Limit setzen (Watt)
  if [ -n "$POWER_LIMIT" ]; then
    sudo_wrap nvidia-smi -i "$gid" -pl "$POWER_LIMIT"
  fi

  # Optional: GPU Core Clocks locken (min,max MHz). Reset via -rgc.
  if [ -n "$LOCK_GC" ]; then
    sudo_wrap nvidia-smi -i "$gid" -lgc "$LOCK_GC"
  fi
done

# CPU Governor -> performance (alle CPUs)
if [ "${SET_PERF_CPU}" = "1" ]; then
  sudo_wrap cpupower frequency-set -g performance
fi

# --- Run command -----------------------------------------------------------
if [ $# -eq 0 ]; then
  echo "Usage: GPU_IDS=0 LOCK_GC=1695,1695 POWER_LIMIT=300 CPU_CORES=2-11 ./scripts/run_bench.sh <command> [args]"
  exit 2
fi

if [ -n "$CPU_CORES" ]; then
  exec taskset -c "$CPU_CORES" "$@"
else
  exec "$@"
fi
