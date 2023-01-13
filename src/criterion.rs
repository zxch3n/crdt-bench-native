use criterion::{black_box, Criterion};

use crate::{bench_random_list_insert, Crdt};

fn list_random_insert_10k<C: Crdt>(c: &mut Criterion) {
    c.bench_function("list_random_insert_10k", |b| {
        b.iter(|| bench_random_list_insert::<C>(black_box(10_000)))
    });
}

pub fn entry<C: Crdt>() {
    let mut criterion: Criterion<_> = Criterion::default().configure_from_args();
    list_random_insert_10k::<C>(&mut criterion);
}
