use itertools::Itertools;
use serde_json::json;
use std::io::stdin;

use indradb::*;
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
    // will interact with loop
    let mut buf = String::new();
    let mut state = AppState::Welcome;
    // helper closure
    let f = |s: &str| Identifier::new(s).expect("typed in wrong code!");
    let q_v_id = |v_id: Identifier| RangeVertexQuery::new().t(v_id);
    let ppt_v = |q: VertexPropertyQuery| db.get_vertex_properties(q).expect("get ppt from db of v");
    // general info
    let q_title_all = q_v_id(TestReport::iden());
    let titles = db
        .get_vertex_properties(q_title_all.clone().property(f("title")))
        .expect("report titles from bd");
    let uniq_ttl = ppts2str(&titles);
    println!("目前已经导入的报告数为:{}\n", titles.len());
    println!("目前可供查询的报告大类为:\n{}", uniq_ttl);
    let q_product = q_title_all.outbound().inbound().property(f("material"));
    let producs = db.get_vertex_properties(q_product.into()).unwrap();
    let uniq_produ = ppts2str(&producs);
    println!("目前可供查询的材料统有:\n{}", uniq_produ);
    // daemon
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
                // dbg!(&ppts);
                let id_max_min = ppts.iter().map(|vp| {
                    let prod_query = SpecificVertexQuery::single(vp.id)
                        .clone()
                        .inbound()
                        .outbound()
                        .t(Sample::iden())
                        .inbound()
                        .outbound()
                        .t(songma::vertexes::Product::iden());
                    let rep_query = prod_query
                        .clone()
                        .inbound()
                        .outbound()
                        .t(songma::vertexes::TestReport::iden());
                    let val = vp.value.as_array().unwrap();
                    let max = val.first().unwrap().as_f64().unwrap();
                    let min = val.last().unwrap().as_f64().unwrap();
                    (
                        (
                            prod_query.property(f("material")),
                            rep_query.property(f("id")),
                        ),
                        (max, min),
                    )
                });
                // so last() return the max
                let result_list_pop_max: Vec<_> = id_max_min
                    .sorted_by(|zip1, zip2| zip1.1 .0.partial_cmp(&zip2.1 .0).unwrap())
                    .collect();
                let ((q_v_prod, q_v_rep), (_, max_val)) = result_list_pop_max.last().unwrap();
                let prod_desc = ppts2str(&ppt_v(q_v_prod.clone()));
                let report_desc = ppts2str(&ppt_v(q_v_rep.clone()));
                println!(
                    "最大值:\n报告:{},产品:{},值:{}",
                    report_desc, prod_desc, max_val
                );

                // so last() return min
                let result_list_pop_min: Vec<_> = result_list_pop_max
                    .into_iter()
                    .sorted_by(|zip1, zip2| zip2.1 .1.partial_cmp(&zip1.1 .1).unwrap())
                    .collect();
                let ((q_v_prod, q_v_rep), (_, min_val)) = result_list_pop_min.last().unwrap();
                let prod_desc = ppts2str(&ppt_v(q_v_prod.clone()));
                let report_desc = ppts2str(&ppt_v(q_v_rep.clone()));
                println!(
                    "最小值:\n报告:{},产品:{},值:{}",
                    report_desc, prod_desc, min_val
                );
            }
            _ => state.home(),
        }
    }
}

fn ppts2str(ppts: &Vec<VertexProperty>) -> String {
    ppts.iter()
        .filter_map(|p| p.value.as_str())
        .map(|s| s.to_ascii_lowercase())
        .unique()
        .join("\n")
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
