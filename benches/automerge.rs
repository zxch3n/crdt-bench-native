use crdt_bench_native::{entry, AutomergeDoc};
use criterion::criterion_main;

pub fn bench() {
    entry::<AutomergeDoc>("automerge");
}

criterion_main!(bench);
