# Cross-Mapping of API Concepts

This table shows how the main constructs of our API (`hpc-core`) relate to
concepts in OpenCL, CUDA, and SYCL. The mapping is only for orientation
purposes — we do not re-implement these APIs, but highlight where our
type-state model introduces differences.

| Our API (`hpc-core`)              | OpenCL (C-API)            | CUDA / SYCL (roughly)             | Notes |
|-----------------------------------|---------------------------|-----------------------------------|-------|
| `Context::create_context`         | `clCreateContext`         | `sycl::context`                   | Resource root, owns device information. |
| `Context::create_queue`           | `clCreateCommandQueue`    | `sycl::queue` / CUDA stream       | Queue is bound to a specific context/device. |
| `create_empty_buffer<T>`          | `clCreateBuffer`          | `sycl::buffer` / `cudaMalloc`     | Our type carries element size and state information. |
| `write_(non)_blocking`            | `clEnqueueWriteBuffer`    | `queue.submit(copy)`              | Enforces state transition: `Empty → Written`. |
| `enqueue_kernel`                  | `clEnqueueNDRangeKernel`  | `parallel_for` / CUDA kernel launch | Returns buffer in `InFlight` plus an `EventToken`. |
| `EventToken`                      | `cl_event`                | `sycl::event` / CUDA event        | Must be explicitly consumed via `wait()` or detached. |
