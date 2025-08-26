use hpc_core::*;

fn main() {
    // Force inclusion of API symbols so cargo-bloat can measure them.
    let _ctx = Context::create_context().unwrap();
    println!("treatment build ready");

}