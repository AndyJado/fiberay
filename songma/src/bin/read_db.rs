use std::{io::stdin, mem::swap};

use indradb::{Datastore, RangeVertexQuery, RocksdbDatastore};
use songma::client::AppState;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory datastore
    let p = &"./image.rdb";
    let db = RocksdbDatastore::new(p, None)?;
    client(&db);
    Ok(())
}

fn client(db: &RocksdbDatastore) {
    let mut buf = String::new();
    let mut state = AppState::Welcome;
    loop {
        println!("{state}");
        let stdin = stdin();
        let reading_in = stdin.read_line(&mut buf);
        let Some(_) = reading_in.ok() else {continue};
        //FIXME: story goes here!
        if &buf == "1\n" {
            dbg_db(db);
            state.walking();
        } else {
            state.home();
        }
        // next run
        buf.clear();
    }
}

pub fn dbg_db(db: &RocksdbDatastore) {
    dbg!(&db);
    let all_v_q = RangeVertexQuery {
        limit: 2000,
        t: None,
        start_id: None,
    };
    let all_ppts = &db
        .get_all_vertex_properties(all_v_q.into())
        .expect("get ver_ppt");
    dbg!(all_ppts);
}
