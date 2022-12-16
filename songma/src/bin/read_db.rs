use itertools::Itertools;
use serde_json::json;
use simple_excel_writer::{Row, ToCellValue};
use std::io::stdin;

use indradb::*;
use songma::{
    client::AppState,
    vertexes::{Product, Sample, TestReport},
};

fn main() -> anyhow::Result<()> {
    // Create an in-memory datastore
    let p = &"./image.rdb";
    let db = RocksdbDatastore::new(p, None)?;
    // dbg!(title_all);
    client(&db).unwrap();
    Ok(())
}

fn client(db: &RocksdbDatastore) -> anyhow::Result<()> {
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
                let q = q_v_id(Sample::iden())
                    .outbound()
                    .t(f("Test"))
                    .with_property_equal_to(f("code"), json!(test_code))
                    .inbound();
                // failed body ppts vec
                let Some(mut pptss) = db.get_all_vertex_properties(q.into()).ok() else {continue};
                let cell_iter = pptss
                    .iter_mut()
                    .map(|ppts| {
                        let v_id = ppts.vertex.id;
                        let q_prod = SpecificVertexQuery::single(v_id)
                            .inbound()
                            .outbound()
                            .t(Sample::iden())
                            .inbound()
                            .outbound()
                            .t(Product::iden());
                        let q_rep = q_prod.clone().inbound().outbound().t(TestReport::iden());
                        let v_ppts = ppts.props.iter_mut();
                        (q_rep, q_prod, v_ppts)
                    })
                    .map(|c| {
                        let rep_id = db.get_vertex_properties(c.0.property(f("id"))).unwrap();
                        let prod_mat = db
                            .get_vertex_properties(c.1.property(f("material")))
                            .unwrap();
                        let mut max_min =
                            c.2.filter(|p| p.name != f("fail_mode"))
                                .map(|ppt| {
                                    dbg!(&ppt.value);
                                    ppt.value
                                        .as_array()
                                        .unwrap()
                                        .into_iter()
                                        .filter_map(|v| v.as_f64())
                                        .map(|v| v.to_cell_value())
                                        .collect_vec()
                                })
                                .flatten()
                                .rev()
                                .collect_vec();
                        max_min.push(ppts2str(&prod_mat).to_cell_value());
                        max_min.push(ppts2str(&rep_id).to_cell_value());
                        max_min.into_iter().rev()
                    });
                let mut excel = simple_excel_writer::Workbook::create("query_result.xlsx");
                let mut sheet = excel.create_sheet("query1");
                excel
                    .write_sheet(&mut sheet, |w| {
                        for i in cell_iter {
                            w.append_row(Row::from_iter(i)).unwrap();
                        }
                        Ok(())
                    })
                    .unwrap();
                excel.close().unwrap();
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
