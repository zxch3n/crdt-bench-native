use indicatif::ProgressBar;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

use crate::{
    automerge::get_automerge_actions, merge, AutomergeDoc, Crdt, DiamondTypeDoc, LoroDoc, YrsDoc,
};

pub struct DocSizeReport {
    name: String,
    dataset_name: String,
    gc: Result<bool, bool>,
    compression: Result<bool, bool>,
    doc_size: Option<usize>,
}

fn gen_report<C: Crdt>(gc: bool, compression: bool) -> DocSizeReport {
    let mut crdt = C::create(gc, compression);
    let mut run = true;
    if let Err(support_gc) = crdt.gc() {
        run = support_gc == gc;
    }
    if let Err(support_compression) = crdt.compression() {
        run = support_compression == compression;
    }

    if !run {
        return DocSizeReport {
            name: C::name().to_string(),
            dataset_name: "automerge paper".to_string(),
            gc: crdt.gc(),
            compression: crdt.compression(),
            doc_size: None,
        };
    }
    let actions = get_automerge_actions();
    let bar = ProgressBar::new(actions.len() as u64);
    for action in &actions {
        bar.inc(1);
        if action.del != 0 {
            crdt.text_del(action.pos, action.del);
        }

        if !action.ins.is_empty() {
            crdt.text_insert(action.pos, &action.ins);
        }
    }
    let encoded = crdt.encode_full();
    bar.finish();
    println!(
        "{} gc {} compression {} doc_size {}",
        C::name(),
        crdt.gc().map_or("x".to_string(), |v| v.to_string()),
        crdt.compression()
            .map_or("x".to_string(), |v| v.to_string()),
        Some(encoded.len())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "x".to_string()),
    );
    DocSizeReport {
        name: C::name().to_string(),
        dataset_name: "automerge paper".to_string(),
        gc: crdt.gc(),
        compression: crdt.compression(),
        doc_size: Some(encoded.len()),
    }
}

fn gen_report_parallel<C: Crdt>(gc: bool, compression: bool) -> DocSizeReport {
    let mut crdt = C::create(gc, compression);
    let mut crdt2 = C::create(gc, compression);
    let mut run = true;
    if let Err(support_gc) = crdt.gc() {
        run = support_gc == gc;
    }
    if let Err(support_compression) = crdt.compression() {
        run = support_compression == compression;
    }

    if !run {
        return DocSizeReport {
            name: C::name().to_string(),
            dataset_name: "automerge paper".to_string(),
            gc: crdt.gc(),
            compression: crdt.compression(),
            doc_size: None,
        };
    }
    let mut rng: StdRng = SeedableRng::seed_from_u64(1);

    let mut actions = get_automerge_actions().into_iter();
    let bar = ProgressBar::new(actions.len() as u64);
    while let Some(action) = actions.next() {
        bar.inc(1);
        if action.del != 0 {
            crdt.text_del(action.pos, action.del);
        }

        if !action.ins.is_empty() {
            crdt.text_insert(action.pos, &action.ins);
        }
        merge(&mut crdt, &mut crdt2);
        let r = rng.gen_range(1..11);
        for _ in 0..r {
            if let Some(action) = actions.next() {
                bar.inc(1);
                if action.del != 0 {
                    crdt2.text_del(action.pos, action.del);
                }
                if !action.ins.is_empty() {
                    crdt2.text_insert(action.pos, &action.ins);
                }
            } else {
                break;
            }
        }
        merge(&mut crdt, &mut crdt2);
    }
    let encoded = crdt.encode_full();
    bar.finish();
    println!(
        "{} gc {} compression {} doc_size {}",
        C::name(),
        crdt.gc().map_or("x".to_string(), |v| v.to_string()),
        crdt.compression()
            .map_or("x".to_string(), |v| v.to_string()),
        Some(encoded.len())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "x".to_string()),
    );
    DocSizeReport {
        name: C::name().to_string(),
        dataset_name: "automerge paper".to_string(),
        gc: crdt.gc(),
        compression: crdt.compression(),
        doc_size: Some(encoded.len()),
    }
}

struct ReportTable(HashMap<String, Vec<DocSizeReport>>);

impl ReportTable {
    fn new() -> Self {
        Self(Default::default())
    }

    fn insert_report<C: Crdt>(&mut self, report: DocSizeReport) {
        self.0
            .entry(C::name().to_string())
            .or_insert_with(|| Vec::with_capacity(4))
            .push(report);
    }

    fn to_all_md(&self) -> String {
        let mut md = String::new();
        let loro = self.0.get(LoroDoc::name()).unwrap();
        let automerge = self.0.get(AutomergeDoc::name()).unwrap();
        let diamond_type = self.0.get(DiamondTypeDoc::name()).unwrap();
        let yrs = self.0.get(YrsDoc::name()).unwrap();
        md.push_str("|     |  loro  | automerge | diamond-type | yrs |\n");
        md.push_str("|  ----  | ----  |  ----  | ----  |  ----  |");

        for (title, index) in [("", 0), ("gc", 1), ("compress", 2), ("gc & compress", 3)] {
            md.push_str(&format!("\n|{}|", title));
            for crdt in [loro, automerge, diamond_type, yrs] {
                let size = crdt[index]
                    .doc_size
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "x".to_string());
                md.push_str(&format!(" {} |", size))
            }
        }
        md.push('\n');
        md
    }

    // fn to_crdt_md<C: Crdt>(&self) -> String {}
}

fn per_crdt<C: Crdt>(table: &mut ReportTable, parallel: bool) {
    println!("Benchmarking {}", C::name());
    // TODO: skip if crdt doesn't support gc or compression
    for compression in [false, true] {
        for gc in [false, true] {
            let report = if parallel {
                gen_report_parallel::<C>(gc, compression)
            } else {
                gen_report::<C>(gc, compression)
            };
            table.insert_report::<C>(report);
        }
    }
}

fn bench_document_size(parallel: bool) -> ReportTable {
    println!("Benchmarking doc size......");
    let mut report_table = ReportTable::new();
    // per_crdt::<LoroDoc>(&mut report_table, parallel);
    // per_crdt::<AutomergeDoc>(&mut report_table, parallel);
    per_crdt::<YrsDoc>(&mut report_table, parallel);
    per_crdt::<DiamondTypeDoc>(&mut report_table, parallel);
    report_table
}

pub fn run_doc_size(parallel: bool) -> String {
    let table = bench_document_size(parallel);
    table.to_all_md()
}
