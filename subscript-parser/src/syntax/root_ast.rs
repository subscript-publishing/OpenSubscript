use crate::{binders::StreamBinder, stream::Stream, output::Output};

use super::{cmd::PipeCmd, plain_text::{PlainText, ParsePlainText}};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum RootAst<'a> {
    BackslashCmd(Box<PipeCmd<'a>>),
    PipeCmd(Box<PipeCmd<'a>>),
    PlainText(PlainText<'a>),
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseRootAst {}

impl StreamBinder for ParseRootAst {
    type Ok<'a> = RootAst<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.static_alternatives(&[
            &|stream| stream.apply_binder(ParsePlainText::default()).ok_map(|x| RootAst::PlainText(x))
        ])
    }
}

