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

pub fn all_table_fisrt(tables: &Vec<StrTable>) {
    for i in tables {
        let s = i.index(1, 1);
        println!("{s:?}");
    }
}

pub fn index_contain(t: &StrTable, i: usize, j: usize, s: &str) -> bool {
    let Some(cell) = t.index(i, j) else {return false};
    if cell.contains(s) {
        // for i in 1..=t.col_size {
        match t.index(i, j) {
            Some(s) => println!("{s}"),
            None => return true,
        }
        // }
        return true;
    }
    false
}

/// print table cell at index $2 $3
pub fn pan_table(ast: pandoc_ast::Pandoc, args: Vec<String>) {
    let mut blocks = ast.blocks.into_iter().peekable();
    let (i, j) = (
        &args[2].parse::<usize>().unwrap(),
        &args[3].parse::<usize>().unwrap(),
    );
    loop {
        // begin with a block
        let Some(pre_table_block) = blocks.next() else { return };
        // peek a Table
        let Some(&Block::Table(..)) = blocks.peek() else { continue };
        let table_title = pre_table_block.to_string();
        let table_block = blocks.next().unwrap();
        let table = BlockGal(table_block).do_table().unwrap();
        dbg!(table_title);
        dbg!(table.index(*i, *j));
        'inner: loop {
            let Some(&Block::Table(..)) = blocks.peek() else { break 'inner };
            let table_block = blocks.next().unwrap();
            let table = BlockGal(table_block).do_table().unwrap();
            // dbg!(table.debugy());
            dbg!(table.index(*i, *j));
        }
    }
}
