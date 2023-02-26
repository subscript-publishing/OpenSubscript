use crate::{binders::StreamBinder, output::Output, stream::Stream};

use super::TagElement;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// ROOT AST
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum ParseAst<'a> {
    TagElement(TagElement<'a>),
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// ROOT AST PARSER
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseRootAst {}

impl StreamBinder for ParseRootAst {
    type Ok<'a> = ParseAst<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        unimplemented!()
    }
}
