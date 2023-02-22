use std::time::Duration;

use criterion::{black_box, measurement::WallTime, BenchmarkGroup, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    automerge::{self, get_automerge_actions},
    merge, Crdt,
};

const GC: bool = false;
const COMPRESSION: bool = false;

pub fn bench_random_list_insert<C: Crdt>(n: usize) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let mut crdt = C::create(GC, COMPRESSION, None);
    let mut crdt_new = C::create(GC, COMPRESSION, None);
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
                docs.push(C::create(GC, COMPRESSION, None));
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
            let mut crdt = C::create(GC, COMPRESSION, None);
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
    let mut crdt = C::create(GC, COMPRESSION, None);
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
            let mut new_crdt = C::create(GC, COMPRESSION, None);
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

pub fn automerge_parallel<C: Crdt>(name: &str) {
    let mut criterion: Criterion<_> = Criterion::default()
        .configure_from_args()
        .measurement_time(Duration::from_secs(1))
        .sample_size(10);
    let mut b = criterion.benchmark_group(name);
    b.bench_function("parallel applying automerge edits", |b| {
        b.iter(|| {
            let mut crdt = C::create(false, false, None);
            let mut crdt2 = C::create(false, false, None);
            let mut rng: StdRng = SeedableRng::seed_from_u64(1);
            let mut actions = get_automerge_actions().into_iter();
            let mut len = 0;
            while let Some(mut action) = actions.next() {
                let mut a_len: isize = 0;
                let mut b_len: isize = 0;
                action.pos %= len as usize + 1;
                action.del = action.del.min((len - a_len).max(0) as usize);
                if action.del != 0 {
                    a_len -= action.del as isize;
                    crdt.text_del(action.pos, action.del);
                }

                if !action.ins.is_empty() {
                    a_len += action.ins.len() as isize;
                    crdt.text_insert(action.pos, &action.ins);
                }
                let r = rng.gen_range(1..11);
                for _ in 0..r {
                    if let Some(mut action) = actions.next() {
                        action.pos %= len as usize + 1;
                        action.del = action.del.min((len - a_len).max(0) as usize);
                        if action.del != 0 {
                            b_len -= action.del as isize;
                            crdt2.text_del(action.pos, action.del);
                        }
                        if !action.ins.is_empty() {
                            b_len += action.del as isize;
                            crdt2.text_insert(action.pos, &action.ins);
                        }
                    } else {
                        break;
                    }
                }
                merge(&mut crdt, &mut crdt2);
                len = a_len + b_len;
                len = len.max(0);
            }
        })
    });
}
