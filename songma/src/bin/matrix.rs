use pandoc_ast::Block;
use songma::doc_reader::{pan_read, BlockGal, Pan2Str, StrTable};
use std::{collections::HashMap, env::args};

fn main() {
    let args: Vec<String> = args().collect();
    let ast = pan_read(&args[1]);
    let doc_map = DocMap::new(ast);
    doc_map.dbgy();
    println!("======================");
}

pub struct DocMap {
    map: HashMap<String, Vec<StrTable>>,
    keys: Vec<String>,
}

impl DocMap {
    /// generate one from ast
    fn new(ast: pandoc_ast::Pandoc) -> Self {
        let mut section_table_map: HashMap<String, Vec<StrTable>> = HashMap::new();
        let mut keys: Vec<String> = vec![];
        for b in ast.blocks.into_iter() {
            let gal = BlockGal(b);
            match gal.do_para() {
                Some(s) if s.chars().next().unwrap_or_default().is_numeric() => {
                    keys.push(s);
                    section_table_map.insert(keys.last().unwrap().clone(), vec![]);
                }
                None => {
                    let Some(t) = gal.do_table() else {continue};
                    let Some(k) = keys.last().cloned() else {continue};
                    section_table_map.entry(k).and_modify(|ts| ts.push(t));
                }
                _ => continue,
            };
        }
        Self {
            map: section_table_map,
            keys,
        }
    }

    /// print to std
    fn dbgy(&self) {
        for k in &self.keys {
            let Some(ts) = self.map.get(k) else {continue};
            println!("{k}");
            for t in ts {
                for i in 1..=t.col_size {
                    let mut row = String::new();
                    for j in 1..t.row_size {
                        let cell = format!("{}; ", &t.index(i, j).unwrap_or_default());
                        row.push_str(&cell);
                    }
                    println!("{row}");
                    row.clear()
                }
            }
        }
    }

        }
    }
}
