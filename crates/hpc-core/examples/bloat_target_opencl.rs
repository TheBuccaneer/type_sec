// crates/hpc-core/examples/bloat_probe_baseline.rs
// Ziel: Raw OpenCL (oder das, was eure niedrigste Schicht ist) referenzieren,
// damit ein „faire“ Baseline entsteht. Je nach eurer Lib ggf. opencl3 direkt.

#[allow(unused_imports)]
use opencl3::{
    device::get_all_devices, // irgendein Symbol als Platzhalter
    platform::get_platforms,
};

fn main() {
    // Einfache Symbolnutzung, damit LTO nicht alles droppt:
    let _ = get_platforms();
    // Optional:
    // let _ = get_all_devices(opencl3::device::CL_DEVICE_TYPE_ALL);
}
