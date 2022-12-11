use indradb::Datastore;
use songma::{
    doc_reader::{pan_read, DocMap},
    test_blob::TestSuite,
};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let ast = pan_read(&args[1]);
    let p = &"./image.rdb";
    let db = indradb::RocksdbDatastore::new(p, None).expect("should connect db");
    let doc_map = DocMap::new(ast);
    doc_map.a_report_bulk(&db);
    println!("======================");
}
