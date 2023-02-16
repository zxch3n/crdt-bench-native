use crdt_bench_native::run_doc_size;
use std::fs;

fn main() {
    let md = run_doc_size(false);
    fs::write("./doc_size.md", md).unwrap();
    let md = run_doc_size(true);
    fs::write("./doc_size_parallel.md", md).unwrap();
}
