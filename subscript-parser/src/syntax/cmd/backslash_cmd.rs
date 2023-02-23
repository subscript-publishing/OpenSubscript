use crate::{stream::{IndexedChar, Stream}, token::TokenView, syntax::{parts::{InSquareBrackets, InCurlyBrackets, LabeledEnclosure}, RootAst}, binders::StreamBinder, output::Output, character::UnconsSpec};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct BackslashCmdIdentifier<'a> {
    pub backslash_token: IndexedChar,
    pub identifier: TokenView<'a>
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct BackslashCmd<'a> {
    pub identifier: BackslashCmdIdentifier<'a>,
    pub attributes: Option<InSquareBrackets<RootAst<'a>>>,
    pub argument: Option<InCurlyBrackets<RootAst<'a>>>,
    pub colon: Option<IndexedChar>,
    pub trailing: Vec<LabeledEnclosure<'a>>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct ParseBackslashCmd {}

impl StreamBinder for ParseBackslashCmd {
    type Ok<'a> = BackslashCmd<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        // let results = stream.static_threesome((
        //     &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match('\\')),
        //     // &|stream: Stream<'a>| stream.apply_binder(InSquareBrackets::),
        // ));
        unimplemented!()
    }
}
