pub use crate::criterion::entry;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

mod criterion;

pub trait Crdt {
    type Version;
    fn create() -> Self;
    fn insert_text(&mut self, pos: usize, text: &str);
    fn get_text(&self) -> Box<str>;
    fn insert_list(&mut self, pos: usize, num: i32);
    fn get_list(&self) -> Vec<i32>;
    fn insert_map(&mut self, key: &str, num: i32);
    fn get_map(&self) -> HashMap<String, i32>;
    fn encode(&self, version: Self::Version) -> Vec<u8>;
    fn decode(&mut self, update: Vec<u8>);
    fn version(&self) -> Self::Version;
}

#[inline(always)]
pub fn merge<C: Crdt>(a: &mut C, b: &C) {
    a.decode(b.encode(a.version()))
}

pub fn bench_random_list_insert<C: Crdt>(n: usize) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let mut crdt = C::create();
    for i in 0..n {
        crdt.insert_list(rng.gen::<usize>() % (i + 1), i as i32);
    }
}

pub fn bench_automerge_paper() {}
