// crates/hpc-core/examples/bloat_probe_baseline.rs
// Purpose: Reference raw OpenCL (or the lowest-level layer of our stack)
// to create a "fair" baseline for code size / bloat comparison.
// Depending on the library setup this may directly use `opencl3`.

#[allow(unused_imports)]
use opencl3::{device::get_all_devices, platform::get_platforms};

//needs a binary
fn main() {
    let _ = get_platforms();
}
