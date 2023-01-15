use crdt_bench_native::{entry, Crdt};
use criterion::criterion_main;
use diamond_types::{
    list::{
        encoding::{ENCODE_FULL, ENCODE_PATCH},
        remote_ids::RemoteId,
        ListCRDT,
    },
    AgentId,
};
use rand::Rng;

struct DiamondTypeDoc {
    doc: ListCRDT,
    id: String,
}

impl Crdt for DiamondTypeDoc {
    type Version = Vec<RemoteId>;

    fn create() -> Self {
        let mut doc = ListCRDT::new();
        let id: u64 = rand::thread_rng().gen();
        let inner_id = doc.get_or_create_agent_id(&id.to_string());
        DiamondTypeDoc {
            doc,
            id: id.to_string(),
        }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.doc.insert(0, pos, text);
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.doc.delete(0, pos..len + pos);
    }

    fn get_text(&self) -> Box<str> {
        self.doc.branch.content().to_string().into_boxed_str()
    }

    fn list_insert(&mut self, pos: usize, _num: i32) {
        self.doc.insert(0, pos, "0");
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.doc.delete(0, pos..pos + len);
    }

    fn get_list(&self) -> Vec<i32> {
        todo!()
    }

    fn map_insert(&mut self, key: &str, num: i32) {}

    fn map_del(&mut self, key: &str) {}

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
        match version {
            Some(version) => self.doc.oplog.encode(ENCODE_FULL),
            None => self.doc.oplog.encode(ENCODE_FULL),
        }
    }

    fn decode(&mut self, update: &[u8]) {
        self.doc.oplog.decode_and_add(update).unwrap();
        self.doc
            .branch
            .merge(&self.doc.oplog, self.doc.oplog.local_version_ref())
    }

    fn version(&self) -> Self::Version {
        self.doc.oplog.remote_version().into_vec()
    }
}

pub fn bench() {
    entry::<DiamondTypeDoc>("diamond-type");
}

criterion_main!(bench);
