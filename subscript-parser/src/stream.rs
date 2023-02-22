use std::fmt::Display;

use crate::{binders::StreamBinder, output::{Output, IO}};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteIndex(pub usize);

impl From<usize> for ByteIndex {
    fn from(value: usize) -> Self { ByteIndex(value) }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexedChar {
    pub index: ByteIndex,
    pub char: char,
}

impl IndexedChar {
    pub fn new(index: impl Into<ByteIndex>, char: char) -> Self {
        Self { char, index:  index.into()}
    }
}

impl From<IndexedChar> for char {
    fn from(value: IndexedChar) -> Self { value.char }
}
impl AsRef<char> for IndexedChar {
    fn as_ref(&self) -> &char { &self.char }
}
impl Display for IndexedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.char))
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub index: usize,
    pub column: usize,
    pub line: usize,
}

impl Cursor {
    pub fn forward_sync_for<T: AsRef<char>>(self, span: &[T]) -> Cursor {
        let index = self.index + span.len();
        let mut column = self.column;
        let mut line = self.line;
        for x in span.into_iter().map(|x| *x.as_ref()) {
            if x == '\n' {
                line = line + 1;
                column = 0;
                continue;
            }
            column = column + 1;
        }
        Self { index, column, line }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// STREAM
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Debug, Clone, Copy)]
pub struct Stream<'a> {
    pub slice: &'a [IndexedChar],
    pub cursor: Cursor,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// BASIC UTILS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a> Stream<'a> {
    pub fn at_column(&self, column_level: usize) -> bool {
        self.cursor.column == column_level
    }
}




//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// PARSER HELPERS - ALTERNATIVE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a> Stream<'a> {
    // pub fn two_options<T: StreamBinder>(self) {

    // }
    // pub fn alternatives<T: StreamBinder, const N: usize>(self, options: [(); N]) -> Output<'a, T::Ok<'a>, T::Err> {
    //     unimplemented!()
    // }
}

impl<'a> Stream<'a> {
    pub fn static_alternatives<'f, Ok, Err: Default>(
        self,
        options: &'f [&'f dyn Fn(Stream<'a>) -> Output<'a, Ok, Err>]
    ) -> Output<'a, Ok, Err> {
        for option in options {
            if let Some(io) = option(self.clone()).try_unwrap_success() {
                return Output::success(io);
            }
        }
        Output::failure(IO { context: self, value: Err::default() })
    }
}

