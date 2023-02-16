use std::io::{Read, Write};

use yrs::{
    updates::decoder::Decode, Array, ArrayRef, Doc, GetString, Map, MapRef, Options, ReadTxn,
    StateVector, Text, TextRef, Transact, Update,
};

use crate::Crdt;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};

pub struct YrsDoc {
    doc: Doc,
    map: MapRef,
    list: ArrayRef,
    text: TextRef,
    compression: bool,
}

impl Crdt for YrsDoc {
    type Version = StateVector;

    fn name() -> &'static str {
        "yrs"
    }

    fn create(gc: bool, compression: bool) -> Self {
        let options = Options {
            skip_gc: !gc,
            ..Default::default()
        };
        let doc = Doc::with_options(options);
        YrsDoc {
            map: doc.get_or_insert_map("map"),
            list: doc.get_or_insert_array("list"),
            text: doc.get_or_insert_text("text"),
            doc,
            compression,
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

    fn get_text(&mut self) -> Box<str> {
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

    fn get_list(&mut self) -> Vec<i32> {
        todo!()
    }

    fn map_insert(&mut self, key: &str, num: i32) {
        self.map.insert(&mut self.doc.transact_mut(), key, num);
    }

    fn map_del(&mut self, key: &str) {
        self.map.remove(&mut self.doc.transact_mut(), key);
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
        let encoded = self
            .doc
            .transact_mut()
            .encode_state_as_update_v2(&Default::default());

        if self.compression {
            let mut ans = vec![];
            {
                let mut c = DeflateEncoder::new(&mut ans, Compression::default());
                c.write_all(&encoded).unwrap();
                c.try_finish().unwrap();
            }
            ans
        } else {
            encoded
        }
    }

    fn decode_full(&mut self, update: &[u8]) {
        let mut ans = vec![];
        let update = if self.compression {
            let mut c = DeflateDecoder::new(update);
            c.read_to_end(&mut ans).unwrap();
            &ans
        } else {
            update
        };
        self.doc
            .transact_mut()
            .apply_update(Update::decode_v2(update).unwrap())
    }

    fn version(&self) -> Self::Version {
        self.doc.transact_mut().before_state().clone()
    }

    fn merge(&mut self, other: &mut Self) {
        let a = self.doc.transact_mut().before_state().clone();
        let b = other.doc.transact_mut().before_state().clone();
        let a_to_b = self.doc.transact_mut().encode_state_as_update_v2(&b);
        let b_to_a = other.doc.transact_mut().encode_state_as_update_v2(&a);
        self.doc
            .transact_mut()
            .apply_update(Update::decode_v2(&b_to_a).unwrap());
        other
            .doc
            .transact_mut()
            .apply_update(Update::decode_v2(&a_to_b).unwrap());
    }

    fn gc(&self) -> Result<bool, bool> {
        Ok(!self.doc.options().skip_gc)
    }

    fn compression(&self) -> Result<bool, bool> {
        Ok(self.compression)
    }
}
