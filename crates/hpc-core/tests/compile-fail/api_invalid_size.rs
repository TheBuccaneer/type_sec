use hpc_core::api::{Context, DeviceBuffer, Queue};

fn main() {
    let ctx = Context::new();
    let q = Queue::new(&ctx);

    // compile-fail: Wir tun so, als würden wir eine Größe nehmen, 
    // die nicht zum Elementtyp passt.
    // Erwartet wird DeviceBuffer<'id,'ctx,u32,_>, aber Signatur erzwingt Konsistenz.
    let buf: DeviceBuffer<u32, _> = q.create_buffer_elems(3); // 3*4 = 12 Bytes
    // Angenommen, intern erzwingt API ein Vielfaches von sizeof(T).
    // => Compile-Fail, wenn wir an read/write o. Ä. gehen.
    let mut out = [0u32; 3];
    q.read_blocking(&buf, &mut out); // sollte scheitern
}
