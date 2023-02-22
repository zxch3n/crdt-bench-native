use std::{collections::HashMap, rc::Rc};

use crate::Crdt;
use diamond_types::{
    list::{
        encoding::{ENCODE_FULL, ENCODE_PATCH},
        remote_ids::RemoteId,
        ListCRDT,
    },
    LocalVersion,
};
use rand::Rng;

pub struct DiamondTypeDoc {
    doc: ListCRDT,
    id: Rc<String>,
    compression: bool,
    connections: HashMap<Rc<String>, LocalVersion>,
    gc: bool,
}

impl DiamondTypeDoc {
    pub fn version(&self) -> Vec<RemoteId> {
        self.doc.oplog.remote_version().into_iter().collect()
    }

    pub fn encode_for(&self, other: &Self) -> Vec<u8> {
        if let Some(other_version) = self.connections.get(&other.id) {
            self.doc.oplog.encode_from(ENCODE_PATCH, other_version)
        } else {
            self.doc.oplog.encode(ENCODE_FULL)
        }
    }

    pub fn merge_other(&mut self, other: &Self, update: &[u8]) {
        self.decode_full(update);
        self.connections.insert(
            other.id.clone(),
            self.doc
                .oplog
                .remote_to_local_version(other.version().iter()),
        );
    }
}

impl Crdt for DiamondTypeDoc {
    type Version = Vec<RemoteId>;

    fn name() -> &'static str {
        "diamond-type"
    }
    fn create(gc: bool, compression: bool, client_id: Option<u64>) -> Self {
        let mut doc = ListCRDT::new();
        let id = if client_id.is_some() {
            client_id.unwrap()
        } else {
            rand::thread_rng().gen()
        };
        let _ = doc.get_or_create_agent_id(&id.to_string());
        DiamondTypeDoc {
            doc,
            id: Rc::new(id.to_string()),
            compression,
            connections: Default::default(),
            gc,
        }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.doc.insert(0, pos, text);
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.doc.delete(0, pos..len + pos);
    }

    fn get_text(&mut self) -> Box<str> {
        self.doc.branch.content().to_string().into_boxed_str()
    }

    fn list_insert(&mut self, pos: usize, _num: i32) {
        self.doc.insert(0, pos, "0");
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.doc.delete(0, pos..pos + len);
    }

    fn get_list(&mut self) -> Vec<i32> {
        todo!()
    }

    fn map_insert(&mut self, _key: &str, _num: i32) {}

    fn map_del(&mut self, _key: &str) {}

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
        let mut opts = ENCODE_FULL;
        opts.store_deleted_content = !self.gc;
        opts.compress_content = self.compression;
        self.doc.oplog.encode(opts)
    }

    fn decode_full(&mut self, update: &[u8]) {
        self.doc.oplog.decode_and_add(update).unwrap();
        self.doc
            .branch
            .merge(&self.doc.oplog, self.doc.oplog.local_version_ref())
    }

    fn merge(&mut self, other: &mut Self) {
        self.merge_other(other, &other.encode_for(self));
        other.merge_other(self, &self.encode_for(other));
    }

    fn version(&self) -> Self::Version {
        self.doc.oplog.remote_version().into_vec()
    }

    fn gc(&self) -> Result<bool, bool> {
        Ok(self.gc)
    }

    fn compression(&self) -> Result<bool, bool> {
        Ok(self.compression)
    }
}
