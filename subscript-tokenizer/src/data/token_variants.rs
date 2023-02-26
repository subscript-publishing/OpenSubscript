use std::fmt::{Display, Write};

use tree_formatter::ToDisplayTree;

use crate::{SrcChar, SrcString, Range, SrcGroup};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// #[derive(Debug, Clone)]
// pub struct AnyWhitespace(pub IndexedString);
pub type AnyWhitespace = SrcString;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Newline(pub SrcString);

impl Newline {
    pub fn new(string: SrcString) -> Self {
        assert!(string.all_satisfies(|x| x == '\n'));
        Self(string)
    }
    pub fn range_of(&self) -> Range { self.0.range_of() }
    pub fn istring(self) -> SrcString { self.0 }
    pub fn append_other(&mut self, other: Self) {
        self.0.append(other.0);
    }
    pub fn with(mut self, other: Newline) -> Self {
        self.append_other(other);
        self
    }
}
impl ToDisplayTree for Newline {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ Newline")
        } else {
            String::default()
        };
        let label = format!("NL×{}{kind}", self.0.string.len());
        tree_formatter::DisplayTree::leaf(label)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Space(pub SrcString);

impl Space {
    pub fn new(string: SrcString) -> Self {
        assert!(string.all_satisfies(|x| x == ' '));
        Self(string)
    }
    pub fn range_of(&self) -> Range { self.0.range_of() }
    pub fn istring(self) -> SrcString { self.0 }
    pub fn column_len(&self) -> usize {
        self.0.string.len()
    }
    pub fn append_other(&mut self, other: Self) {
        self.0.append(other.0);
    }
    pub fn with(mut self, other: Self) -> Self {
        self.append_other(other);
        self
    }
}
impl ToDisplayTree for Space {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ Space")
        } else {
            String::default()
        };
        let label = format!("WS×{}{kind}", self.0.string.len());
        tree_formatter::DisplayTree::leaf(label)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// #[derive(Debug, Clone)]
// pub struct LeadingIndent(pub SrcString);

// pub type BeginColumn = LeadingIndent;

// impl LeadingIndent {
//     pub fn new(string: SrcString) -> Self {
//         assert!(string.all_satisfies(|x| x != '\n'));
//         Self(string)
//     }
//     pub fn column_len(&self) -> usize {
//         self.0.string.len()
//     }
//     pub fn range_of(&self) -> Range { self.0.range_of() }
//     pub fn istring(self) -> SrcString { self.0 }
//     pub fn append_other(&mut self, other: Self) {
//         self.0.append(other.0);
//     }
// }
// impl ToDisplayTree for BeginColumn {
//     fn to_display_tree(&self) -> tree_formatter::DisplayTree {
//         let label = format!("BeginColumn{{count: {}}}", self.column_len());
//         tree_formatter::DisplayTree::leaf(label)
//     }
// }

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct SrcChar2 {
    pub first: SrcChar,
    pub second: SrcChar,
}

impl SrcChar2 {
    pub fn new(first: SrcChar, second: SrcChar) -> Self {
        Self { first, second }
    }
    pub fn range_of(&self) -> Range {
        self.first.pos.range_to(self.second.pos)
    }
    pub fn istring(self) -> SrcString {
        SrcString::from_iter([self.first, self.second])
    }
    pub fn to_tuple(self) -> (SrcChar, SrcChar) {
        (self.first, self.second)
    }
    pub fn semantically_equal_to(&self, other: &Self) -> bool {
        self.first.char == other.first.char &&
        self.second.char == other.second.char
    }
}

impl From<(SrcChar, SrcChar)> for SrcChar2 {
    fn from((first, second): (SrcChar, SrcChar)) -> Self { Self { first, second } }
}
impl ToDisplayTree for SrcChar2 {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SrcChar2")
        } else {
            String::default()
        };
        let label = format!("'{}{}'{kind}", self.first.char, self.second.char);
        tree_formatter::DisplayTree::leaf(label)
    }
}
impl Display for SrcChar2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_char(self.first.char)?;
        f.write_char(self.second.char)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub type Pipe2 = SrcChar2;
