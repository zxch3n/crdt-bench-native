use automerge::{
    sync::{State, SyncDoc},
    transaction::Transactable,
    AutoCommit, ObjId, ObjType, ROOT,
};
use crdt_bench_native::{entry, Crdt};
use criterion::criterion_main;

struct AutomergeDoc {
    doc: AutoCommit,
    text: ObjId,
    list: ObjId,
    map: ObjId,
}

impl Crdt for AutomergeDoc {
    type Version = ();

    fn create() -> Self {
        let mut d = automerge::AutoCommit::new();
        d.set_actor(automerge::ActorId::random());
        let text = d.put_object(ROOT, "text", ObjType::Text).unwrap();
        let list = d.put_object(ROOT, "list", ObjType::List).unwrap();
        let map = d.put_object(ROOT, "map", ObjType::Map).unwrap();
        AutomergeDoc {
            doc: d,
            text,
            list,
            map,
        }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.doc.insert(&self.text, pos, text).unwrap();
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.doc.splice_text(&self.text, pos, len, "").unwrap();
    }

    fn get_text(&mut self) -> Box<str> {
        // self.doc.(&self.text).unwrap().into_boxed_str()
        "".to_string().into_boxed_str()
    }

    fn list_insert(&mut self, pos: usize, num: i32) {
        self.doc.insert(&self.list, pos, num).unwrap();
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.doc.splice(&self.list, pos, len, []).unwrap();
    }

    fn get_list(&mut self) -> Vec<i32> {
        todo!()
    }

    fn map_insert(&mut self, key: &str, num: i32) {
        self.doc.put(&self.map, key, num).unwrap();
    }

    fn map_del(&mut self, key: &str) {
        self.doc.delete(&self.map, key).unwrap();
    }

    fn get_map(&mut self) -> std::collections::HashMap<String, i32> {
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

    fn encode_full(&mut self) -> Vec<u8> {
        self.doc.save_nocompress()
    }

    fn decode_full(&mut self, update: &[u8]) {
        self.doc.load_incremental(update).unwrap();
    }

    fn merge(&mut self, other: &mut Self) {
        let mut state_a = State::new();
        let mut state_b = State::new();
        // sync version
        let to_b = self.doc.sync().generate_sync_message(&mut state_a).unwrap();
        let to_a = other
            .doc
            .sync()
            .generate_sync_message(&mut state_b)
            .unwrap();
        other
            .doc
            .sync()
            .receive_sync_message(&mut state_b, to_b)
            .unwrap();
        self.doc
            .sync()
            .receive_sync_message(&mut state_a, to_a)
            .unwrap();

        // sync state
        let Some(to_b) = self.doc.sync().generate_sync_message(&mut state_a) else { return };
        let to_a = other
            .doc
            .sync()
            .generate_sync_message(&mut state_b)
            .unwrap();
        other
            .doc
            .sync()
            .receive_sync_message(&mut state_b, to_b)
            .unwrap();
        self.doc
            .sync()
            .receive_sync_message(&mut state_a, to_a)
            .unwrap();
    }

    fn version(&self) -> Self::Version {}
}

pub fn bench() {
    entry::<AutomergeDoc>("automerge");
}

criterion_main!(bench);
