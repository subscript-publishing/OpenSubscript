use crate::{binders::StreamBinder, stream::Stream, output::Output};

use super::{cmd::PipeCmd, plain_text::PlainText};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
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
        unimplemented!()
    }
}

