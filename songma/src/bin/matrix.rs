use pandoc_ast::Block;
use songma::doc_reader::{pan_read, BlockGal, Pan2Str};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let ast = pan_read(&args[1]);
    let tables: Vec<_> = ast
        .blocks
        .into_iter()
        .filter_map(|b| BlockGal(b).do_table())
        .collect();
    for t in &tables {
        let Some(hed) = t.index(1, 1) else {continue};
        if hed.contains("item") {
            for i in 1..=t.col_size {
                dbg!(t.index(i, 1));
            }
        }
    }
    // let t_tts = tables.map(|t| t.index(1, 1)).take_while(|s| s.is_some());
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
        let Some(pre_table_block) = blocks.next() else { break };
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
