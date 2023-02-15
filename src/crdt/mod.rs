use std::collections::HashMap;

pub mod automerge;
pub mod diamond_type;
pub mod loro;
pub mod yrs;
pub trait Crdt: Sized {
    type Version;
    fn name() -> &'static str;
    fn create(gc: bool, compression: bool) -> Self;
    // return Ok if crdt supports gc, else return the default setting in Err
    fn gc(&self) -> Result<bool, bool>;
    fn compression(&self) -> Result<bool, bool>;
    fn text_insert(&mut self, pos: usize, text: &str);
    fn text_del(&mut self, pos: usize, len: usize);
    fn get_text(&mut self) -> Box<str>;
    fn list_insert(&mut self, pos: usize, num: i32);
    fn list_del(&mut self, pos: usize, len: usize);
    fn get_list(&mut self) -> Vec<i32>;
    fn map_insert(&mut self, key: &str, num: i32);
    fn map_del(&mut self, key: &str);
    fn get_map(&mut self) -> HashMap<String, i32>;
    fn encode_full(&mut self) -> Vec<u8>;
    fn decode_full(&mut self, update: &[u8]);
    /// merge other into self and self into other
    fn merge(&mut self, other: &mut Self);
    fn version(&self) -> Self::Version;
}

#[inline(always)]
pub fn merge<C: Crdt>(a: &mut C, b: &mut C) {
    a.merge(b);
}
