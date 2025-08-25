// tests/compile_fail/api_inflight_map.rs
//! Compile-fail: Mapping ist aus InFlight nicht erlaubt.

#![deny(warnings)] // macht must_use etc. hart – nicht nötig, aber schadet nicht.

use hpc_core::*;

fn main() {
    // Wir brauchen keinen echten Wert; Typprüfung reicht aus, um den Fehler zu triggern.
    // Achtung: Wir erzeugen absichtlich einen "Phantom"-Wert, der nie benutzt wird.
    let q: Queue<'static> = unsafe { core::mem::zeroed() };
    let mut buf_inflight: DeviceBuffer<'static, u8, InFlight> = unsafe { core::mem::zeroed() };

    // Erwartet: **compile error** (kein Mapping aus InFlight erlaubt)
    // Falls deine API `map_write_blocking` heißt (wie in unseren Skizzen):
    let _mapped = buf_inflight
        .map_write_blocking(&q, 0..16, /*invalidate=*/ true)
        .unwrap();

    // Falls deine API stattdessen `map_for_write_block(...)` heißt,
    // kommentiere die Zeile oben aus und nutze diese hier:
    // let (_guard, _token) = buf_inflight.map_for_write_block(&q, 0..16).unwrap();
}
