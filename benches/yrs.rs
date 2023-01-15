use crdt_bench_native::{entry, Crdt};
use criterion::criterion_main;
use yrs::{
    updates::decoder::Decode, Array, ArrayRef, Doc, GetString, Map, MapRef, ReadTxn, StateVector,
    Text, TextRef, Transact, Update,
};

struct YrsDoc {
    doc: Doc,
    map: MapRef,
    list: ArrayRef,
    text: TextRef,
}

impl Crdt for YrsDoc {
    type Version = StateVector;

    fn create() -> Self {
        let doc = Doc::new();
        YrsDoc {
            map: doc.get_or_insert_map("map"),
            list: doc.get_or_insert_array("list"),
            text: doc.get_or_insert_text("text"),
            doc,
        }
    }

    fn text_insert(&mut self, pos: usize, text: &str) {
        self.text
            .insert(&mut self.doc.transact_mut(), pos as u32, text);
    }

    fn text_del(&mut self, pos: usize, len: usize) {
        self.text
            .remove_range(&mut self.doc.transact_mut(), pos as u32, len as u32)
    }

    fn get_text(&self) -> Box<str> {
        self.text.get_string(&self.doc.transact()).into_boxed_str()
    }

    fn list_insert(&mut self, pos: usize, num: i32) {
        self.list
            .insert(&mut self.doc.transact_mut(), pos as u32, num);
    }

    fn list_del(&mut self, pos: usize, len: usize) {
        self.list
            .remove_range(&mut self.doc.transact_mut(), pos as u32, len as u32);
    }

    fn get_list(&self) -> Vec<i32> {
        todo!()
    }

    fn map_insert(&mut self, key: &str, num: i32) {
        self.map.insert(&mut self.doc.transact_mut(), key, num);
    }

    fn map_del(&mut self, key: &str) {
        self.map.remove(&mut self.doc.transact_mut(), key);
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
        match version {
            Some(version) => self.doc.transact_mut().encode_diff_v1(&version),
            None => self.doc.transact_mut().encode_update_v2(),
        }
    }

    fn decode(&mut self, update: &[u8]) {
        self.doc
            .transact_mut()
            .apply_update(Update::decode_v1(update).unwrap())
    }

    fn version(&self) -> Self::Version {
        self.doc.transact_mut().before_state().clone()
    }
}

pub fn yrs() {
    entry::<YrsDoc>("yrs");
}

criterion_main!(yrs);
