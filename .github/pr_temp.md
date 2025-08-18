# Zero-Cost Abstraction Probes & Documentation

## Summary
This PR introduces a new set of **zero-cost abstraction probes** (`bloat_probe`, `bloat_typestates_probe`, `bloat_raw_probe`, `bloat_api_probe`) and a detailed documentation file `docs/zero_cost_full_with_fazit.md`.

The goal is to provide scientific evidence that our typestate-based GPU buffer API incurs **no runtime overhead** compared to raw operations.

## Changes
- Added four probes under `--features bloat-probe`:
  - `bloat_probe::bloat_hotpath_probe_entry` (baseline)
  - `bloat_typestates_probe::bloat_hotpath_typestates_entry` (typestate hotpath)
  - `bloat_raw_probe::bloat_hotpath_raw_entry` (raw hotpath)
  - `bloat_api_probe::bloat_hotpath_api_entry` (API hotpath)
- Added detailed **zero-cost.md** documentation including:
  - Definitions and references (Rust Embedded Book, Stroustrup, StackOverflow)
  - Assembly output for all probes
  - Comparison & interpretation
  - Visual ASCII diagram
  - Final conclusion

## Motivation
To strengthen the scientific contribution of this project by demonstrating that:
- Typestate-based APIs (PhantomData, marker states) compile away entirely.
- The API abstraction introduces **no runtime cost** (identical assembly).
- Supports the paper’s claim of “zero-cost abstractions in GPU programming with Rust.”

## Verification
- Ran `cargo build -p hpc-core --release --features bloat-probe`
- Verified assembly equivalence with `cargo-asm`
- Confirmed identical ASM sequences for Baseline, Typestate, Raw, API

## Next Steps
- Merge into `main` after review
- Reference this doc in upcoming paper submission
- Optionally: include diagram export in LaTeX/PDF

---

**Reviewer Checklist:**
- [ ] Code compiles cleanly (`cargo build`)
- [ ] No warnings (`cargo fix` run)
- [ ] Documentation is clear and complete
- [ ] Evidence of zero-cost abstraction is reproducible
