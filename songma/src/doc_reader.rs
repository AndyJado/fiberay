use std::collections::HashMap;

use pandoc_ast::{Block, Inline};

pub fn pan_read(p: &str) -> pandoc_ast::Pandoc {
    let mut pandoc = pandoc::new();
    pandoc
        .add_input(&p)
        .set_output(pandoc::OutputKind::Pipe)
        .set_output_format(pandoc::OutputFormat::Json, vec![]);
    let pandoc::PandocOutput::ToBuffer(json) = pandoc.execute().expect("pandoc should execute") else {
        panic!("pandoc should gen json ast as buffer")
    };
    pandoc_ast::Pandoc::from_json(&json)
}

/// table as a matrix
#[derive(Debug)]
pub struct StrTable {
    // row followes a row
    pub stream: Vec<Option<String>>,
    // j bound
    pub j_bound: usize,
    // i bound
    pub i_bound: usize,
}

impl StrTable {
    pub fn index(&self, i: usize, j: usize) -> Option<String> {
        let base = self.j_bound * (i - 1);
        let index = j - 1 + base;
        self.stream[index].clone()
    }

    pub fn col(&self, j: usize) -> Vec<Option<String>> {
        (1..=self.i_bound)
            .into_iter()
            .map(|i| self.index(i, j))
            .collect()
    }

    pub fn debugy(&self) {
        for i in 1..=self.i_bound {
            for j in 1..=self.j_bound {
                let cell = self.index(i, j);
                dbg!(i, j, cell);
            }
        }
    }
}

// FIXME: consumes Block, only deal with table, I know, I know..
pub struct BlockGal(pub pandoc_ast::Block);

impl BlockGal {
    /// read para ast, return the **stronged word**
    pub fn do_para(&self) -> Option<String> {
        let Block::Para(ref v) = self.0 else {return None};
        let strongs: Vec<_> = v
            .clone()
            .into_iter()
            .filter_map(|ln| match ln {
                Inline::Strong(s) => Some(s),
                _ => None,
            })
            .flatten()
            .collect();
        strongs.to_string()
    }
    // return a table with matrix index empowered
    pub fn do_table(self) -> Option<StrTable> {
        match self.0 {
            // head.1 & body.i.2&.3 are rows, .2 is empty empirically
            Block::Table(_, _, _, head, mut body, _) => {
                // dbg!(&head.1.len());
                let mut table = Self::rows2vec(head.1);
                let row_len = table.len();
                if row_len == 0 {
                    return None;
                };
                // dbg!(body.len());
                let body = body.pop().expect("table body should has row");
                let col_len = body.3.len() + 1;
                let mut bodys = Self::rows2vec(body.3);
                table.append(&mut bodys);
                let is_matrix = table.len() / row_len == col_len;
                if !is_matrix {
                    return None;
                    // unimplemented!("{row_len} {col_len}\n fatal: table can not be a matrix!:")
                };
                Some(StrTable {
                    stream: table,
                    j_bound: row_len,
                    i_bound: col_len,
                })
            }
            _ => None,
        }
    }

    /// pandoc rows to vec string
    fn rows2vec(rows: Vec<pandoc_ast::Row>) -> Vec<Option<String>> {
        rows.into_iter().flat_map(|r| Self::row2vec(r)).collect()
    }

    /// a pandoc row to a vec of string, not all type covered.
    fn row2vec(row: pandoc_ast::Row) -> Vec<Option<String>> {
        // cells
        let head_cells = row.1;
        let cell_blocks = head_cells.into_iter().map(|row| row.4);
        cell_blocks
            .map(|blos| blos.into_iter().map(|blo| blo.to_string()).collect())
            .collect()
    }
}

pub trait Pan2Str {
    fn to_string(self) -> Option<String>;
}

