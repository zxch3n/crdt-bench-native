pub use crate::criterion::entry;
use std::collections::HashMap;

mod automerge;
mod criterion;

pub trait Crdt {
    type Version;
    fn create() -> Self;
    fn text_insert(&mut self, pos: usize, text: &str);
    fn text_del(&mut self, pos: usize, len: usize);
    fn get_text(&self) -> Box<str>;
    fn list_insert(&mut self, pos: usize, num: i32);
    fn list_del(&mut self, pos: usize, len: usize);
    fn get_list(&self) -> Vec<i32>;
    fn map_insert(&mut self, key: &str, num: i32);
    fn map_del(&mut self, key: &str);
    fn get_map(&self) -> HashMap<String, i32>;
    fn encode(&self, version: Option<Self::Version>) -> Vec<u8>;
    fn decode(&mut self, update: &[u8]);
    fn version(&self) -> Self::Version;
}

#[inline(always)]
pub fn merge<C: Crdt>(a: &mut C, b: &C) {
    a.decode(&b.encode(Some(a.version())))
}
