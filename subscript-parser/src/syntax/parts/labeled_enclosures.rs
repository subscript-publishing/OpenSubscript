use crate::{stream::IndexedChar, token::TokenView, syntax::RootAst};

use super::InSomeEnclosure;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
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
pub struct LabeledEnclosure<'a> {
    pub identifier: LabelIdentifier<'a>,
    pub enclosure: InSomeEnclosure<RootAst<'a>>
}
