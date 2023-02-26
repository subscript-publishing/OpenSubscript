#![allow(unused)]
use character::{UnconsSpec, StaticCharPredicate};
use output::IO;
use stream::{Cursor, IndexedChar, Stream};
use syntax::{ParseRootAst, ast_parser, cmd::{ParseBackslashCmd, ParseTrailingLabeledArguments}, parts::{ParseLabelIdentifier, ParseLabeledEnclosure}};
// use syntax::{ParseRootAst, parts::{ParseAstNodeInSomeEnclosure, ParseAstNodeInCurlyBrackets}, plain_text::ParsePlainText};

pub mod ast;
pub mod binders;
pub mod character;
pub mod output;
pub mod stream;
pub mod syntax;
pub mod token;

fn main() {
    let source_code = include_str!("../samples/parsing/file1.txt");
    // let source_code = "\\h1{Hello World}\n@alpha {Lorem Ipsum}\n@beta {Lorem Ipsum}";
    let source_slice = source_code
        .chars()
        .enumerate()
        .map(|(ix, c)| IndexedChar::new(ix, c))
        .collect::<Vec<_>>();
    let source_stream = Stream {
        slice: &source_slice[..],
        cursor: Cursor::ZERO,
    };
    // let result = source_stream.partition_where_false(StaticCharPredicate::IS_NOT_NEWLINE);
    // println!("{result:#?}");
    // let op = ParseIndentedAsteriskItem {
    //     column_level: 4,
    // };
    // let op = ParseIndentedList::default();
    // let op = ParseAstNodeInCurlyBrackets::default();
    // let op = ParsePlainText::default();
    // let output = source_stream
    //     .static_threesome((
    //         &|stream| stream.apply_binder(UnconsSpec::must_match('{')),
    //         &|stream| stream.apply_binder(op),
    //         &|stream| stream.apply_binder(UnconsSpec::must_match('}')),
    //     ));
    // let op = ast_parser::ParseInSquareBrackets::default();
    // let op = ast_parser::ParseEverything::default();
    // let op = ParseTrailingLabeledArguments::default();
    // let op = ParseLabeledEnclosure::default();
    // let op = ParseBackslashCmd::default();
    let op = ast_parser::ParseEverything::default();
    let output = source_stream.apply_binder(op);
    match output {
        output::Output::Success(IO { context, value }) => {
            println!("{value:#?}");
            if context.slice.is_empty() {
                println!("SUCCESS: FULLY CONSUMED")
            } else {
                println!("OK: NOT FULLY CONSUMED")
            }
        }
        output::Output::Failure(IO { context, value }) => {
            println!("{context:#?}");
            println!("VALUE: {value:#?}");
            println!("FAILED")
        }
    }
}
