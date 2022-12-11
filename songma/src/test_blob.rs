use crate::doc_reader::StrTable;
use crate::edges::*;
use crate::vertexes::*;
use indradb::BulkInsertItem;
use indradb::Datastore;
use indradb::Vertex;
use regex::Regex;

pub trait TestSuite {
    type Db;
    fn a_report_bulk(&self, db: &Self::Db);
}

impl TestSuite for crate::doc_reader::DocMap {
    fn a_report_bulk(&self, db: &Self::Db) {
        let mut bulks: Vec<BulkInsertItem> = vec![];
        let v_id = |bulk: &Vec<BulkInsertItem>| {
            let v = &bulk[0];
            let BulkInsertItem::Vertex(Vertex { ref id,.. })= v else {panic!()};
            id.clone()
        };
        let pro_id = {
            let tb = self.tables_under_section("roofless")[0];
            let report = TestReport {
                id: tb.index(5, 2),
                title: tb.index(1, 2),
            };
            let product = Product {
                client: tb.index(3, 2),
                material: tb.index(2, 2),
            };
            let mut v_rep = report.vertex_with_property();
            let rep_id = v_id(&v_rep);
            let mut v_prod = product.vertex_with_property();
            let pro_id = v_id(&v_prod);
            let key = Sealed::edgekey(&Sealed, rep_id, pro_id);
            bulks.append(&mut v_prod);
            bulks.append(&mut v_rep);
            bulks.push(BulkInsertItem::Edge(key));
            // let seal = Sealed;
            pro_id
        };
        {
            let pograms = self.tables_under_section("programme");
            let pogram_tb = the_programme_table(pograms).expect("programme table");
            let col_iter = |j: usize| pogram_tb.col(j).into_iter().skip(1);
            let id_col = col_iter(4).map(|c| TestProgramme::from_code(c));
        }
        db.bulk_insert(bulks).expect("bulk insert");
        dbg!(db);
    }

    type Db = indradb::RocksdbDatastore;
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
