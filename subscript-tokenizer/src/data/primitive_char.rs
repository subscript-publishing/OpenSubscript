use std::fmt::{Display, Write};

use itertools::Itertools;

use crate::Position;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct SrcChar {
    pub pos: Position,
    pub char: char,
}

impl SrcChar {
    pub fn is_any_whitespace(&self) -> bool {
        self.char.is_whitespace()
    }
    pub fn is_valid_close_token(self, close: SrcChar) -> bool {
        match (self.char, close.char) {
            ('{', '}') => true,
            ('[', ']') => true,
            ('(', ')') => true,
            ('<', '>') => true,
            _ => false,
        }
    }
}

impl Display for SrcChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.char)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl SrcChar {
    pub fn to_indexed_chars(source_code: &str) -> Vec<SrcChar> {
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
                SrcChar{
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
