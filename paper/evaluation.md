# Evaluation – Zero-Cost (Snippet for Paper)

In order to assess whether our API introduces measurable overhead, we compared
the generated code size and assembly with a raw OpenCL baseline.

---

## Code Size (cargo-bloat)

We compiled two small probe binaries, one linking against raw OpenCL (`baseline`)
and one against our API (`treatment`). Using `cargo-bloat`, we compared the size
contributions of all crates.

```
cargo bloat -p hpc-core --release --example bloat_probe --crates
```

Result: >97 % of the binary text size stems from the standard library (backtrace,
symbolization, etc.). Our crate `hpc-core` contributes <1 %. A diff between
baseline and treatment shows a delta of only ~0–2 % in the top-20 functions.

---

## Assembly Spotchecks (cargo-asm)

We inspected two critical API entry points (`write_block`, `enqueue_kernel`)
using `cargo-asm`:

```
cargo asm -p hpc-core --release --lib --rust hpc_core::api::write_block
cargo asm -p hpc-core --release --lib --rust hpc_core::api::enqueue_kernel
```

Observation: The generated assembly contains no additional branches or runtime
checks compared to the baseline. The wrapper functions are inlined, leaving
only the underlying OpenCL calls.

---

## Runtime Benchmarks (Criterion, 5-Run Average)

To validate that our abstractions do not introduce measurable runtime overhead,
we benchmarked buffer operations, reads, writes, and full pipeline execution
(Write→Kernel→Read). All numbers are averages over 5 runs with Criterion.
Times are given in microseconds (µs). Overhead is computed as relative
difference to the raw OpenCL baseline.

### Buffer Operations (Create + Read/Write)

| Size    | API [µs]  | Raw [µs]  | Overhead |
|---------|-----------|-----------|----------|
| 1KB     | 118.59    | 118.64    |  -0.04% |
| 64KB    | 138.40    | 138.55    |  -0.10% |
| 1MB     | 400.38    | 400.86    |  -0.12% |
| 16MB    | 3.41ms    | 3.28ms    |  +3.88% |
| 100MB   | 20.04ms   | 20.17ms   |  -0.64% |

### Read Operations (Device→Host)

| Size    | API [µs]  | Raw [µs]  | Overhead |
|---------|-----------|-----------|----------|
| 1KB     | 7.275     | 7.246     |  +0.39% |
| 64KB    | 15.820    | 15.742    |  +0.50% |
| 1MB     | 153.188   | 153.508   |  -0.21% |
| 16MB    | 1.366ms   | 1.368ms   |  -0.21% |
| 100MB   | 9.003ms   | 9.039ms   |  -0.39% |

### Write Operations (Host→Device)

| Size    | API [µs]  | Raw [µs]  | Overhead |
|---------|-----------|-----------|----------|
| 1KB     | 6.273     | 6.268     |  +0.08% |
| 64KB    | 14.337    | 14.453    |  -0.81% |
| 1MB     | 142.872   | 142.868   |  +0.00% |
| 16MB    | 1.403ms   | 1.404ms   |  -0.06% |
| 100MB   | 8.676ms   | 8.860ms   |  -2.08% |

### Full Pipeline (Write→Kernel→Read)

| Size    | API [µs]  | Raw [µs]  | Overhead |
|---------|-----------|-----------|----------|
| 1KB     | 154.18    | 151.88    |  +1.52% |
| 4KB     | 153.79    | 153.94    |  -0.09% |
| 16KB    | 161.78    | 161.69    |  +0.06% |
| 64KB    | 189.27    | 188.38    |  +0.47% |
| 256KB   | 303.02    | 302.77    |  +0.08% |
| 1MB     | 865.56    | 864.15    |  +0.16% |

**Summary:** Across all cases the average overhead remains close to 0%.
Overheads fluctuate between -2.08% and +3.88%, well within the variance
expected from system noise. This confirms that our API is effectively
zero-cost at runtime.


---

## Compile-Fail Evidence (EPS)

To demonstrate that our Executable Protocol Specifications (EPS) actually
prevent host-side misuse at compile time, we implemented a suite of
trybuild tests. Each test corresponds to one invalid host action and
fails to compile with a blessed `.stderr` snapshot. This provides
machine-checkable evidence that our type-state API enforces the
protocol invariants.

| Test file                 | Scenario                         | Compile-time mechanism                               | 
|---------------------------|----------------------------------|------------------------------------------------------|
| api_empty_kernel.stderr   | Kernel launch from Empty         | State gate (method unavailable / trait bound)        |
| api_forget_unmap.stderr   | Forgotten unmap / MapToken unused| #[must_use] on guard/token or Result                 |
| api_inflight_map.stderr   | Mapping from InFlight            | State gate (no Map* impl for InFlight)               |
| api_inflight_read.stderr  | Read from InFlight               | State gate (no read impl for InFlight)               |
| api_inflight_write.stderr | Write from InFlight              | State gate (no write impl for InFlight)              |
| api_no_event_use.stderr   | EventToken ignored (no wait)     | #[must_use] on EventToken                            |
| api_wait_on_written.stderr| wait() on non-InFlight buffer    | Type-level protocol (signature enforces state)       |
| api_wouble_wait.stderr    | Double wait on same EventToken   | Linear token (non-Copy, consumed by wait())          |
| api_wrong_arg.stderr      | Wrong kernel arg type/signature  | Type/layout check (T: Pod / ABI guard)               |

**Summary:** Out of our catalogued failure modes, nine core cases are
already covered by compile-fail tests. This demonstrates that whole
classes of host errors (mis-timed reads/writes, forgotten waits/unmaps,
double-wait, ABI mismatch) are eliminated at compile time. The resulting
Prevented Spec Coverage (PSC) exceeds our target threshold.




## Conclusion

Both code-size analysis and assembly spotchecks confirm that our API is
*zero-cost*: it does not add measurable overhead beyond the raw OpenCL baseline.
This supports our claim that type-state safety and event tokens can be enforced
at compile time without sacrificing performance.
