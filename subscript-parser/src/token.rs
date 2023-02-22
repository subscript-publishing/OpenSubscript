use std::fmt::{Debug, Display};

use crate::stream::{IndexedChar, Stream};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub index: usize,
    pub length: usize,
}

impl Range {
    pub fn len(self) -> usize {
        self.length
    }
    pub fn valid_delta(self, delta: usize) -> bool {
        delta <= self.len()
    }
    pub fn seek_forward(self, delta: usize) -> Option<Range> {
        if !self.valid_delta(delta) {
            return None
        }
        Some(Self { index: self.index + delta, length: self.length - delta })
    }
    pub fn slice_range(self) -> std::ops::Range<usize> {
        self.index..self.index+self.length
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct TokenView<'a> {
    pub slice: &'a [IndexedChar],
    pub range: Range,
}

impl<'a> TokenView<'a> {
    pub fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }
    pub fn to_string(&'a self) -> String {
        self.slice.into_iter().map(|x| x.char).collect::<String>()
    }
}
impl<'a> Stream<'a> {
    pub fn to_token_view(self) -> TokenView<'a> {
        let range = Range{index: self.cursor.index, length: self.slice.len()};
        TokenView { slice: self.slice, range }
    }
}
impl<'a> Display for TokenView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.to_string();
        f.write_str(&string)
    }
}