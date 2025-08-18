#!/usr/bin/env bash
set -euo pipefail

# --- Config ---
RESULTS_DIR="results/2025-08-18"
mkdir -p "$RESULTS_DIR"

# --- Environment capture (no privileged changes) ---
{ 
  echo "# System & Toolchain"
  date -Is
  echo "uname -a:"; uname -a || true
  echo; echo "lscpu:"; lscpu || true
  echo; echo "lsmem (summary):"; lsmem || true
  echo; echo "GPU/OpenCL:"; which clinfo && clinfo || true
  echo; echo "Rust toolchain:"; rustc -Vv || true; cargo -V || true
} > "$RESULTS_DIR/env.txt"

# --- Optional: governor/turbo hints (manual) ---
cat > "$RESULTS_DIR/notes_governor.txt" <<'EOF'
Optional (manuell, Root erforderlich):
  sudo cpupower frequency-set -g performance
  echo 0 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo
Nach den Benches ggf. rückgängig machen.
EOF

# --- Run benches (no plots to save time) ---
echo "[run] cargo bench --all -- --noplot"
cargo bench --all -- --noplot | tee "$RESULTS_DIR/bench_stdout.txt"

# --- Copy criterion reports (HTML/SVG) to results snapshot ---
echo "[copy] criterion reports → $RESULTS_DIR/criterion_reports/"
mkdir -p "$RESULTS_DIR/criterion_reports"
# Copy from each crate's local target/criterion (workspace-aware)
shopt -s nullglob
for CR in crates/*; do
  if [[ -d "$CR/target/criterion" ]]; then
    # mirror estimates.json and report/ for each bench
    rsync -a --include '*/' --include '*estimates.json' --include 'report/***' --exclude '*'       "$CR/target/criterion/" "$RESULTS_DIR/criterion_reports/${CR##*/}/" 2>/dev/null || true
  fi
done

# --- Parse medians to CSV ---
echo "[parse] medians → $RESULTS_DIR/criterion_medians.csv"
python3 parse_criterion.py > "$RESULTS_DIR/criterion_medians.csv" || true

echo "Done. Artifacts in: $RESULTS_DIR"
