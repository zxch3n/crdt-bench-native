use crdt_bench_native::{entry, Crdt};
use criterion::criterion_main;
use diamond_types::list::{
    encoding::{ENCODE_FULL, ENCODE_PATCH},
    remote_ids::RemoteId,
    ListCRDT,
};
use rand::Rng;

struct DiamondTypeDoc {
    doc: ListCRDT,
    id: u32,
}

#[test]
pub fn test() {
    let mut doc = ListCRDT::new();
    let id = doc.get_or_create_agent_id("a");
    doc.insert(id, 0, "123");
    let mut doc_b = ListCRDT::new();
    let id = doc_b.get_or_create_agent_id("b");
    doc_b.insert(id, 0, "ccc");
}
