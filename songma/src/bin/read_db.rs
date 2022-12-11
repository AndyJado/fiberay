use std::io::stdin;

use indradb::*;

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
    println!("{state}");
    loop {
        let stdin = stdin();
        let reading_in = stdin.read_line(&mut buf);
        let Some(_) = reading_in.ok() else {continue};
        println!("{buf}");
        buf.clear();
    }
}

enum AppState {
    Welcome,
    Ask,
    Tell,
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AppState::Welcome => "👋\n按1问我我知道的\n按2告诉我我不知道的",
            AppState::Ask => "🙋问吧",
            AppState::Tell => "📖报告地址?",
        };
        write!(f, "{s}")
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
