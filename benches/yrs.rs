use crdt_bench_native::{entry, YrsDoc};
use criterion::criterion_main;

pub fn yrs() {
    entry::<YrsDoc>("yrs");
}

criterion_main!(yrs);
