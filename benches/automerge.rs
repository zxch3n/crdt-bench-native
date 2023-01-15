use std::sync::Arc;

use automerge::{transaction::Transactable, ActorId, AutoCommit, Automerge, ObjId, ObjType, ROOT};
use crdt_bench_native::{entry, Crdt};
use criterion::criterion_main;

struct AutomergeDoc {
    doc: AutoCommit,
    text: ObjId,
    list: ObjId,
}

impl Crdt for AutomergeDoc {
    type Version = ();

    fn create() -> Self {
        let mut d = automerge::AutoCommit::new();
        d.set_actor(automerge::ActorId::random());
        let text = d.put_object(ROOT, "text", ObjType::Text).unwrap();
        let list = d.put_object(ROOT, "list", ObjType::List).unwrap();
        AutomergeDoc { doc: d, text, list }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.doc.insert(&self.text, pos, text).unwrap();
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.doc.splice_text(&self.text, pos, len, "").unwrap();
    }

    fn get_text(&self) -> Box<str> {
        self.doc.text(&self.text).unwrap().into_boxed_str()
    }

    fn list_insert(&mut self, pos: usize, num: i32) {
        self.doc.insert(&self.list, pos, num).unwrap();
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.doc.splice(&self.list, pos, len, []).unwrap();
    }

    fn get_list(&self) -> Vec<i32> {
        todo!()
    }

    fn map_insert(&mut self, key: &str, num: i32) {
        todo!()
    }

    fn map_del(&mut self, key: &str) {
        todo!()
    }

    fn get_map(&self) -> std::collections::HashMap<String, i32> {
        todo!()
        // let t = self.doc.transact();
        // self.map
        //     .iter(&t)
        //     .map(|(key, value)| {
        //         let v: i64 = value.to_json(&t).into();
        //         (key.to_owned(), v as i32)
        //     })
        //     .collect()
    }

    fn encode(&mut self, version: Option<Self::Version>) -> Vec<u8> {
        self.doc.save()
    }

    fn decode(&mut self, update: &[u8]) {
        self.doc.load_incremental(&update).unwrap();
    }

    fn version(&self) -> Self::Version {
        ()
    }
}

pub fn bench() {
    entry::<AutomergeDoc>("automerge");
}

criterion_main!(bench);
