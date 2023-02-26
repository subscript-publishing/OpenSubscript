use std::fmt::{Display, Write};
use tree_formatter::{DisplayTree, ToDisplayTree};
use super::{IndexedChar, IndexedString, Position, Range};

#[derive(Debug, Clone)]
pub enum Bucket {
    Backslash1(IndexedChar),
    Backslash2(IndexedChar, IndexedChar),
    Pipe1(IndexedChar),
    Pipe2(IndexedChar, IndexedChar),
    OpenBracket(IndexedChar),
    CloseBracket(IndexedChar),
    PlainText(IndexedString),
    Whitespace(IndexedString),
    TagIdentifier {
        backslash: IndexedChar,
        identifier: IndexedString,
    },
    PipeTagIdentifier {
        pipe: IndexedChar,
        identifier: IndexedString,
    },
    LabelIdentifier1 {
        at_sign: IndexedChar,
        identifier: IndexedString,
    },
    LabelIdentifier2 {
        at_sign: IndexedChar,
        identifier1: IndexedString,
        whitespace: IndexedString,
        identifier2: IndexedString,
    },
    SectionHeader {
        open_p1: IndexedChar,
        open_p2: IndexedChar,
        contents: IndexedString,
        close_p1: IndexedChar,
        close_p2: IndexedChar,
    },
}

impl Bucket {
    pub fn is_any_open(&self) -> bool {
        match self {
            Self::OpenBracket(_) => true,
            _ => false,
        }
    }
    pub fn is_any_close(&self) -> bool {
        match self {
            Self::CloseBracket(_) => true,
            _ => false,
        }
    }
    pub fn is_plain_text(&self) -> bool {
        match self {
            Self::PlainText(_) => true,
            _ => false
        }
    }
    pub fn into_plain_text(self) -> Option<IndexedString> {
        match self {
            Self::PlainText(x) => Some(x),
            _ => None
        }
    }
    pub fn is_whitespace(&self) -> bool {
        match self {
            Self::Whitespace(_) => true,
            _ => false
        }
    }
    pub fn into_whitespace(self) -> Option<IndexedString> {
        match self {
            Self::Whitespace(x) => Some(x),
            _ => None
        }
    }
    pub fn is_tag_identifier(&self) -> bool {
        match self {
            Self::TagIdentifier{..} => true,
            _ => false
        }
    }
    pub fn is_pipe_tag_identifier(&self) -> bool {
        match self {
            Self::PipeTagIdentifier{..} => true,
            _ => false
        }
    }
    pub fn is_label_identifier1(&self) -> bool {
        match self {
            Self::LabelIdentifier1{..} => true,
            _ => false
        }
    }
    pub fn is_label_identifier2(&self) -> bool {
        match self {
            Self::LabelIdentifier2{..} => true,
            _ => false
        }
    }
    pub fn is_open_bracket(&self) -> bool {
        match self {
            Self::OpenBracket(_) => true,
            _ => false
        }
    }
    pub fn is_close_bracket(&self) -> bool {
        match self {
            Self::CloseBracket(_) => true,
            _ => false
        }
    }
    pub fn range_of(&self) -> Range {
        match self {
            Self::Backslash1(x) => { x.pos.to_singleton_range() }
            Self::Backslash2(x, y) => { x.pos.range_to(y.pos) }
            Self::Pipe1(x) => { x.pos.to_singleton_range() }
            Self::Pipe2(x, y) => { x.pos.range_to(y.pos) }
            Self::TagIdentifier { backslash, identifier } => {
                backslash.pos.range_to(identifier.range_of().position)
            }
            Self::LabelIdentifier1 { at_sign, identifier } => {
                at_sign.pos.range_to(identifier.range_of().position)
            }
            Self::LabelIdentifier2 { at_sign, identifier1, whitespace, identifier2 } => {
                at_sign.pos.range_to(identifier2.range_of().position)
            }
            Self::PipeTagIdentifier { pipe, identifier } => {
                pipe.pos.range_to(identifier.range_of().position)
            }
            Self::SectionHeader { open_p1, open_p2, contents, close_p1, close_p2 } => {
                open_p1.pos.range_to(close_p2.pos)
            }
            Self::PlainText(x) => {
                x.range_of()
            }
            Self::Whitespace(x) => {
                x.range_of()
            }
            Self::OpenBracket(x) => {
                x.pos.to_singleton_range()
            }
            Self::CloseBracket(x) => {
                x.pos.to_singleton_range()
            }
        }
    }
}

impl Bucket {
    pub fn unwrap_open_bracket(self) -> Result<IndexedChar, Bucket> {
        match self {
            Self::OpenBracket(x) => Ok(x),
            x => Err(x),
        }
    }
    pub fn unwrap_close_bracket(self) -> Result<IndexedChar, Bucket> {
        match self {
            Self::CloseBracket(x) => Ok(x),
            x => Err(x),
        }
    }
}

