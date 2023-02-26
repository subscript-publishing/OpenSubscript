use std::fmt::Display;

use tree_formatter::{ToDisplayTree, DisplayTree};

use crate::{SrcChar, SrcString};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// BRACKETS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum SomeBracket {
    Open(SomeOpenBracket),
    Close(SomeCloseBracket),
}

#[derive(Debug, Clone, Copy)]
pub enum SomeOpenBracket {
    CurlyBracket(SrcChar),
    SquareBracket(SrcChar),
    RoundBracket(SrcChar),
    AngleBracket(SrcChar),
}

#[derive(Debug, Clone, Copy)]
pub enum SomeCloseBracket {
    CurlyBracket(SrcChar),
    SquareBracket(SrcChar),
    RoundBracket(SrcChar),
    AngleBracket(SrcChar),
}

impl SomeBracket {
    pub fn istring(self) -> SrcString {
        match self {
            Self::Open(x) => x.istring(),
            Self::Close(x) => x.istring(),
        }
    }
}
impl SomeOpenBracket {
    pub fn kind(&self) -> BracketType {
        match self {
            Self::CurlyBracket(_) => BracketType::CurlyBracket,
            Self::SquareBracket(_) => BracketType::SquareBracket,
            Self::RoundBracket(_) => BracketType::RoundBracket,
            Self::AngleBracket(_) => BracketType::AngleBracket,
        }
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::CurlyBracket(x) => SrcString::singleton(x),
            Self::SquareBracket(x) => SrcString::singleton(x),
            Self::RoundBracket(x) => SrcString::singleton(x),
            Self::AngleBracket(x) => SrcString::singleton(x),
        }
    }
}
impl SomeCloseBracket {
    pub fn kind(&self) -> BracketType {
        match self {
            Self::CurlyBracket(_) => BracketType::CurlyBracket,
            Self::SquareBracket(_) => BracketType::SquareBracket,
            Self::RoundBracket(_) => BracketType::RoundBracket,
            Self::AngleBracket(_) => BracketType::AngleBracket,
        }
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::CurlyBracket(x) => SrcString::singleton(x),
            Self::SquareBracket(x) => SrcString::singleton(x),
            Self::RoundBracket(x) => SrcString::singleton(x),
            Self::AngleBracket(x) => SrcString::singleton(x),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// BRACKETS - DEBUG
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Display for SomeOpenBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurlyBracket(x) => x.fmt(f),
            Self::SquareBracket(x) => x.fmt(f),
            Self::RoundBracket(x) => x.fmt(f),
            Self::AngleBracket(x) => x.fmt(f),
        }
    }
}
impl Display for SomeCloseBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurlyBracket(x) => x.fmt(f),
            Self::SquareBracket(x) => x.fmt(f),
            Self::RoundBracket(x) => x.fmt(f),
            Self::AngleBracket(x) => x.fmt(f),
        }
    }
}
impl Display for SomeBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close(x) => x.fmt(f),
            Self::Open(x) => x.fmt(f),
        }
    }
}
impl ToDisplayTree for SomeOpenBracket {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SomeOpenBracket")
        } else {
            String::default()
        };
        match self {
            Self::CurlyBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::SquareBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::RoundBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::AngleBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
        }
    }
}
impl ToDisplayTree for SomeCloseBracket {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SomeCloseBracket")
        } else {
            String::default()
        };
        match self {
            Self::CurlyBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::SquareBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::RoundBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::AngleBracket(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
        }
    }
}
impl ToDisplayTree for SomeBracket {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Open(x) => x.to_display_tree(),
            Self::Close(x) => x.to_display_tree(),
        }
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketType {
    /// Either `{` or `}`.
    CurlyBracket,
    /// Either `[` or `]`.
    SquareBracket,
    /// Either `(` or `)`.
    RoundBracket,
    /// Either `<` or `>`.
    AngleBracket,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// QUOTES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub enum SomeQuotationMark {
    Single(SrcChar),
    Double(SrcChar),
}

impl SomeQuotationMark {
    pub fn kind(&self) -> QuotationMarkType {
        match self {
            Self::Single(_) => QuotationMarkType::Single,
            Self::Double(_) => QuotationMarkType::Double,
        }
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::Single(x) => SrcString::singleton(x),
            Self::Double(x) => SrcString::singleton(x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotationMarkType {
    Single,
    Double,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// QUOTES - DEBUG
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Display for SomeQuotationMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(x) => x.fmt(f),
            Self::Double(x) => x.fmt(f),
        }
    }
}
impl ToDisplayTree for SomeQuotationMark {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Single(_) => DisplayTree::leaf(format!("QuotationMark → Single")),
            Self::Double(_) => DisplayTree::leaf(format!("QuotationMark → Double")),
        }
    }
}
