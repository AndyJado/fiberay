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
    pub row_size: usize,
    // i bound
    pub col_size: usize,
}

impl StrTable {
    pub fn index(&self, i: usize, j: usize) -> Option<String> {
        let base = self.row_size * (i - 1);
        let index = j - 1 + base;
        self.stream[index].clone()
    }

    pub fn debugy(&self) {
        for i in 1..=self.col_size {
            for j in 1..=self.row_size {
                let cell = self.index(i, j);
                dbg!(i, j, cell);
            }
        }
    }
}

// FIXME: consumes Block, only deal with table, I know, I know..
pub struct BlockGal(pub pandoc_ast::Block);

impl BlockGal {
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
                    row_size: row_len,
                    col_size: col_len,
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
                Inline::Str(s) => s.to_string(),
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
