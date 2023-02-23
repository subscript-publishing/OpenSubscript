use crate::{token::TokenView, binders::StreamBinder, stream::Stream, output::Output, character::TakeWhileSpec};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct PlainText<'a> {
    pub content: TokenView<'a>
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default, Clone, Copy)]
pub struct ParsePlainText {}

impl StreamBinder for ParsePlainText {
    type Ok<'a> = PlainText<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.apply_binder(TakeWhileSpec::NOT_SYMBOL).ok_map(|x| PlainText{content: x})
    }
}