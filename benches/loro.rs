use crdt_bench_native::{entry, Crdt};
use criterion::criterion_main;
use loro_internal::{
    container::registry::ContainerWrapper,
    log_store::{EncodeConfig, EncodeMode},
    LoroCore, VersionVector,
};

struct LoroDoc {
    doc: LoroCore,
    map: loro_internal::Map,
    list: loro_internal::List,
    text: loro_internal::Text,
}

impl Crdt for LoroDoc {
    type Version = Vec<u8>;

    fn create() -> Self {
        let mut doc = LoroCore::default();
        let text = doc.get_text("text");
        let map = doc.get_map("map");
        let list = doc.get_list("list");
        LoroDoc {
            doc,
            map,
            list,
            text,
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
        self.doc
            .encode_with_cfg(EncodeConfig::new(EncodeMode::Snapshot).without_compress())
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
}

pub fn loro() {
    entry::<LoroDoc>("loro");
}

criterion_main!(loro);
