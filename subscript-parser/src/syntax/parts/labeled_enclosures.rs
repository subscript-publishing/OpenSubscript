use crate::{stream::IndexedChar, token::TokenView, syntax::RootAst};

use super::InSomeEnclosure;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum LabelIdentifier<'a> {
    Single {
        at_sign: IndexedChar,
        identifier: TokenView<'a>,
    },
    Double {
        at_sign: IndexedChar,
        first_identifier: TokenView<'a>,
        second_identifier: TokenView<'a>,
    },
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct LabeledEnclosure<'a> {
    pub identifier: LabelIdentifier<'a>,
    pub enclosure: InSomeEnclosure<RootAst<'a>>
}
