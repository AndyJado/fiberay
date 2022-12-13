use itertools::Itertools;
use serde_json::json;
use std::io::stdin;

use indradb::{
    Datastore, EdgePropertyQuery, EdgeQueryExt, Identifier, PipePropertyPresenceEdgeQuery,
    PipePropertyValueEdgeQuery, PipePropertyValueVertexQuery, PropertyPresenceEdgeQuery,
    PropertyValueEdgeQuery, PropertyValueVertexQuery, RangeVertexQuery, RocksdbDatastore,
    VertexPropertyQuery, VertexQuery, VertexQueryExt,
};
use songma::{
    client::AppState,
    vertexes::{Sample, TestReport},
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory datastore
    let p = &"./image.rdb";
    let db = RocksdbDatastore::new(p, None)?;
    // dbg!(title_all);
    client(&db);
    Ok(())
}

fn client(db: &RocksdbDatastore) {
    // let q_v_f_val = |c: Identifier, v: Value| PropertyValueVertexQuery { name: c, value: v };
    // let q_v_f = |c: Identifier| PropertyPresenceVertexQuery { name: c };
    let f = |s: &str| Identifier::new(s).expect("typed in wrong code!");
    let q_v_id = |v_id: Identifier| RangeVertexQuery::new().t(v_id);
    let q_p_of_v = |v_id: Identifier, p_id: Identifier| VertexPropertyQuery {
        inner: q_v_id(v_id).into(),
        name: p_id,
    };
    let q_e_with_p_id = |ep: Identifier| PropertyPresenceEdgeQuery { name: ep };
    // edge has e_id out has property value
    let q_vp_of_ep_out = |ep: Identifier, vp: Identifier| VertexPropertyQuery {
        inner: q_e_with_p_id(ep).outbound().into(),
        name: vp,
    };
    let mut buf = String::new();
    let mut state = AppState::Welcome;
    let q_title_all = q_v_id(TestReport::iden());
    let titles = db
        .get_vertex_properties(VertexPropertyQuery {
            inner: q_title_all.into(),
            name: f("title"),
        })
        .ok()
        .unwrap();
    let mut i_ids = titles.iter().map(|c| c.id);
    let edge_count = db
        .get_edge_count(
            i_ids.next().unwrap(),
            None,
            indradb::EdgeDirection::Outbound,
        )
        .unwrap();
    dbg!(edge_count);
    let uniq_ttl = titles
        .iter()
        .filter_map(|p| p.value.as_str())
        .map(|s| s.to_ascii_lowercase())
        .unique()
        .join("\n");
    println!("目前已经导入的报告数为:{}\n", titles.len());
    println!("目前可供查询的报告大类为:\n{}", uniq_ttl);
    loop {
        // next run
        buf.clear();
        println!("{state}");
        let stdin = stdin();
        stdin.read_line(&mut buf).ok();
        let mut input = buf.trim_start().trim_end().split_whitespace();
        match state {
            AppState::Welcome => {
                let Some(test_code) = input.next() else {continue};
                let Some(ppt_code) = input.next() else {continue};
                let q = q_v_id(Sample::iden())
                    .outbound()
                    .t(f("Test"))
                    .with_property_equal_to(f("code"), json!(test_code))
                    .inbound()
                    .property(f(ppt_code));
                let Some(ppts) = db.get_vertex_properties(q.into()).ok() else {continue};
                // dbg!(ppts);
                let id_max_min: Vec<_> = ppts
                    .iter()
                    .map(|vp| {
                        let val = vp.value.as_array().unwrap();
                        let max = val.first().unwrap().as_f64().unwrap();
                        let min = val.first().unwrap().as_f64().unwrap();
                        (vp.id, max, min)
                    })
                    .collect();
                dbg!(id_max_min);
            }
            _ => state.home(),
        }
    }
}

#[test]
fn value_eq() {
    assert_eq!(json!("s"), serde_json::Value::String("s".to_owned()));
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
