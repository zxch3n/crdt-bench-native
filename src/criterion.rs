use std::time::Duration;

use criterion::{black_box, measurement::WallTime, BenchmarkGroup, Criterion};
use rand::{Rng, SeedableRng};

use crate::{automerge, merge, Crdt};

pub fn bench_random_list_insert<C: Crdt>(n: usize) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let mut crdt = C::create();
    let mut crdt_new = C::create();
    for i in 0..n {
        crdt.list_insert(rng.gen::<usize>() % (i + 1), i as i32);
        merge(&mut crdt_new, &mut crdt);
    }
}

fn list_random_insert_1k<C: Crdt>(c: &mut BenchmarkGroup<WallTime>) {
    c.bench_function("list_random_insert_1k", |b| {
        b.iter(|| bench_random_list_insert::<C>(black_box(1_000)))
    });
}

fn concurrent_list_inserts<C: Crdt>(b: &mut BenchmarkGroup<WallTime>) {
    b.bench_function("concurrent list inserts", |b| {
        b.iter(|| {
            let mut docs = vec![];
            for _ in 0..100 {
                docs.push(C::create());
            }

            for doc in docs.iter_mut() {
                doc.text_insert(0, "123");
            }

            for i in 1..100 {
                let (a, b) = arref::array_mut_ref!(&mut docs, [0, i]);
                merge(a, b);
            }

            for i in 1..100 {
                let (a, b) = arref::array_mut_ref!(&mut docs, [0, i]);
                merge(b, a);
            }
        });
    });
}

fn apply_automerge_paper<C: Crdt>(b: &mut BenchmarkGroup<WallTime>) {
    let actions = automerge::get_automerge_actions();
    b.bench_function("automerge - apply", |b| {
        b.iter(|| {
            let mut crdt = C::create();
            for action in &actions {
                if action.del != 0 {
                    crdt.text_del(action.pos, action.del);
                }

                if !action.ins.is_empty() {
                    crdt.text_insert(action.pos, &action.ins);
                }
            }
        })
    });
    let mut crdt = C::create();
    for action in &actions {
        if action.del != 0 {
            crdt.text_del(action.pos, action.del);
        }

        if !action.ins.is_empty() {
            crdt.text_insert(action.pos, &action.ins);
        }
    }
    b.bench_function("automerge - encode time", |b| {
        b.iter(|| {
            black_box(crdt.encode_full());
        })
    });
    let encoded = crdt.encode_full();
    b.bench_function("automerge - decode time", |b| {
        b.iter(|| {
            let mut new_crdt = C::create();
            new_crdt.decode_full(black_box(&encoded));
        })
    });
}

pub fn entry<C: Crdt>(name: &str) {
    let mut criterion: Criterion<_> = Criterion::default()
        .configure_from_args()
        .measurement_time(Duration::from_secs(1))
        .sample_size(10);
    let mut b = criterion.benchmark_group(name);
    list_random_insert_1k::<C>(&mut b);
    concurrent_list_inserts::<C>(&mut b);
    apply_automerge_paper::<C>(&mut b);
}
