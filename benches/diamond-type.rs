use crdt_bench_native::{entry, DiamondTypeDoc};
use criterion::criterion_main;

pub fn bench() {
    entry::<DiamondTypeDoc>("diamond-type");
}

criterion_main!(bench);
