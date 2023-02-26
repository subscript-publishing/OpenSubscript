use crate::{
    binders::StreamBinder,
    output::{Output, IO},
    stream::Stream,
    syntax::{plain_text::{ParsePlainText, PlainText}, RootAst, cmd::ParseBackslashCmd}, character::UnconsSpec,
};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseRootAst;

impl StreamBinder for ParseRootAst {
    type Ok<'a> = RootAst<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.static_alternatives(&[
            &|stream| stream.apply_binder(ParseBackslashCmd::default()).ok_map(|x| RootAst::BackslashCmd(Box::new(x))),
            &|stream| stream.apply_binder(ParsePlainText::default()).ok_map(|x| RootAst::PlainText(x)),
        ])
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseAnything;

impl StreamBinder for ParseAnything {
    type Ok<'a> = RootAst<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.static_alternatives(&[
            &|stream| stream.apply_binder(ParseRootAst::default()),
            &|stream| stream.apply_binder(UnconsSpec::default()).ok_map(|x| RootAst::CharError(x)),
        ])
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseEverything;

impl StreamBinder for ParseEverything {
    type Ok<'a> = Vec<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let mut results: Vec<RootAst<'a>> = Default::default();
        let mut context = stream;
        while let Output::Success(IO { context: next, value }) = context.apply_binder(ParseAnything::default()) {
            assert!(context.cursor.index != next.cursor.index);
            context = next;
            results.push(value);
        }
        if !context.slice.is_empty() {
            let end_cursor = context.cursor.forward_sync_for(context.slice);
            let rest = PlainText{content: context.to_token_view()};
            results.push(RootAst::PlainText(rest));
            context = Stream{cursor: end_cursor, slice: &[]};
        }
        let end_cursor = context.cursor;
        Output::Success(IO { context: Stream { slice: &[], cursor: end_cursor }, value: results })
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseRestOfLine;

impl StreamBinder for ParseRestOfLine {
    type Ok<'a> = Vec<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let origional = stream.clone();
        let (leading, trailing) = stream.partition_at_newline();
        if leading.slice.is_empty() {
            return Output::Failure(IO::no_op(origional))
        }
        match leading.apply_binder(ParseEverything::default()) {
            Output::Success(IO { context: leftward, value }) => {
                assert!(leftward.slice.is_empty());
                Output::Success(IO { context: trailing, value: value })
            }
            Output::Failure(IO { context: _, value: _ }) => {
                unimplemented!("WHAT DO TO?")
            }
        }
    }
}
