use crate::{stream::{IndexedChar, Stream}, token::TokenView, syntax::{parts::{InSquareBrackets, InCurlyBrackets, LabeledEnclosure}, RootAst}, binders::StreamBinder, output::Output};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct PipeCmdIdentifier<'a> {
    pub pipe_char: IndexedChar,
    pub identifier: TokenView<'a>,
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
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


