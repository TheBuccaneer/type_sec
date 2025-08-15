use std::marker::PhantomData;

#[cfg(feature = "metrics")]
use hpc_core::metrics::{log_run, RunLog};

// --- Zustandsmarker ---
struct Queued;
struct InFlight;
struct Ready;

// --- Einfacher Buffer (CPU) ---
struct Buffer {
    data: Vec<i32>,
}

// --- Typisierte Hülle über Buffer ---
struct Buf<S> {
    inner: Buffer,
    _s: PhantomData<S>,
}

impl Buf<Queued> {
    fn new(data: Vec<i32>) -> Self {
        log_metric("pipeline.queue", data.len());
        Self { inner: Buffer { data }, _s: PhantomData }
    }

    fn launch(self) -> Buf<InFlight> {
        log_metric("pipeline.launch", self.inner.data.len());
        Buf { inner: self.inner, _s: PhantomData }
    }
}

impl Buf<InFlight> {
    fn wait(self) -> Buf<Ready> {
        // (simuliere Arbeit)
        let _ = self.inner.data.iter().map(|x| x + 1).sum::<i32>();
        log_metric("pipeline.wait", self.inner.data.len());
        Buf { inner: self.inner, _s: PhantomData }
    }
}

impl Buf<Ready> {
    fn sum(&self) -> i32 {
        self.inner.data.iter().sum()
    }
}

// --- kleiner Helper: nur loggen wenn Feature aktiv ---
fn log_metric(example: &'static str, n: usize) {
    #[cfg(feature = "metrics")]
    {
        let _ = log_run(&RunLog { example, n });
    }
}

fn main() {
    // legaler Pfad: Queued -> InFlight -> Ready
    let buf_q = Buf::<Queued>::new(vec![1, 2, 3, 4]);
    let buf_f = buf_q.launch();
    let buf_r = buf_f.wait();
    let total = buf_r.sum();
    println!("sum = {}", total);

    // !!! Auskommentiert lassen: illegaler Pfad, soll später per Typchecker scheitern
    // let bad = Buf::<Ready> { inner: Buffer { data: vec![1] }, _s: PhantomData }; // verbieten wir später
    // let _ = bad.wait(); // sollte nicht kompilieren, wenn Ready kein wait() hat
}
