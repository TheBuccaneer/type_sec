// hpc-core/examples/vec_add.rs

/// Liest --n von der CLI (Standard 1024)
fn parse_n() -> usize {
    let mut args = std::env::args();
    let mut n: usize = 1024;
    while let Some(arg) = args.next() {
        if arg == "--n" {
            if let Some(v) = args.next() {
                n = v.parse().unwrap_or(n);
            }
        }
    }
    n
}

fn main() {
    let n = parse_n();

    // Später: Context/Queue/Kernel etc. aufrufen
    // Hier nur Log-Ausgabe für erste Integration
    println!("{{\"event\":\"run\",\"example\":\"vec_add\",\"n\":{}}}", n);

    #[cfg(feature = "metrics")]
{
    let run = hpc_core::metrics::RunLog { example: "vec_add", n };
    hpc_core::metrics::log_run(&run);
}
}
