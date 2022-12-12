use indradb::{Datastore, RocksdbDatastore};
use songma::{
    doc_reader::{pan_read, DocMap},
    test_blob::TestSuite,
};
use std::env::args;

// Create a new database at path, read doc to it.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = &"./image.rdb";
    // Create an in-memory datastore
    let db = RocksdbDatastore::new(db_path, None)?;
    let args: Vec<String> = args().collect();
    let ast = pan_read(&args[1]);
    let doc_map = DocMap::new(ast);
    let bulks = doc_map.a_report_bulk();
    db.bulk_insert(bulks)?;
    // default create vertex bulk
    db.sync()?;
    Ok(())
}
