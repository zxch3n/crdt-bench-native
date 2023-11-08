pub use crate::criterion::{automerge_parallel, entry};

mod automerge;
mod crdt;
mod criterion;
mod doc_size;

pub use crdt::{
    automerge::AutomergeDoc, diamond_type::DiamondTypeDoc, loro::Loro, merge, yrs::YrsDoc, Crdt,
};

pub use doc_size::run_doc_size;
