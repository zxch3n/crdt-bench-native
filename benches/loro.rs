use crdt_bench_native::{automerge_parallel, entry, LoroDoc};
use criterion::criterion_main;

pub fn loro() {
    entry::<LoroDoc>("loro");
    automerge_parallel::<LoroDoc>("loro");
}

criterion_main!(loro);
