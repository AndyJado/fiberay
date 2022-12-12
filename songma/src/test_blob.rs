use crate::doc_reader::StrTable;
use crate::edges::*;
use crate::vertexes::*;
use indradb::BulkInsertItem;
use indradb::Vertex;
use serde_json::json;

pub trait TestSuite {
    type Bulks;
    fn a_report_bulk(&self) -> Self::Bulks;
}

impl TestSuite for crate::doc_reader::DocMap {
    type Bulks = Vec<BulkInsertItem>;
    fn a_report_bulk(&self) -> Self::Bulks {
        let mut bulks: Vec<BulkInsertItem> = vec![];
        // get vertex id from bulk
        let v_id = |bulk: &Vec<BulkInsertItem>| {
            let v = &bulk[0];
            let BulkInsertItem::Vertex(Vertex { ref id,.. })= v else {panic!()};
            id.clone()
        };
        // FIXME: currently one report one sample vertex
        let sample_id = {
            let tb = self.tables_under_section("roofless")[0];
            let report = TestReport {
                id: tb.index(5, 2),
                title: tb.index(1, 2),
            };
            let sample = Sample { id: tb.index(5, 2) };
            let product = Product {
                client: tb.index(3, 2),
                material: tb.index(2, 2),
            };
            let mut v_rep = report.vertex_with_property();
            let rep_id = v_id(&v_rep);
            let mut v_samp = sample.vertex_with_property();
            let samp_id = v_id(&v_samp);
            let mut v_prod = product.vertex_with_property();
            let pro_id = v_id(&v_prod);
            let key = Sealed::edgekey(&Sealed, rep_id, pro_id);
            bulks.append(&mut v_prod);
            bulks.append(&mut v_rep);
            bulks.append(&mut v_samp);
            bulks.push(BulkInsertItem::Edge(key));
            // let seal = Sealed;
            samp_id
        };
        {
            let col_iter = |j: usize, t: &StrTable| t.col(j).into_iter().skip(1);
            // find the table for Test edge
            let pograms = self.tables_under_section("programme");
            let pogram_tb = the_programme_table(pograms).expect("programme table");
            let id_col = col_iter(4, pogram_tb).map(|c| TestProgramme::from_code(c));
            // section list, linearly
            let sec_list = &mut self.keys.iter();
            for code in id_col {
                // a test edge
                let Some(desc_key) = sec_list.find(|&c| c.contains("description")) else {continue};
                let Some(desc_table) = self.map.get(desc_key) else {continue};
                let desc_table = desc_table.last();
                let e_test = code.test_from(desc_table);
                // guarantee a body vertex
                let Some(result_key) = sec_list.find(|&c| c.contains("result")) else {continue};
                let Some(result_table) = self.map.get(result_key) else {continue};
                let v_body = code.body_from(result_table.last().unwrap());
                // bulk gen
                let body_bulk = &mut v_body.vertex_with_property();
                let body_uid = v_id(body_bulk);
                // test result as property of failed body
                let results = &mut code.reult_bulk_from(body_uid, result_table.last().unwrap());
                let test_bulk = &mut e_test.edge_with_property(sample_id, body_uid);
                bulks.append(body_bulk);
                bulks.append(test_bulk);
                bulks.append(results);
            }
            // if there is a description table
        }
        dbg!(bulks)
    }
}

fn the_programme_table(mut pograms: Vec<&StrTable>) -> std::io::Result<&StrTable> {
    if !pograms.len() == 1 {
        panic!("one report should have programe table")
    };
    let Some(pogram) = pograms.pop() else { panic!() };
    Ok(pogram)
}

#[derive(Debug)]
enum TestProgramme {
    T0,
    T90,
    C0,
    C90,
    V,
    M,
    Tg,
    NotYet,
}

impl TestProgramme {
    fn reult_bulk_from(&self, body_id: uuid::Uuid, tb: &StrTable) -> Vec<BulkInsertItem> {
        let mut bulks: Vec<BulkInsertItem> = vec![];
        let mut bulk_a = |s: &str, v: serde_json::Value| {
            let bulk = BulkInsertItem::VertexProperty(
                body_id,
                indradb::Identifier::new(s).expect("test result gen indentifier"),
                v,
            );
            bulks.push(bulk);
        };
        let mut max_min = |s: &str, j: usize| {
            bulk_a(
                s,
                json!([
                    tb.index(11, j)
                        .unwrap()
                        .parse::<f32>()
                        .expect("parse cell to f32"),
                    tb.index(12, j)
                        .unwrap()
                        .parse::<f32>()
                        .expect("parse cell to f32"),
                ]),
            )
        };
        use TestProgramme::*;
        match self {
            T0 => {
                max_min("b", 2);
                max_min("h", 3);
                max_min("Fm", 4);
                max_min("Sm", 5);
                max_min("E", 6);
                max_min("Epm", 7);
                max_min("Mu", 8);
            }
            T90 => {
                max_min("b", 2);
                max_min("h", 3);
                max_min("Fm", 4);
                max_min("Sm", 5);
                max_min("E", 6);
                max_min("Epm", 7);
            }
            C0 => {
                max_min("b", 2);
                max_min("h", 3);
                max_min("Fm", 4);
                max_min("Sm", 5);
                max_min("E", 6);
                max_min("Epm", 7);
            }
            C90 => {
                max_min("b", 2);
                max_min("h", 3);
                max_min("Fm", 4);
                max_min("Sm", 5);
                max_min("E", 6);
                max_min("Epm", 7);
            }
            V => {
                max_min("b", 2);
                max_min("h", 3);
                max_min("Fm", 4);
                max_min("Fu", 5);
                max_min("G", 6);
            }
            _ => {
                dbg!("some tests results not included");
            }
        }
        bulks
    }
    /// a test edge
    fn test_from(&self, tb: Option<&StrTable>) -> Test {
        let (instrument, standard, execution) = if let Some(desc_tb) = tb {
            let c = |i: usize, j: usize| desc_tb.index(i, j);
            (c(2, 2), c(1, 2), c(6, 3))
        } else {
            // have descrp section but no table under case
            (None, None, None)
        };
        Test {
            code: self.desc(),
            instrument,
            standard,
            execution,
        }
    }
    /// read table a fail body vertex to be
    fn body_from(&self, tb: &StrTable) -> FailedBody {
        use TestProgramme::*;
        let fail_mode = match self {
            T0 | C0 | T90 => tb.index(2, 9),
            C90 | V => tb.index(2, 9),
            _ => None,
        };
        FailedBody { fail_mode }
    }

    fn desc(&self) -> String {
        match self {
            TestProgramme::T0 => "TO",
            TestProgramme::T90 => "T90",
            TestProgramme::C0 => "C0",
            TestProgramme::C90 => "C90",
            TestProgramme::V => "V",
            TestProgramme::M => "M",
            TestProgramme::Tg => "Tg",
            TestProgramme::NotYet => unimplemented!(),
        }
        .to_owned()
    }
    fn from_code(s: Option<String>) -> Self {
        let Some(s) = s else {return NotYet};
        use TestProgramme::*;
        let match_code = |code: &str| s.ends_with(code);
        if match_code("MXX") {
            M
        } else if match_code("T90XX") {
            T90
        } else if match_code("C90XX") {
            C90
        } else if match_code("C0XX") {
            C0
        } else if match_code("T0XX") {
            T0
        } else if match_code("VXX") {
            V
        } else {
            Tg
        }
    }
}
