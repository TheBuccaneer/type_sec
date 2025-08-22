# Cross-Mapping of API Concepts

Diese Tabelle zeigt, wie zentrale Konstrukte unserer API (`hpc-core`) zu bekannten
Konzepten in OpenCL, CUDA und SYCL in Beziehung stehen. Sie dient nur der Orientierung
für Leser:innen, die aus der GPU-Welt kommen. Wir implementieren *keine* dieser
APIs, sondern zeigen, was inhaltlich vergleichbar ist und wo unsere Typ-States
einen Unterschied machen.

| Our API (`hpc-core`)              | OpenCL (C-API)            | CUDA / SYCL (roughly)          | Notes |
|-----------------------------------|---------------------------|--------------------------------|-------|
| `Context::create_context`         | `clCreateContext`         | `sycl::context`                | Resource root, hält Geräteinfo |
| `Context::create_queue`           | `clCreateCommandQueue`    | `sycl::queue` / CUDA stream    | Queue ist an Context/Device gebunden |
| `create_buffer::<T>`              | `clCreateBuffer`          | `sycl::buffer` / `cudaMalloc`  | Unser Typ trägt zusätzlich Size + State |
| `enqueue_write`                   | `clEnqueueWriteBuffer`    | `queue.submit(copy)`           | Erzwingt Transition: Empty → Ready |
| `enqueue_kernel`                  | `clEnqueueNDRangeKernel`  | `parallel_for` / CUDA kernel launch | Rückgabe: InFlight + EventToken |
| `EventToken`                      | `cl_event`                | `sycl::event` / CUDA event     | Muss explizit `wait()` oder `detach()` werden |
| Type-State (`Empty/Ready/InFlight`)| –                        | –                              | Compile-time Enforcements, kein Gegenstück in CL/SYCL/CUDA |
