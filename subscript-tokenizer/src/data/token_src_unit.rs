use std::fmt::Display;
use tree_formatter::{DisplayTree, ToDisplayTree};
use crate::{SomeSectionHeader, LabelIdentifier2, PipeTagIdentifier, LabelIdentifier1, TagIdentifier, SrcChar2};

use super::{SrcChar, SrcString, Range};



#[derive(Debug, Clone)]
pub enum SrcUnit {
    AtSign(SrcChar),
    Backslash(SrcChar),
    Pipe(SrcChar),
    Pipe2(SrcChar2),
    Backslash2(SrcChar2),
    OpenBracket(SrcChar),
    CloseBracket(SrcChar),
    PlainText(SrcString),
    AnyWhitespace(SrcString),
    TagIdentifier(TagIdentifier),
    PipeTagIdentifier(PipeTagIdentifier),
    LabelIdentifier1(LabelIdentifier1),
    LabelIdentifier2(LabelIdentifier2),
    SectionHeader(SomeSectionHeader),
}

impl SrcUnit {
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
    pub fn into_plain_text(self) -> Option<SrcString> {
        match self {
            Self::PlainText(x) => Some(x),
            _ => None
        }
    }
    pub fn is_any_whitespace(&self) -> bool {
        match self {
            Self::AnyWhitespace(_) => true,
            _ => false
        }
    }
    pub fn into_any_whitespace(self) -> Option<SrcString> {
        match self {
            Self::AnyWhitespace(x) => Some(x),
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
            Self::Backslash(x) => { x.pos.to_singleton_range() }
            Self::AtSign(x) => { x.pos.to_singleton_range() }
            Self::Backslash2(x) => { x.range_of() }
            Self::Pipe(x) => { x.pos.to_singleton_range() }
            Self::Pipe2(x) => { x.range_of() }
            Self::TagIdentifier(x) => x.range_of(),
            Self::LabelIdentifier1(x) => x.range_of(),
            Self::LabelIdentifier2(x) => x.range_of(),
            Self::PipeTagIdentifier(x) => x.range_of(),
            Self::SectionHeader(x) => x.range_of(),
            Self::PlainText(x) => x.range_of(),
            Self::AnyWhitespace(x) => x.range_of(),
            Self::OpenBracket(x) => x.pos.to_singleton_range(),
            Self::CloseBracket(x) => x.pos.to_singleton_range(),
        }
    }
}

impl SrcUnit {
    pub fn unwrap_open_bracket(self) -> Result<SrcChar, SrcUnit> {
        match self {
            Self::OpenBracket(x) => Ok(x),
            x => Err(x),
        }
    }
    pub fn unwrap_close_bracket(self) -> Result<SrcChar, SrcUnit> {
        match self {
            Self::CloseBracket(x) => Ok(x),
            x => Err(x),
        }
    }
}

