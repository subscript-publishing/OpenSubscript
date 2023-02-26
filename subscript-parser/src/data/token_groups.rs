use std::fmt::Display;

use tree_formatter::{ToDisplayTree, DisplayTree};

use crate::{SrcChar, SrcString, SomeSectionHeader, SomeBracket, SomeQuotationMark, SrcChar2, Space, zipper::{ZipperTryCoalesce, ZipperTryCoalesce2}, Newline};

// pub type AnyWhitespace = SrcString;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum SomePrimitive {
    Bracket(SomeBracket),
    QuotationMark(SomeQuotationMark),
    Backslash(SrcChar),
    Pipe(SrcChar),
    AtSign(SrcChar),
    Backslash2(SrcChar2),
    Pipe2(SrcChar2),
}

impl SomePrimitive {
    pub fn istring(self) -> SrcString {
        match self {
            Self::Bracket(x) => x.istring(),
            Self::QuotationMark(x) => x.istring(),
            Self::Backslash(x) => SrcString::singleton(x),
            Self::Pipe(x) => SrcString::singleton(x),
            Self::AtSign(x) => SrcString::singleton(x),
            Self::Backslash2(x) => x.istring(),
            Self::Pipe2(x) => x.istring(),
        }
    }
}

impl Display for SomePrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bracket(x) => x.fmt(f),
            Self::QuotationMark(x) => x.fmt(f),
            Self::Backslash(x) => x.fmt(f),
            Self::Pipe(x) => x.fmt(f),
            Self::AtSign(x) => x.fmt(f),
            Self::Backslash2(x) => x.fmt(f),
            Self::Pipe2(x) => x.fmt(f),
        }
    }
}
impl ToDisplayTree for SomePrimitive {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ Token")
        } else {
            String::default()
        };
        match self {
            Self::Bracket(x) => x.to_display_tree(),
            Self::QuotationMark(x) => x.to_display_tree(),
            Self::Backslash(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::Pipe(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::AtSign(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::Backslash2(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
            Self::Pipe2(x) => DisplayTree::leaf(format!("`{}`{kind}", x)),
        }
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum SomeIdentifier1 {
    Backslash {
        begin: SrcChar,
        identifier: SrcString,
    },
    Pipe {
        begin: SrcChar,
        identifier: SrcString,
    },
    AtSign {
        begin: SrcChar,
        identifier: SrcString,
    },
}

impl SomeIdentifier1 {
    pub fn backslash(begin: SrcChar, id: SrcString) -> Self {
        Self::Backslash { begin, identifier: id }
    }
    pub fn pipe(begin: SrcChar, id: SrcString) -> Self {
        Self::Pipe { begin, identifier: id }
    }
    pub fn at_sign(begin: SrcChar, id: SrcString) -> Self {
        Self::AtSign { begin, identifier: id }
    }
    pub fn extend_id(&mut self, trailing: SrcString) {
        match self {
            Self::Backslash { identifier, .. } => identifier.append(trailing),
            Self::Pipe { identifier, .. } => identifier.append(trailing),
            Self::AtSign { identifier, .. } => identifier.append(trailing),
        }
    }
    pub fn with_trailing(mut self, trailing: SrcString) -> Self {
        self.extend_id(trailing);
        self
    }
    pub fn try_promote(self, identifier2: SrcString) -> Result<SomeIdentifier2, (SomeIdentifier1, SrcString)> {
        match self {
            SomeIdentifier1::AtSign { begin, identifier: identifier1 } => {
                let new = SomeIdentifier2::AtSign { begin, identifier1, identifier2 };
                Ok(new)
            }
            x => Err((x, identifier2))
        }
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::AtSign { begin, identifier } => {
                let mut xs = SrcString::singleton(begin);
                xs.append(identifier);
                xs
            }
            Self::Backslash { begin, identifier } => {
                let mut xs = SrcString::singleton(begin);
                xs.append(identifier);
                xs
            }
            Self::Pipe { begin, identifier } => {
                let mut xs = SrcString::singleton(begin);
                xs.append(identifier);
                xs
            }
        }
    }
}
impl ToDisplayTree for SomeIdentifier1 {
    fn to_display_tree(&self) -> DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SomeIdentifier1")
        } else {
            String::default()
        };
        match self {
            Self::Backslash { begin, identifier } => {
                let label = format!("`{begin}{identifier}`{kind}");
                DisplayTree::leaf(label)
            }
            Self::Pipe { begin, identifier } => {
                let label = format!("`{begin}{identifier}`{kind}");
                DisplayTree::leaf(label)
            }
            Self::AtSign { begin, identifier } => {
                let label = format!("`{begin}{identifier}`{kind}");
                DisplayTree::leaf(label)
            }
        }
    }
}
impl ToDisplayTree for SomeIdentifier2 {
    fn to_display_tree(&self) -> DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SomeIdentifier2")
        } else {
            String::default()
        };
        match self {
            Self::AtSign { begin, identifier1, identifier2 } => {
                let label = format!("`{begin}{identifier1} {identifier2}`{kind}");
                DisplayTree::leaf(label)
            }
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum SomeIdentifier2 {
    AtSign {
        begin: SrcChar,
        identifier1: SrcString,
        identifier2: SrcString,
    },
}

impl SomeIdentifier2 {
    pub fn try_promote(id: SomeIdentifier1, identifier2: SrcString) -> Result<Self, (SomeIdentifier1, SrcString)> {
        match id {
            SomeIdentifier1::AtSign { begin, identifier: identifier1 } => {
                let new = SomeIdentifier2::AtSign { begin, identifier1, identifier2 };
                Ok(new)
            }
            x => Err((x, identifier2)),
        }
    }
    pub fn extend_second_id(&mut self, trailing: SrcString) {
        match self {
            Self::AtSign { identifier2, .. } => {
                identifier2.append(trailing)
            }
        }
    }
    pub fn with_trailing(mut self, trailing: SrcString) -> Self {
        self.extend_second_id(trailing);
        self
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::AtSign { begin, identifier1, identifier2 } => {
                let mut xs = SrcString::singleton(begin);
                xs.append(identifier1);
                xs.append(identifier2);
                xs
            }
        }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum SrcGroup {
    SomePrimitive(SomePrimitive),
    Newline(Newline),
    Space(Space),
    Id1(SomeIdentifier1),
    Id2(SomeIdentifier2),
    SectionHeader(SomeSectionHeader),
    PlainText(SrcString),
}

impl SrcGroup {
    pub fn from_src_char(item: SrcChar) -> Self {
        match item.char {
            ' ' => Self::Space(Space::new(SrcString::singleton(item))),
            '\n' => Self::Newline(Newline::new(SrcString::singleton(item))),
            '\\' => Self::SomePrimitive(SomePrimitive::Backslash(item)),
            '|' => Self::SomePrimitive(SomePrimitive::Pipe(item)),
            '@' => Self::SomePrimitive(SomePrimitive::AtSign(item)),
            '{' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Open(crate::SomeOpenBracket::SquareBracket(item)))),
            '[' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Open(crate::SomeOpenBracket::SquareBracket(item)))),
            '(' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Open(crate::SomeOpenBracket::SquareBracket(item)))),
            '<' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Open(crate::SomeOpenBracket::SquareBracket(item)))),
            '}' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Close(crate::SomeCloseBracket::SquareBracket(item)))),
            ']' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Close(crate::SomeCloseBracket::SquareBracket(item)))),
            ')' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Close(crate::SomeCloseBracket::SquareBracket(item)))),
            '>' => Self::SomePrimitive(SomePrimitive::Bracket(SomeBracket::Close(crate::SomeCloseBracket::SquareBracket(item)))),
            '\t' => unimplemented!("TODO: Tab Char"),
            _ => Self::PlainText(SrcString::singleton(item))
        }
    }
    pub fn primitive(x: SomePrimitive) -> Self {
        Self::SomePrimitive(x)
    }
    pub fn coalesce(self, next: Self) -> Result<Self, (Self, Self)> {
        self.try_coalesce_focus(next)
    }
    pub fn map_prim_ref<T>(&self, f: impl FnOnce(&SomePrimitive) -> T) -> Option<T> {
        match self {
            Self::SomePrimitive(x) => Some(f(x)),
            _ => None,
        }
    }
    pub fn is_some_newline(&self) -> bool {
        match self {Self::Newline(_) => true, _ => false}
    }
    pub fn is_some_space(&self) -> bool {
        match self {Self::Space(_) => true, _ => false}
    }
    pub fn istring(self) -> SrcString {
        match self {
            Self::SomePrimitive(x) => x.istring(),
            Self::Newline(x) => x.istring(),
            Self::Space(x) => x.istring(),
            Self::Id1(x) => x.istring(),
            Self::Id2(x) => x.istring(),
            Self::SectionHeader(x) => x.istring(),
            Self::PlainText(x) => x,
        }
    }
}

