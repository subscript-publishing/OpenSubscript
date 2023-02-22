use ast::ParseInSomeEnclosure;
use stream::{IndexedChar, Cursor, Stream};

pub mod character;
pub mod cursor;
pub mod stream;
pub mod output;
pub mod binders;
pub mod token;
pub mod ast;
pub mod syntax;

fn main() {
    // let source_code = include_str!("../samples/misc/random2.txt");
    let source_code = "{a}";
    let source_slice = source_code.chars().enumerate().map(|(ix, c)| IndexedChar::new(ix, c)).collect::<Vec<_>>();
    let source_stream = Stream {
        slice: &source_slice[..],
        cursor: Cursor { index: 0, column: 0, line: 0 },
    };
    // let op = ParseIndentedAsteriskItem {
    //     column_level: 4,
    // };
    let op = ParseInSomeEnclosure::default();
    // let op = ParseIndentedList::default();
    let output = source_stream.apply_binder(op);
    println!("{output:#?}")
}
