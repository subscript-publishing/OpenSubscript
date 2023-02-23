use std::fmt::{Display, Debug};

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
    /// NOTE:
    /// The lifetimes aren’t explicitly set to static but in practice it’ll
    /// pretty much always have to be static. 
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

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// PARSER HELPERS - SEQUENCING
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a> Stream<'a> {
    /// Sequentially executes the given parsers in sequence threading the
    /// returned stream throughout (i.e. back into the next parser), either
    /// returning the final stream (and the resulting product), or the
    /// original stream if any one parser fails. 
    /// 
    /// NOTE:
    /// The lifetimes aren’t explicitly set to static but in practice it’ll
    /// pretty much always have to be static (good luck otherwise). 
    pub fn static_homogeneous_sequence<'f, Ok, Err: Default>(
        self,
        sequence: &'f [&'f dyn Fn(Stream<'a>) -> Output<'a, Ok, Err>]
    ) -> Output<'a, Vec<Ok>, Err> {
        let origional = self.clone();
        let mut results: Vec<Ok> = Default::default();
        let mut context: Stream<'a> = self;
        for next in sequence {
            match next(context.clone()) {
                Output::Success(IO { context: next, value }) => {
                    results.push(value);
                    context = next;
                }
                Output::Failure(IO { context: _, value }) => {
                    return Output::failure(IO { context: origional, value });
                }
            }
        }
        Output::success(IO { context, value: results })
    }
    /// Sequentially executes two (2) parsers in sequence threading the
    /// returned stream throughout (i.e. back into the next parser),
    /// either returning the final stream (and the resulting product),
    /// or the original stream if any one parser fails. 
    /// 
    /// NOTE:
    /// The lifetimes aren’t explicitly set to static but in practice it’ll
    /// pretty much always have to be static (good luck otherwise). 
    pub fn static_twosome<'f, A, B, Err: Default>(
        self,
        (first, second): (
            &'f dyn Fn(Stream<'a>) -> Output<'a, A, Err>,
            &'f dyn Fn(Stream<'a>) -> Output<'a, B, Err>,
        ),
    ) -> Output<'a, (A, B), Err> {
        let origional = self.clone();
        match first(self) {
            Output::Success(IO { value: a, context }) => match second(context) {
                Output::Success(IO { value: b, context }) => Output::Success(IO { value: (a, b), context }),
                Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
            },
            Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
        }
    }
    /// Sequentially executes three (3) parsers in sequence threading the
    /// returned stream throughout (i.e. back into the next parser),
    /// either returning the final stream (and the resulting product),
    /// or the original stream if any one parser fails. 
    /// 
    /// NOTE:
    /// The lifetimes aren’t explicitly set to static but in practice it’ll
    /// pretty much always have to be static (good luck otherwise). 
    pub fn static_threesome<'f, A, B, C, Err: Default>(
        self,
        (first, second, third): (
            &'f dyn Fn(Stream<'a>) -> Output<'a, A, Err>,
            &'f dyn Fn(Stream<'a>) -> Output<'a, B, Err>,
            &'f dyn Fn(Stream<'a>) -> Output<'a, C, Err>,
        ),
    ) -> Output<'a, (A, B, C), Err> where A: Debug, B: Debug, C: Debug, Err: Debug {
        let origional = self.clone();
        match first(self) {
            Output::Success(IO { value: a, context }) => match second(context) {
                Output::Success(IO { value: b, context }) => match third(context) {
                    Output::Success(IO { value: c, context }) => {
                        Output::Success(IO { value: (a, b, c), context })
                    },
                    Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
                },
                Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
            },
            Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
        }
    }
    /// Sequentially executes four (4) parsers in sequence threading the
    /// returned stream throughout (i.e. back into the next parser),
    /// either returning the final stream (and the resulting product),
    /// or the original stream if any one parser fails. 
    /// 
    /// NOTE:
    /// The lifetimes aren’t explicitly set to static but in practice it’ll
    /// pretty much always have to be static. 
    pub fn static_foursome<'f, A, B, C, D, Err: Default>(
        self,
        (first, second, third, fourth): (
            &'f dyn Fn(Stream<'a>) -> Output<'a, A, Err>,
            &'f dyn Fn(Stream<'a>) -> Output<'a, B, Err>,
            &'f dyn Fn(Stream<'a>) -> Output<'a, C, Err>,
            &'f dyn Fn(Stream<'a>) -> Output<'a, D, Err>,
        ),
    ) -> Output<'a, (A, B, C, D), Err> {
        let origional = self.clone();
        match first(self) {
            Output::Success(IO { value: a, context }) => match second(context) {
                Output::Success(IO { value: b, context }) => match third(context) {
                    Output::Success(IO { value: c, context }) => match fourth(context) {
                        Output::Success(IO { value: d, context }) => {
                            Output::Success(IO { value: (a, b, c, d), context })
                        },
                        Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
                    },
                    Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
                },
                Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
            },
            Output::Failure(io) => Output::Failure(io.set_override_context(origional)),
        }
    }
}

