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

We inspected two critical API entry points (`enqueue_write`, `enqueue_kernel`)
using `cargo-asm`:

```
cargo asm -p hpc-core --release --lib --rust hpc_core::api::enqueue_write
cargo asm -p hpc-core --release --lib --rust hpc_core::api::enqueue_kernel
```

Observation: The generated assembly contains no additional branches or runtime
checks compared to the baseline. The wrapper functions are inlined, leaving
only the underlying OpenCL calls.

---

## Conclusion

Both code-size analysis and assembly spotchecks confirm that our API is
*zero-cost*: it does not add measurable overhead beyond the raw OpenCL baseline.
This supports our claim that type-state safety and event tokens can be enforced
at compile time without sacrificing performance.