impl SrcUnit {
    pub fn new(item: SrcChar) -> Self {
        match item.char {
            x if x.is_whitespace() => Self::AnyWhitespace(SrcString::singleton(item)),
            '\\' => Self::Backslash(item),
            '|' => Self::Pipe(item),
            '{' => Self::OpenBracket(item),
            '[' => Self::OpenBracket(item),
            '(' => Self::OpenBracket(item),
            '<' => Self::OpenBracket(item),
            '}' => Self::CloseBracket(item),
            ']' => Self::CloseBracket(item),
            ')' => Self::CloseBracket(item),
            '>' => Self::CloseBracket(item),
            '@' => Self::AtSign(item),
            _ => Self::PlainText(SrcString::singleton(item))
        }
    }
    pub fn coalesce(self, other: Self) -> Result<SrcUnit, (SrcUnit, SrcUnit)> {
        match (self, other) {
            (Self::Backslash(x), Self::Backslash(y)) => {
                Ok(Self::Backslash2(SrcChar2::new(x, y)))
            }
            (Self::Pipe(x), Self::Pipe(y)) => {
                Ok(Self::Pipe2(SrcChar2::new(x, y)))
            }
            (Self::Backslash(x), Self::PlainText(y)) => {
                let new = TagIdentifier::new(x, y);
                Ok(Self::TagIdentifier(new))
            }
            (Self::Pipe(x), Self::PlainText(y)) => {
                let new = PipeTagIdentifier{pipe: x, identifier: y};
                Ok(Self::PipeTagIdentifier(new))
            }
            (Self::AtSign(x), Self::PlainText(y)) => {
                let new = LabelIdentifier1{at_sign: x, identifier: y};
                Ok(Self::LabelIdentifier1(new))
            }
            (Self::TagIdentifier(mut x), Self::PlainText(y)) if y.is_valid_identfier() => {
                x.append_to_identifier(y);
                Ok(Self::TagIdentifier(x))
            }
            (Self::PipeTagIdentifier(mut x), Self::PlainText(y)) if y.is_valid_identfier() => {
                x.append_to_identifier(y);
                Ok(Self::PipeTagIdentifier(x))
            }
            (Self::LabelIdentifier1(mut x), Self::PlainText(y)) if y.is_valid_identfier() => {
                x.append_to_identifier(y);
                Ok(Self::LabelIdentifier1(x))
            }
            (Self::PlainText(mut x), Self::PlainText(y)) => {
                x.append(y);
                Ok(Self::PlainText(x))
            }
            (Self::AnyWhitespace(mut x), Self::AnyWhitespace(y)) => {
                x.append(y);
                Ok(Self::AnyWhitespace(x))
            }
            (l, r) => {
                Err((l, r))
            }
        }
    }
    pub fn coalesce2(self, first: Self, second: Self) -> Result<SrcUnit, (SrcUnit, SrcUnit, SrcUnit)> {
        match (self, first, second) {
            (
                SrcUnit::LabelIdentifier1(LabelIdentifier1{ at_sign, identifier }),
                SrcUnit::AnyWhitespace(ws),
                SrcUnit::PlainText(identifier2)
            ) => {
                let new = LabelIdentifier2 {
                    at_sign,
                    identifier1: identifier,
                    whitespace: ws,
                    identifier2,
                };
                Ok(SrcUnit::LabelIdentifier2(new))
            }
            (l, c, r) => Err((l, c, r))
        }
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::AtSign(x) => SrcString::singleton(x),
            Self::Backslash(x) => SrcString::singleton(x),
            Self::Pipe(x) => SrcString::singleton(x),
            Self::Pipe2(x) => x.istring(),
            Self::Backslash2(x) => x.istring(),
            Self::TagIdentifier(x) => x.istring(),
            Self::LabelIdentifier1(x) => x.istring(),
            Self::LabelIdentifier2(x) => x.istring(),
            Self::SectionHeader(x) => x.istring(),
            Self::PipeTagIdentifier(x) => x.istring(),
            Self::PlainText(string) => string,
            Self::AnyWhitespace(string) => string,
            Self::OpenBracket(item) => SrcString::singleton(item),
            Self::CloseBracket(item) => SrcString::singleton(item),
        }
    }
}

impl Display for SrcUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.clone().istring();
        if self.is_plain_text() || self.is_any_whitespace() {
            let string = format!("{string}");
            let format = format!("{string:?}");
            f.write_str(&format)
        } else {
            let format = format!("`{string}`");
            f.write_str(&format)
        }
    }
}

impl ToDisplayTree for SrcUnit {
    fn to_display_tree(&self) -> DisplayTree {
        // let format = format!("{}", self);
        // DisplayTree::leaf(format)
        let format_char = |label: &str, x: SrcChar| -> DisplayTree {
            let format = format!("{label}('{x}')");
            DisplayTree::leaf(format)
        };
        let format_char2 = |label: &str, x: SrcChar2| -> DisplayTree {
            let format = format!("{label}('{x}')");
            DisplayTree::leaf(format)
        };
        let format_string = |label: &str, x: &SrcString| -> DisplayTree {
            let format = format!("{label}({:?})", x.to_string());
            DisplayTree::leaf(format)
        };
        match self {
            Self::AtSign(x) => format_char("AtSign", *x),
            Self::Backslash(x) => format_char("Backslash", *x),
            Self::Pipe(x) => format_char("Pipe", *x),
            Self::Pipe2(x) => format_char2("Pipe2", *x),
            Self::Backslash2(x) => format_char2("Backslash2", *x),
            Self::OpenBracket(x) => format_char("OpenBracket", *x),
            Self::CloseBracket(x) => format_char("CloseBracket", *x),
            Self::PlainText(x) => format_string("PlainText", x),
            Self::AnyWhitespace(x) => format_string("AnyWhitespace", x),
            Self::TagIdentifier(x) => x.to_display_tree(),
            Self::PipeTagIdentifier(x) => x.to_display_tree(),
            Self::LabelIdentifier1(x) => x.to_display_tree(),
            Self::LabelIdentifier2(x) => x.to_display_tree(),
            Self::SectionHeader(x) => x.to_display_tree(),
        }
    }
}
