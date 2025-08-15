// Dieser Test soll NICHT kompilieren: Auf Ready gibt es kein `wait()`.
//
// trybuild erwartet, dass der Build fehlschlägt.
// Wir konstruieren absichtlich einen Ready-Buffer und rufen `wait()` darauf auf.

use hpc_core::Ready;

fn main() {
    // Wir brauchen keinen echten Buffer; es geht nur um den Typ.
    // SAFETY: absichtlich unsicher für Compile-Fail; es wird nicht ausgeführt.
    let buf: hpc_core::GpuBuffer<Ready> = unsafe { std::mem::zeroed() };

    // darf es auf `Ready` NICHT geben:
    let _bad = buf.wait();
}
