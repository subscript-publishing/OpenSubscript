use crate::{
    binders::StreamBinder,
    character::UnconsSpec,
    output::Output,
    stream::{IndexedChar, Stream},
    syntax::RootAst,
};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum InSomeEnclosure<T> {
    /// `{` and '}'
    CurlyBrackets(InCurlyBrackets<T>),
    /// (`[` and `]`
    SquareBrackets(InSquareBrackets<T>),
    /// (`(` and `)`
    RoundBrackets(InRoundBrackets<T>),
    /// (`<` and `>`
    AngleBrackets(InAngleBrackets<T>),
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InCurlyBrackets<T> {
    pub open: IndexedChar,
    pub content: Option<T>,
    pub close: IndexedChar,
}

impl<T> InCurlyBrackets<T> {
    pub const OPEN_CHAR: char = '{';
    pub const CLOSE_CHAR: char = '}';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InSquareBrackets<T> {
    pub open: IndexedChar,
    pub content: Option<T>,
    pub close: IndexedChar,
}

impl<T> InSquareBrackets<T> {
    pub const OPEN_CHAR: char = '[';
    pub const CLOSE_CHAR: char = ']';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InRoundBrackets<T> {
    pub open: IndexedChar,
    pub content: Option<T>,
    pub close: IndexedChar,
}

impl<T> InRoundBrackets<T> {
    pub const OPEN_CHAR: char = '(';
    pub const CLOSE_CHAR: char = ')';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InAngleBrackets<T> {
    pub open: IndexedChar,
    pub content: Option<T>,
    pub close: IndexedChar,
}

impl<T> InAngleBrackets<T> {
    pub const OPEN_CHAR: char = '<';
    pub const CLOSE_CHAR: char = '>';
}
