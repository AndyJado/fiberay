use std::{io::stdin, mem::swap};

use indradb::{Datastore, RangeVertexQuery, RocksdbDatastore};

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

enum AppState {
    Welcome,
    Ask,
    Tell,
    DarkCorner,
    Walking,
}

impl AppState {
    fn home(&mut self) {
        swap(self, &mut Self::Welcome)
    }
    fn ask(&mut self) {
        swap(self, &mut Self::Ask)
    }
    fn lost(&mut self) {
        swap(self, &mut Self::DarkCorner)
    }
    fn walking(&mut self) {
        swap(self, &mut Self::Walking)
    }
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AppState::Welcome => {
                "👋,你好哇,又见面了\n按1问我我知道的\n按2告诉我我不知道的\n Ctrl C 退出"
            }
            AppState::Ask => "🙋问吧",
            AppState::Tell => "📖报告地址?",
            AppState::DarkCorner => "你不该来这的,回去吧",
            AppState::Walking => "好了,现在呢?",
        };
        std::write!(f, "{s}")
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
