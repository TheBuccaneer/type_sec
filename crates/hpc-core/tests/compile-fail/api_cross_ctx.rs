use hpc_core::api::{Context, DeviceBuffer, Queue, Kernel};

fn main() {
    // Zwei getrennte Kontexte + Queues
    let ctx1 = Context::new();
    let q1 = Queue::new(&ctx1);
    let ctx2 = Context::new();
    let q2 = Queue::new(&ctx2);
    
    // Buffer in Kontext 1 erzeugen
    let buf: DeviceBuffer<_, _, u32, _> = q1.create_buffer_elems(4);
    let buf = q1.enqueue_write(buf, &[1, 2, 3, 4]);
    
    let k = Kernel::new(&q2, "dummy");
    
    // compile-fail: versuche Buffer (ctx1) mit Kernel aus ctx2 zu nutzen
    let _ = q2.enqueue_kernel(&k, buf);
}