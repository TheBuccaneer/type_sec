// tests/compile_fail/api_forget_unmap.rs
//! Compile-fail: MapToken/Guard wird erzeugt, aber nicht benutzt/unmapped.

#![deny(unused_must_use)]

/*
no use of the unmap token. You have to unmap first, befor the written-state
*/

use hpc_core::*;

fn main() {
    let ctx = Context::create_context().unwrap();
    let queue = ctx.create_queue().unwrap();

    // Leeren Buffer erzeugen
    let buf = ctx.create_empty_buffer::<u8>(16).unwrap();


    buf.map_for_write_block(&queue).unwrap();

}
