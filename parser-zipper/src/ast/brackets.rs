use crate::basics::IndexedChar;

use super::{LabelIdentifier, ParseAst};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum InSomeEnclosure {
    /// `{` and '}'
    CurlyBrackets(InCurlyBrackets),
    /// (`[` and `]`
    SquareBrackets(InSquareBrackets),
    /// (`(` and `)`
    RoundBrackets(InRoundBrackets),
    /// (`<` and `>`
    AngleBrackets(InAngleBrackets),
}

#[derive(Debug, Clone)]
pub struct LabeledEnclosure {
    pub identifier: LabelIdentifier,
    pub enclosure: InSomeEnclosure,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InCurlyBrackets {
    pub open: IndexedChar,
    pub content: Option<ParseAst>,
    pub close: IndexedChar,
}

impl InCurlyBrackets {
    pub const OPEN_CHAR: char = '{';
    pub const CLOSE_CHAR: char = '}';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InSquareBrackets {
    pub open: IndexedChar,
    pub content: Option<ParseAst>,
    pub close: IndexedChar,
}

impl InSquareBrackets {
    pub const OPEN_CHAR: char = '[';
    pub const CLOSE_CHAR: char = ']';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InRoundBrackets {
    pub open: IndexedChar,
    pub content: Option<ParseAst>,
    pub close: IndexedChar,
}

impl InRoundBrackets {
    pub const OPEN_CHAR: char = '(';
    pub const CLOSE_CHAR: char = ')';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InAngleBrackets {
    pub open: IndexedChar,
    pub content: Option<ParseAst>,
    pub close: IndexedChar,
}

impl InAngleBrackets {
    pub const OPEN_CHAR: char = '<';
    pub const CLOSE_CHAR: char = '>';
}
