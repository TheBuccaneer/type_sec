# SPEC ↔ Test Mapping

This document links each SPEC rule (EPS) to the corresponding trybuild
test case. This makes it transparent which protocol violation is covered
by which compile-fail test.

| Rule-ID | Description                        | Test file                                   |
|---------|------------------------------------|---------------------------------------------|
| F1      | Write on buffer in InFlight        | tests/compile_fail/api_inflight_write.rs    |
| F2      | Read on buffer in InFlight         | tests/compile_fail/api_inflight_read.rs     |
| F3      | Mapping from InFlight              | tests/compile_fail/api_inflight_map.rs      |
| F4      | Kernel launch from Empty           | tests/compile_fail/api_empty_kernel.rs      |
| F5      | EventToken used without wait()     | tests/compile_fail/api_no_event_use.rs      |
| F6      | Double wait() on EventToken        | tests/compile_fail/api_wouble_wait.rs       |
| F7      | wait() on wrong state (Written)    | tests/compile_fail/api_wait_on_written.rs   |
| F8      | Forgotten unmap / MapToken unused  | tests/compile_fail/api_forget_unmap.rs      |
| F9      | Kernel argument ABI mismatch       | tests/compile_fail/api_wrong_arg.rs         |

