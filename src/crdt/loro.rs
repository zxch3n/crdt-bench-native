use crate::Crdt;
use loro_internal::{log_store::EncodeConfig, LoroCore};

pub struct LoroDoc {
    doc: LoroCore,
    map: loro_internal::Map,
    list: loro_internal::List,
    text: loro_internal::Text,
    compression: bool,
    // TODO: remove
    gc: bool,
}

impl Crdt for LoroDoc {
    type Version = Vec<u8>;
    fn name() -> &'static str {
        "loro"
    }
    fn create(gc: bool, compression: bool) -> Self {
        let mut doc = LoroCore::default();
        doc.gc(gc);
        let text = doc.get_text("text");
        let map = doc.get_map("map");
        let list = doc.get_list("list");
        LoroDoc {
            doc,
            map,
            list,
            text,
            compression,
            gc,
        }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.text.insert(&self.doc, pos, text).unwrap();
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.text.delete(&self.doc, pos, len).unwrap();
    }

    fn get_text(&mut self) -> Box<str> {
        self.text.get_value().into_string().unwrap()
    }

    fn list_insert(&mut self, pos: usize, num: i32) {
        self.list.insert(&self.doc, pos, num).unwrap();
    }

    fn get_list(&mut self) -> Vec<i32> {
        self.list
            .get_value()
            .into_list()
            .unwrap()
            .into_iter()
            .map(|v| v.into_i32().unwrap())
            .collect()
    }

    fn map_insert(&mut self, key: &str, num: i32) {
        self.map.insert(&self.doc, key, num).unwrap();
    }

    fn get_map(&mut self) -> std::collections::HashMap<String, i32> {
        self.map
            .get_value()
            .into_map()
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k, v.into_i32().unwrap()))
            .collect()
    }

    fn encode_full(&mut self) -> Vec<u8> {
        let mut cfg = EncodeConfig::snapshot();
        if !self.compression {
            cfg = cfg.without_compress();
        }
        self.doc.encode_with_cfg(cfg)
    }

    fn decode_full(&mut self, update: &[u8]) {
        self.doc.decode(update).unwrap()
    }

    fn merge(&mut self, other: &mut Self) {
        let a_to_b = self.doc.encode_from(other.doc.vv_cloned());
        let b_to_a = other.doc.encode_from(self.doc.vv_cloned());
        self.doc.decode(&b_to_a).unwrap();
        other.doc.decode(&a_to_b).unwrap();
    }

    fn version(&self) -> Self::Version {
        self.doc.vv_cloned().encode()
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.list.delete(&self.doc, pos, len).unwrap();
    }

    fn map_del(&mut self, key: &str) {
        self.map.delete(&self.doc, key).unwrap();
    }

    fn gc(&self) -> Result<bool, bool> {
        Ok(self.gc)
    }

    fn compression(&self) -> Result<bool, bool> {
        Ok(self.compression)
    }
}
