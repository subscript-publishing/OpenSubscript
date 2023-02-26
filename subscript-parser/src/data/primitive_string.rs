use std::fmt::Display;

use itertools::Itertools;
use tree_formatter::ToDisplayTree;

use crate::{SrcChar, Range};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Default)]
pub struct SrcString {
    pub string: Vec<SrcChar>
}

impl SrcString {
    pub fn from_iter(string: impl IntoIterator<Item = SrcChar>) -> Self {
        let string = string.into_iter().collect_vec();
        Self { string }
    }
    pub fn singleton(char: SrcChar) -> Self { SrcString { string: vec![char] } }
    pub fn is_empty(&self) -> bool { self.string.is_empty() }
    pub fn append(&mut self, other: SrcString) {
        self.string.extend(other.string)
    }
    pub fn push(&mut self, item: SrcChar) {
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
    pub fn all_any_whitespace(&self) -> bool {
        if self.string.is_empty() {
            return false
        }
        self.string.iter().all(|x| x.is_any_whitespace())
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
    pub fn is_alphanumeric(&self) -> bool {
        self.all_satisfies(|x| x.is_alphanumeric())
    }
    pub fn is_valid_identfier(&self) -> bool {
        self.all_satisfies(|x| x.is_alphanumeric() || x == '-' || x == ':')
    }
    pub fn with(mut self, other: Self) -> Self {
        self.append(other);
        self
    }
}

impl Display for SrcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let view = self.string.iter().map(|x| x.char).collect::<String>();
        f.write_str(&view)
    }
}

impl ToDisplayTree for SrcString {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SrcString")
        } else {
            String::default()
        };
        let format = format!("{:?}{kind}", self.to_string());
        tree_formatter::DisplayTree::leaf(format)
    }
}