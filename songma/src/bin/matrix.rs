use std::env::args;

use pandoc::{OutputKind, PandocOutput};
use pandoc_ast::Block;
use songma::doc_reader::{BlockGal, Pan2Str};

fn main() {
    let args: Vec<String> = args().collect();

    // run actual pandoc, get ast iter
    let mut pandoc = pandoc::new();
    pandoc
        .add_input(&args[1])
        .set_output(OutputKind::Pipe)
        .set_output_format(pandoc::OutputFormat::Json, vec![]);
    let PandocOutput::ToBuffer(json) = pandoc.execute().expect("pandoc should execute") else {
        panic!("pandoc should gen json ast as buffer")
    };
    let ast = pandoc_ast::Pandoc::from_json(&json);
    // do what we want
    pan_table(ast, args)
}

fn pan_table(ast: pandoc_ast::Pandoc, args: Vec<String>) {
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
        let table = BlockGal(table_block).do_table();
        dbg!(table_title);
        dbg!(table.index(*i, *j));
        'inner: loop {
            let Some(&Block::Table(..)) = blocks.peek() else { break 'inner };
            let table_block = blocks.next().unwrap();
            let table = BlockGal(table_block).do_table();
            // dbg!(table.debugy());
            dbg!(table.index(*i, *j));
        }
    }
}
