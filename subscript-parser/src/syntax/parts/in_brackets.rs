use crate::stream::IndexedChar;

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
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InCurlyBrackets<T> {
    const OPEN_CHAR: char = '{';
    const CLOSE_CHAR: char = '{';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InSquareBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InSquareBrackets<T> {
    const OPEN_CHAR: char = '[';
    const CLOSE_CHAR: char = ']';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InRoundBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InRoundBrackets<T> {
    const OPEN_CHAR: char = '(';
    const CLOSE_CHAR: char = ')';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InAngleBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InAngleBrackets<T> {
    const OPEN_CHAR: char = '<';
    const CLOSE_CHAR: char = '>';
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// PARSERS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct ParseInCurlyBrackets<T: InBracketEnclosureParser>(pub T);
pub struct ParseInSquareBrackets<T: InBracketEnclosureParser>(pub T);
pub struct ParseInRoundBrackets<T: InBracketEnclosureParser>(pub T);
pub struct ParseInAngleBrackets<T: InBracketEnclosureParser>(pub T);

pub trait InBracketEnclosureParser {}
