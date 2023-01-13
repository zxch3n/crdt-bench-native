use crdt_bench_native::{entry, Crdt};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use loro_core::{
    container::registry::ContainerWrapper, log_store::EncodeConfig, LoroCore, VersionVector,
};

struct LoroDoc {
    doc: LoroCore,
    map: loro_core::Map,
    list: loro_core::List,
    text: loro_core::Text,
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

    fn insert_text(&mut self, pos: usize, text: &str) {
        self.text.insert(&self.doc, pos, text).unwrap();
    }

    fn get_text(&self) -> Box<str> {
        self.text.get_value().into_string().unwrap()
    }

    fn insert_list(&mut self, pos: usize, num: i32) {
        self.list.insert(&self.doc, pos, num).unwrap();
    }

    fn get_list(&self) -> Vec<i32> {
        self.list
            .get_value()
            .into_list()
            .unwrap()
            .into_iter()
            .map(|v| v.into_i32().unwrap())
            .collect()
    }

    fn insert_map(&mut self, key: &str, num: i32) {
        self.map.insert(&self.doc, key, num).unwrap();
    }

    fn get_map(&self) -> std::collections::HashMap<String, i32> {
        self.map
            .get_value()
            .into_map()
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k, v.into_i32().unwrap()))
            .collect()
    }

    fn encode(&self, version: Self::Version) -> Vec<u8> {
        let vv = VersionVector::decode(&version).unwrap();
        self.doc.encode(EncodeConfig::from_vv(Some(vv))).unwrap()
    }

    fn decode(&mut self, update: Vec<u8>) {
        self.doc.decode(&update).unwrap()
    }

    fn version(&self) -> Self::Version {
        self.doc.vv_cloned().encode()
    }
}

pub fn loro() {
    entry::<LoroDoc>();
}

criterion_main!(loro);
