use std::fmt::{Display, Write};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn range_to(self, other: Position) -> Range {
        Range::from(self, other)
    }
    pub fn to_singleton_range(self) -> Range {
        Range::single(self)
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub position: Position,
    pub length: usize,
}

impl Range {
    pub fn single(position: Position) -> Self { Self { position: position, length: 0 }}
    pub fn from(start: Position, end: Position) -> Self {
        let length = end.index - start.index;
        Self { position: start, length }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct IndexedChar {
    pub pos: Position,
    pub char: char,
}

impl IndexedChar {
    pub fn is_whitespace(&self) -> bool {
        self.char.is_whitespace()
    }
    pub fn is_valid_close_token(self, close: IndexedChar) -> bool {
        match (self.char, close.char) {
            ('{', '}') => true,
            ('[', ']') => true,
            ('(', ')') => true,
            ('<', '>') => true,
            _ => false,
        }
    }
}

impl Display for IndexedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.char)
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Default)]
pub struct IndexedString {
    pub string: Vec<IndexedChar>
}

impl IndexedString {
    pub fn singleton(char: IndexedChar) -> Self { IndexedString { string: vec![char] } }
    pub fn is_empty(&self) -> bool { self.string.is_empty() }
    pub fn append(&mut self, other: IndexedString) {
        self.string.extend(other.string)
    }
    pub fn push(&mut self, item: IndexedChar) {
        self.string.push(item);
    }
    pub fn starts_with(&self, char: char) -> bool {
        self.string.first().map(|x| x.char == char).unwrap_or(false)
    }
    pub fn all_satisfies(&self, f: impl Fn(char) -> bool) -> bool {
        self.string.iter().all(|x| f(x.char))
    }
    pub fn any_satisfies(&self, f: impl Fn(char) -> bool) -> bool {
        self.string.iter().all(|x| f(x.char))
    }
    pub fn all_whitespace(&self) -> bool {
        if self.string.is_empty() {
            return false
        }
        self.string.iter().all(|x| x.is_whitespace())
    }
    pub fn contains(&self, char: char) -> bool {
        if self.string.is_empty() {
            return false
        }
        self.string.iter().any(|x| x.char == char)
    }
    pub fn range_of(&self) -> Range {
        if self.is_empty() {

        }
        if self.string.len() == 1 {

        }
        let start = self.string.first().unwrap();
        let end = self.string.last().unwrap();
        start.pos.range_to(end.pos)
    }
}

impl Display for IndexedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let view = self.string.iter().map(|x| x.char).collect::<String>();
        f.write_str(&view)
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub mod char_utils {
    use itertools::Itertools;

    use super::IndexedChar;

    pub fn to_indexed_chars(source_code: &str) -> Vec<IndexedChar> {
        let mut line = 0;
        let mut column = 0;
        source_code
            .chars()
            .enumerate()
            .map(|(ix, char)| {
                match char {
                    '\n' => {
                        line = line + 1;
                        column = 0;
                    }
                    _ => {
                        column = 0;
                    }
                }
                IndexedChar{
                    pos: super::Position {
                        index: ix,
                        line,
                        column,
                    },
                    char,
                }
            })
            .collect_vec()
    }
}
