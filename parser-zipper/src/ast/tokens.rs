use std::fmt::Display;

use crate::basics::{IndexedChar, IndexedString};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagIdentifier {
    pub backslash: IndexedChar,
    pub identifier: IndexedString,
}

impl Display for TagIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = format!("{}{}", self.backslash, self.identifier);
        f.write_str(&format)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct LabelIdentifier {
    pub at_sign: IndexedChar,
    pub identifier1: IndexedString,
    pub identifier2: Option<IndexedString>,
}
impl Display for LabelIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(identifier2) = self.identifier2.as_ref() {
            let format = format!("{}{} {}", self.at_sign, self.identifier1, identifier2);
            return f.write_str(&format)
        }
        let format = format!("{}{}", self.at_sign, self.identifier1);
        f.write_str(&format)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct PipeTagIdentifier {
    pub pipe_char: IndexedChar,
    pub identifier1: IndexedString,
}
impl Display for PipeTagIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = format!("{}{}", self.pipe_char, self.identifier1);
        f.write_str(&format)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct SectionHeader {

}

