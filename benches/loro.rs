use crdt_bench_native::{automerge_parallel, entry, Loro};
use criterion::criterion_main;

pub fn loro() {
    entry::<Loro>("loro");
    automerge_parallel::<Loro>("loro");
}

criterion_main!(loro);