impl Bucket {
    pub fn new(item: IndexedChar) -> Self {
        match item.char {
            x if x.is_whitespace() => Self::Whitespace(IndexedString::singleton(item)),
            // '\\' => Self::TagIdentifier { backslash: item, identifier: Default::default() },
            '\\' => Self::Backslash1(item),
            '|' => Self::Pipe1(item),
            '{' => Self::OpenBracket(item),
            '[' => Self::OpenBracket(item),
            '(' => Self::OpenBracket(item),
            '<' => Self::OpenBracket(item),
            '}' => Self::CloseBracket(item),
            ']' => Self::CloseBracket(item),
            ')' => Self::CloseBracket(item),
            '>' => Self::CloseBracket(item),
            '@' => Self::LabelIdentifier1 { at_sign: item, identifier: Default::default() },
            _ => Self::PlainText(IndexedString::singleton(item))
        }
    }
    pub fn coalesce(self, other: Self) -> Result<Bucket, (Bucket, Bucket)> {
        match (self, other) {
            (Self::Backslash1(x), Self::Backslash1(y)) => {
                Ok(Self::Backslash2(x, y))
            }
            (Self::Pipe1(x), Self::Pipe1(y)) => {
                Ok(Self::Pipe2(x, y))
            }
            (Self::Backslash1(x), Self::PlainText(y)) => {
                Ok(Self::TagIdentifier { backslash: x, identifier: y })
            }
            (Self::Pipe1(x), Self::PlainText(y)) => {
                Ok(Self::PipeTagIdentifier { pipe: x, identifier: y })
            }
            (Self::TagIdentifier { backslash, mut identifier }, Self::PlainText(x)) => {
                identifier.append(x);
                Ok(Self::TagIdentifier { backslash, identifier })
            }
            (Self::PipeTagIdentifier { pipe, mut identifier }, Self::PlainText(x)) => {
                identifier.append(x);
                Ok(Self::PipeTagIdentifier { pipe, identifier })
            }
            (Self::LabelIdentifier1 { at_sign, mut identifier }, Self::PlainText(x)) => {
                identifier.append(x);
                Ok(Self::LabelIdentifier1 { at_sign, identifier })
            }
            (Self::PlainText(mut x), Self::PlainText(y)) => {
                x.append(y);
                Ok(Self::PlainText(x))
            }
            (Self::Whitespace(mut x), Self::Whitespace(y)) => {
                x.append(y);
                Ok(Self::Whitespace(x))
            }
            (l, r) => {
                Err((l, r))
            }
        }
    }
    pub fn coalesce2(self, first: Self, second: Self) -> Result<Bucket, (Bucket, Bucket, Bucket)> {
        match (self, first, second) {
            (Bucket::LabelIdentifier1 { at_sign, identifier }, Bucket::Whitespace(ws), Bucket::PlainText(identifier2)) => {
                Ok(Bucket::LabelIdentifier2 { at_sign, identifier1: identifier, whitespace: ws, identifier2 })
            }
            (l, c, r) => Err((l, c, r))
        }
    }
    pub fn into_string(self) -> IndexedString {
        match self {
            Self::Backslash1(x) => {
                IndexedString{string: vec![x]}
            }
            Self::Backslash2(x, y) => {
                IndexedString{string: vec![x, y]}
            }
            Self::Pipe1(x) => {
                IndexedString{string: vec![x]}
            }
            Self::Pipe2(x, y) => {
                IndexedString{string: vec![x, y]}
            }
            Self::TagIdentifier { backslash, identifier } => {
                let mut xs = IndexedString::singleton(backslash);
                xs.append(identifier);
                xs
            }
            Self::LabelIdentifier1 { at_sign, identifier } => {
                let mut xs = IndexedString::singleton(at_sign);
                xs.append(identifier);
                xs
            }
            Self::LabelIdentifier2 { at_sign, identifier1, whitespace, identifier2 } => {
                let mut xs = IndexedString::singleton(at_sign);
                xs.append(identifier1);
                xs.append(whitespace);
                xs.append(identifier2);
                xs
            }
            Self::SectionHeader { open_p1, open_p2, contents, close_p1, close_p2 } => {
                let mut xs = IndexedString::singleton(open_p1);
                xs.push(open_p2);
                xs.append(contents);
                xs.push(close_p1);
                xs.push(close_p2);
                xs
            }
            Self::PipeTagIdentifier { pipe, identifier } => {
                let mut xs = IndexedString::singleton(pipe);
                xs.append(identifier);
                xs
            }
            Self::PlainText(string) => {
                string
            }
            Self::Whitespace(string) => {
                string
            }
            Self::OpenBracket(item) => {
                IndexedString::singleton(item)
            }
            Self::CloseBracket(item) => {
                IndexedString::singleton(item)
            }
        }
    }
}

impl Display for Bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.clone().into_string();
        if self.is_plain_text() || self.is_whitespace() {
            let string = format!("{string}");
            let format = format!("{string:?}");
            f.write_str(&format)
        } else {
            let format = format!("`{string}`");
            f.write_str(&format)
        }
    }
}

impl ToDisplayTree for Bucket {
    fn to_display_tree(&self) -> DisplayTree {
        let format = format!("{}", self);
        DisplayTree::leaf(format)
    }
}