impl Pan2Str for Block {
    fn to_string(self) -> Option<String> {
        let inlines = match self {
            Block::Plain(mut v) => std::mem::take(&mut v),
            Block::Para(mut v) => std::mem::take(&mut v),
            Block::BlockQuote(v) => {
                let ss: Vec<_> = v.into_iter().filter_map(|b| b.to_string()).collect();
                let cs: String = ss.iter().flat_map(|s| s.chars()).collect();
                return Some(cs);
            }
            Block::OrderedList(_, v) => {
                let ss: Vec<_> = v
                    .into_iter()
                    .flatten()
                    .filter_map(|b| b.to_string())
                    .collect();
                let cs: String = ss.iter().flat_map(|s| s.chars()).collect();
                return Some(cs);
            }
            _ => {
                let b_dbg = format!("!!{self:#?}");
                let b_ty = b_dbg.chars().take_while(|cha| *cha != '(').collect();
                return Some(b_ty);
            }
        };
        inlines.to_string()
    }
}

impl Pan2Str for Vec<Inline> {
    fn to_string(self) -> Option<String> {
        let mut ln = String::new();
        for word in self {
            let cat = match word {
                Inline::Str(s) => s,
                Inline::Emph(v) => v.to_string().unwrap_or_default(),
                Inline::Space => "_".to_string(),
                Inline::SoftBreak => "".to_string(),
                Inline::LineBreak => "".to_string(),
                Inline::Subscript(v) => v.to_string().unwrap_or_default(),
                Inline::Superscript(v) => v.to_string().unwrap_or_default(),
                Inline::Underline(v) => v.to_string().unwrap_or_default(),
                Inline::Strong(v) => v.to_string().unwrap_or_default(),
                Inline::Image(_, _v, _) => "image".to_string(),
                Inline::Math(_, v) => v,
                Inline::Span(_, v) => v.to_string().unwrap_or_default(),
                Inline::Quoted(_, v) => v.to_string().unwrap_or_default(),
                Inline::Link(_, v, _) => v.to_string().unwrap_or_default(),
                Inline::RawInline(_, s) => s,
                // _ => continue,
                _ => unimplemented!("to str not for this Inline: {:#?}", word),
            };
            ln = ln + &cat;
        }
        if ln.is_empty() {
            None
        } else {
            Some(ln)
        }
    }
}

pub struct DocMap {
    pub map: HashMap<String, Vec<StrTable>>,
    pub keys: Vec<String>,
}

impl DocMap {
    /// generate one from ast
    pub fn new(ast: pandoc_ast::Pandoc) -> Self {
        let mut section_table_map: HashMap<String, Vec<StrTable>> = HashMap::new();
        let mut keys: Vec<String> = vec![];
        let mut outter_table: Vec<StrTable> = vec![];
        for b in ast.blocks.into_iter() {
            let gal = BlockGal(b);
            match gal.do_para() {
                // **1. xxx*
                Some(s) if s.chars().next().unwrap_or_default().is_numeric() => {
                    keys.push(s);
                    section_table_map.insert(keys.last().unwrap().clone(), vec![]);
                }
                // not a para block
                None => {
                    let Some(t) = gal.do_table() else {continue};
                    if let Some(k) = keys.last().cloned() {
                        section_table_map.entry(k).and_modify(|ts| ts.push(t));
                    } else {
                        outter_table.push(t);
                    }
                }
                // **but what**
                _ => continue,
            };
        }
        if !outter_table.is_empty() {
            //FIXME: handcoded key
            section_table_map.insert("roofless".to_owned(), outter_table);
        };
        Self {
            map: section_table_map,
            keys,
        }
    }
    // FIXME: might use regex for finer control
    pub fn tables_under_section(&self, name: &str) -> Vec<&StrTable> {
        let keys: Vec<_> = self
            .map
            .keys()
            .filter(|k| {
                let s = *k;
                s.as_str().contains(name)
            })
            .collect();
        // dbg!(&keys);
        let tbs: Vec<_> = keys
            .into_iter()
            .filter_map(|k| self.map.get(k))
            .flatten()
            .collect();
        tbs
    }

    /// print to std
    pub fn dbgy(&self) {
        for k in &self.keys {
            self.dbg_table(k);
        }
        self.dbg_table("roofless");
    }

    /// dbg a table
    pub fn dbg_table(&self, k: &str) {
        let Some(ts) = self.map.get(k) else {return};
        println!("{k}");
        for t in ts {
            for i in 1..=t.i_bound {
                let mut row = String::new();
                for j in 1..=t.j_bound {
                    let cell = format!("{}; ", &t.index(i, j).unwrap_or_default());
                    row.push_str(&cell);
                }
                println!("{row}");
                row.clear()
            }
        }
    }
}
