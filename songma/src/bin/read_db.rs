use std::fs;
use std::fs::File;
use std::io::BufReader;

use indradb::Datastore;
use indradb::MemoryDatastore;
use indradb::RangeVertexQuery;
use indradb::RocksdbDatastore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory datastore
    let p = &"./image.rdb";
    let db = RocksdbDatastore::new(p, None)?;
    dbg!(&db);
    let all_v_q = RangeVertexQuery {
        limit: 200,
        t: None,
        start_id: None,
    };
    let all_ppts = &db.get_all_vertex_properties(all_v_q.into())?;
    dbg!(all_ppts);
    Ok(())
}