pub type Backslash2 = SrcChar2;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SYMBOL TOKEN
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagIdentifier {
    pub backslash: SrcChar,
    pub identifier: SrcString,
}
impl TagIdentifier {
    pub fn new(backslash: SrcChar, identifier: SrcString) -> Self {
        Self { backslash, identifier }
    }
    pub fn range_of(&self) -> Range {
        self.backslash.pos.range_to(self.identifier.range_of().position)
    }
    pub fn into_string(self) -> SrcString {
        let mut xs = SrcString::singleton(self.backslash);
        xs.append(self.identifier);
        xs
    }
    pub fn append_to_identifier(&mut self, trailing: SrcString) {
        self.identifier.append(trailing)
    }
    pub fn istring(self) -> SrcString {
        let mut xs = SrcString::singleton(self.backslash);
        xs.append(self.identifier);
        xs
    }
}
impl ToDisplayTree for TagIdentifier {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ TagIdentifier")
        } else {
            String::default()
        };
        let label = format!("`{}{}`{kind}", self.backslash, self.identifier);
        tree_formatter::DisplayTree::leaf(label)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct PipeTagIdentifier {
    pub pipe: SrcChar,
    pub identifier: SrcString,
}
impl PipeTagIdentifier {
    pub fn range_of(&self) -> Range {
        self.pipe.pos.range_to(self.identifier.range_of().position)
    }
    pub fn istring(self) -> SrcString {
        let mut xs = SrcString::singleton(self.pipe);
        xs.append(self.identifier);
        xs
    }
    pub fn append_to_identifier(&mut self, trailing: SrcString) {
        self.identifier.append(trailing)
    }
}
impl ToDisplayTree for PipeTagIdentifier {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ PipeTagIdentifier")
        } else {
            String::default()
        };
        let label = format!("`{}{}`{kind}", self.pipe, self.identifier);
        tree_formatter::DisplayTree::leaf(label)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct LabelIdentifier1 {
    pub at_sign: SrcChar,
    pub identifier: SrcString,
}

impl LabelIdentifier1 {
    pub fn begin_header(at_sign: SrcChar) -> Self {
        Self { at_sign, identifier: Default::default() }
    }
    pub fn range_of(&self) -> Range {
        self.at_sign.pos.range_to(self.identifier.range_of().position)
    }
    pub fn istring(self) -> SrcString {
        let mut xs = SrcString::singleton(self.at_sign);
        xs.append(self.identifier);
        xs
    }
    pub fn append_to_identifier(&mut self, trailing: SrcString) {
        self.identifier.append(trailing)
    }
}
impl ToDisplayTree for LabelIdentifier1 {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ LabelIdentifier1")
        } else {
            String::default()
        };
        let label = format!("`{}{}`{kind}", self.at_sign, self.identifier);
        tree_formatter::DisplayTree::leaf(label)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct LabelIdentifier2 {
    pub at_sign: SrcChar,
    pub identifier1: SrcString,
    pub whitespace: AnyWhitespace,
    pub identifier2: SrcString,
}

impl LabelIdentifier2 {
    pub fn range_of(&self) -> Range {
        self.at_sign.pos.range_to(self.identifier2.range_of().position)
    }
    pub fn istring(self) -> SrcString {
        let mut xs = SrcString::singleton(self.at_sign);
        xs.append(self.identifier1);
        xs.append(self.whitespace);
        xs.append(self.identifier2);
        xs
    }
}
impl ToDisplayTree for LabelIdentifier2 {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ LabelIdentifier2")
        } else {
            String::default()
        };
        let label = format!("`{}{} {}`{kind}", self.at_sign, self.identifier1, self.identifier2);
        tree_formatter::DisplayTree::leaf(label)
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct SomeSectionHeader {
    pub open: SrcChar2,
    pub contents: Vec<SrcGroup>,
    pub close: SrcChar2,
}

impl SomeSectionHeader {
    pub fn new(
        open: SrcChar2,
        contents: Vec<SrcGroup>,
        close: SrcChar2,
    ) -> Self {
        SomeSectionHeader { open, contents, close }
    }
    pub fn range_of(&self) -> Range {
        self.open.first.pos.range_to(self.close.second.pos)
    }
    pub fn istring(self) -> SrcString {
        let mut xs = SrcString::default();
        xs.push(self.open.first);
        xs.push(self.open.second);
        for x in self.contents.clone() {
            xs.append(x.istring());
        }
        xs.push(self.close.first);
        xs.push(self.close.second);
        xs
    }
}
impl ToDisplayTree for SomeSectionHeader {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let sep = if crate::DEBUG_USE_TABS {'\t'} else {' '};
        let kind = if crate::DEBUG_SHOW_TYPE {
            format!("{sep}∷ SomeSectionHeader")
        } else {
            String::default()
        };
        let label = format!(
            "`{}`{kind}",
            self.clone().istring(),
        );
        tree_formatter::DisplayTree::leaf(label)
    }
}