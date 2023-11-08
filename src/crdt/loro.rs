use std::sync::Arc;

use crate::Crdt;
use loro_internal::LoroDoc;

pub struct Loro {
    doc: LoroDoc,
    map: loro_internal::MapHandler,
    list: loro_internal::ListHandler,
    text: loro_internal::TextHandler,
    compression: bool,
    // TODO: remove
    gc: bool,
}

impl Crdt for Loro {
    type Version = Vec<u8>;
    fn name() -> &'static str {
        "loro"
    }

    fn create(gc: bool, compression: bool) -> Self {
        let doc = LoroDoc::new_auto_commit();
        let text = doc.get_text("text");
        let map = doc.get_map("map");
        let list = doc.get_list("list");
        Loro {
            doc,
            map,
            list,
            text,
            compression,
            gc,
        }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.text.insert_(pos, text).unwrap();
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.text.delete_(pos, len).unwrap();
    }

    fn get_text(&mut self) -> Box<str> {
        let s = {
            let this = self.text.get_value().into_string().unwrap();
            Arc::try_unwrap(this).unwrap_or_else(|arc| (*arc).clone())
        };
        s.into_boxed_str()
    }

    fn list_insert(&mut self, pos: usize, num: i32) {
        self.list.insert_(pos, num.into()).unwrap();
    }

    fn get_list(&mut self) -> Vec<i32> {
        let vec = self.list.get_value().into_list().unwrap();
        vec.iter().map(|v| *v.as_i32().unwrap()).collect()
    }

    fn map_insert(&mut self, key: &str, num: i32) {
        self.map.insert_(key, num.into()).unwrap();
    }

    fn get_map(&mut self) -> std::collections::HashMap<String, i32> {
        let hash_map = {
            let this = self.map.get_value().into_map().unwrap();
            Arc::try_unwrap(this).unwrap_or_else(|arc| (*arc).clone())
        };
        hash_map
            .into_iter()
            .map(|(k, v)| (k, v.into_i32().unwrap()))
            .collect()
    }

    fn encode_full(&mut self) -> Vec<u8> {
        self.doc.export_snapshot()
    }

    fn decode_full(&mut self, update: &[u8]) {
        self.doc.import(update).unwrap()
    }

    fn merge(&mut self, other: &mut Self) {
        let a_to_b = self.doc.export_from(&other.doc.oplog_vv());
        let b_to_a = other.doc.export_from(&self.doc.oplog_vv());
        self.doc.import(&b_to_a).unwrap();
        other.doc.import(&a_to_b).unwrap();
    }

    fn version(&self) -> Self::Version {
        self.doc.oplog_vv().encode()
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.list.delete_(pos, len).unwrap();
    }

    fn map_del(&mut self, key: &str) {
        self.map.delete_(key).unwrap();
    }

    fn gc(&self) -> Result<bool, bool> {
        Ok(self.gc)
    }

    fn compression(&self) -> Result<bool, bool> {
        Ok(self.compression)
    }
}
