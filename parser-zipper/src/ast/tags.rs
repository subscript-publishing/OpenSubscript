use super::{TagIdentifier, InCurlyBrackets, ParseAst, InSquareBrackets, LabeledEnclosure};

#[derive(Debug, Clone)]
pub struct Tag {
    pub identifier: TagIdentifier,
    pub attributes: Option<InSquareBrackets>,
    pub argument: Option<InCurlyBrackets>,
    pub trailing: Vec<LabeledEnclosure>,
}