impl ToDisplayTree for SrcGroup {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::SomePrimitive(x) => x.to_display_tree(),
            Self::Newline(x) => x.to_display_tree(),
            Self::Space(x) => x.to_display_tree(),
            Self::Id1(x) => x.to_display_tree(),
            Self::Id2(x) => x.to_display_tree(),
            Self::SectionHeader(x) => x.to_display_tree(),
            Self::PlainText(x) => x.to_display_tree(),
        }
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// COALESCE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl ZipperTryCoalesce for SrcGroup {
    fn try_coalesce_focus(self, next: Self) -> Result<Self, (Self, Self)> {
        match (self, next) {
            (Self::Id1(x), Self::PlainText(y)) if y.is_valid_identfier() => Ok(SrcGroup::Id1(x.with_trailing(y))),
            (Self::Id2(x), Self::PlainText(y)) if y.is_valid_identfier() => Ok(SrcGroup::Id2(x.with_trailing(y))),
            (Self::Newline(x), Self::Newline(y)) => Ok(Self::Newline(x.with(y))),
            (Self::Space(x), Self::Space(y)) => Ok(Self::Space(x.with(y))),
            (Self::PlainText(x), Self::PlainText(y)) => Ok(Self::PlainText(x.with(y))),
            (Self::SomePrimitive(x), Self::SomePrimitive(y)) => {
                x   .try_coalesce(y)
                    .map(|x| SrcGroup::SomePrimitive(x))
                    .map_err(|(x, y)| (Self::SomePrimitive(x), Self::SomePrimitive(y)))
            }
            (Self::SomePrimitive(x), y) => {
                x   .try_coalesce_with(y)
                    .map_err(|(x, y)| (Self::SomePrimitive(x), y))
            }
            x => Err(x)
        }
    }
}
impl ZipperTryCoalesce2 for SrcGroup {
    fn try_coalesce_focus2(self, between: Self, next: Self) -> Result<Self, (Self, Self, Self)> {
        match (self, between, next) {
            (Self::Id1(x), Self::Space(ws), Self::PlainText(y)) if y.is_valid_identfier() => {
                x   .try_promote(y)
                    .map(|x| Self::Id2(x))
                    .map_err(|(x, y)| (Self::Id1(x), Self::Space(ws), Self::PlainText(y)))
            }
            x => Err(x),
        }
    }
}

impl SomePrimitive {
    pub fn try_coalesce(self, other: Self) -> Result<Self, (Self, Self)> {
        match (self, other) {
            (Self::Backslash(x), Self::Backslash(y)) => Ok(Self::Backslash2(SrcChar2::new(x, y))),
            (Self::Pipe(x), Self::Pipe(y)) => Ok(Self::Pipe2(SrcChar2::new(x, y))),
            x => Err(x)
        }
    }
    pub fn try_coalesce_with(self, other: SrcGroup) -> Result<SrcGroup, (Self, SrcGroup)> {
        match (self, other) {
            (Self::AtSign(x), SrcGroup::PlainText(y)) => Ok(SrcGroup::Id1(SomeIdentifier1::at_sign(x, y))),
            (Self::Backslash(x), SrcGroup::PlainText(y)) => Ok(SrcGroup::Id1(SomeIdentifier1::backslash(x, y))),
            (Self::Pipe(x), SrcGroup::PlainText(y)) => Ok(SrcGroup::Id1(SomeIdentifier1::pipe(x, y))),
            x => Err(x)
        }
    }
}

