# Zero-Cost Abstractions

We verified that our type-state and branding abstractions compile away completely:  
they impose **little runtime cost** compared to handwritten OpenCL.

- **Method**: inspected generated assembly and compared code size with `cargo-bloat`.  
- **Result**: delta < 2%, no additional branches or loops beyond the raw baseline.  
- **Conclusion**: *type states and lifetimes are zero-cost abstractions*.

See [reproduce.md] for full commands and [evaluation.md]for detailed results.

---

**Example (Assembly excerpt):**

```asm
; hot-path write_block
; wrapper inlined, only clEnqueueWriteBuffer remains
callq  clEnqueueWriteBuffer
```