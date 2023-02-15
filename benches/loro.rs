use crdt_bench_native::{entry, LoroDoc};
use criterion::criterion_main;

pub fn loro() {
    entry::<LoroDoc>("loro");
}

criterion_main!(loro);
