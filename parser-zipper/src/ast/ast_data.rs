use crate::basics::{IndexedChar, IndexedString};

use super::{TagIdentifier, PipeTagIdentifier, LabeledEnclosure, Tag};

#[derive(Debug, Clone)]
pub enum ParseAst {
    Backslash1(IndexedChar),
    Backslash2(IndexedChar, IndexedChar),
    Pipe1(IndexedChar),
    Pipe2(IndexedChar, IndexedChar),
    OpenBracket(IndexedChar),
    CloseBracket(IndexedChar),
    PlainText(IndexedString),
    Whitespace(IndexedString),
    LabeledEnclosure(Box<LabeledEnclosure>),
    Tag(Box<Tag>),
    PipeTag {
        identifier: PipeTagIdentifier,
    },
    SectionHeader {
        open: IndexedString,
        contents: IndexedString,
        close: IndexedString,
    },
}