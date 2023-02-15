use crdt_bench_native::{automerge_parallel, entry, DiamondTypeDoc};
use criterion::criterion_main;

pub fn bench() {
    entry::<DiamondTypeDoc>("diamond-type");
    automerge_parallel::<DiamondTypeDoc>("diamond-type");
}

criterion_main!(bench);
