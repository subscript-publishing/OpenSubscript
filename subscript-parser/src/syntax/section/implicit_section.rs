use crate::{token::TokenView, syntax::parts::SomeIndentedList};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct ImplicitHeader<'a> {
    pub begin_token: TokenView<'a>,
    pub ident: TokenView<'a>,
    pub close_token: TokenView<'a>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// pub enum ImplicitSectionContent<'a> {
//     Enumerated(SomeIndentedList<'a>)
// }

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub struct ImplicitSection<'a> {
    pub header: ImplicitHeader<'a>,
}



