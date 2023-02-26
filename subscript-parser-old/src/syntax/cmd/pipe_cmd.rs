use crate::{
    binders::StreamBinder,
    output::Output,
    stream::{IndexedChar, Stream},
    syntax::{
        parts::{InCurlyBrackets, InSquareBrackets, LabeledEnclosure},
        RootAst,
    },
    token::TokenView,
};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct PipeCmdIdentifier<'a> {
    pub pipe_char: IndexedChar,
    pub identifier: TokenView<'a>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct PipeCmd<'a> {
    pub identifier: PipeCmdIdentifier<'a>,
    pub attributes: Option<InSquareBrackets<RootAst<'a>>>,
    pub argument: Option<InCurlyBrackets<RootAst<'a>>>,
    pub colon: Option<IndexedChar>,
    pub trailing: Vec<LabeledEnclosure<'a>>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct ParsePipeCmd {}

impl StreamBinder for ParsePipeCmd {
    type Ok<'a> = PipeCmd<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        unimplemented!()
    }
}
