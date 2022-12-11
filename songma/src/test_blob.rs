use crate::doc_reader::StrTable;
use crate::edges::*;
use crate::vertexes::*;

pub trait TestSuite {
    fn a_report_bulk(&self);
}

impl TestSuite for crate::doc_reader::DocMap {
    fn a_report_bulk(&self) {
        let pograms = self.tables_under_section("programme");
        let pogram_tb = the_programme_table(pograms).expect("programme table");
        let col = pogram_tb.col(1).into_iter().skip(1);
        for i in col {
            dbg!(i);
        }
    }
}

fn the_programme_table(mut pograms: Vec<&StrTable>) -> std::io::Result<&StrTable> {
    if !pograms.len() == 1 {
        panic!("one report should have programe table")
    };
    let Some(pogram) = pograms.pop() else { panic!() };
    Ok(pogram)
}
