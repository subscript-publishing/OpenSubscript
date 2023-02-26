use crate::{
    binders::StreamBinder,
    character::UnconsSpec,
    output::Output,
    stream::Stream,
    syntax::{parts::*, RootAst},
};

use super::ParseRootAst;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// AST BASED INDIVIDUAL PARSERS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseInCurlyBrackets;
#[derive(Default)]
pub struct ParseInSquareBrackets;
#[derive(Default)]
pub struct ParseInRoundBrackets;
#[derive(Default)]
pub struct ParseInAngleBrackets;

impl StreamBinder for ParseInCurlyBrackets {
    type Ok<'a> = InCurlyBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (
            InCurlyBrackets::<()>::OPEN_CHAR,
            InCurlyBrackets::<()>::CLOSE_CHAR,
        );
        stream
            .between_optional((
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(open))),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(ParseRootAst::default())),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(close))),
            ))
            .ok_map(|(open, content, close)| Self::Ok {
                open,
                content,
                close,
            })
    }
}
impl StreamBinder for ParseInSquareBrackets {
    type Ok<'a> = InSquareBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (
            InSquareBrackets::<()>::OPEN_CHAR,
            InSquareBrackets::<()>::CLOSE_CHAR,
        );
        stream
            .between_optional((
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(open))),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(ParseRootAst::default())),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(close))),
            ))
            .ok_map(|(open, content, close)| Self::Ok {
                open,
                content,
                close,
            })
    }
}
impl StreamBinder for ParseInRoundBrackets {
    type Ok<'a> = InRoundBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (
            InRoundBrackets::<()>::OPEN_CHAR,
            InRoundBrackets::<()>::CLOSE_CHAR,
        );
        stream
            .between_optional((
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(open))),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(ParseRootAst::default())),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(close))),
            ))
            .ok_map(|(open, content, close)| Self::Ok {
                open,
                content,
                close,
            })
    }
}
impl StreamBinder for ParseInAngleBrackets {
    type Ok<'a> = InAngleBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (
            InAngleBrackets::<()>::OPEN_CHAR,
            InAngleBrackets::<()>::CLOSE_CHAR,
        );
        stream
            .between_optional((
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(open))),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(ParseRootAst::default())),
                &|stream: Stream<'a>| stream.between_opt_whitespace(true, |stream| stream.apply_binder(UnconsSpec::must_match(close))),
            ))
            .ok_map(|(open, content, close)| Self::Ok {
                open,
                content,
                close,
            })
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// AST BASED ANY-TYPE PARSER
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseAstNodeInSomeEnclosure;
impl StreamBinder for ParseAstNodeInSomeEnclosure {
    type Ok<'a> = InSomeEnclosure<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.static_alternatives(&[
            &|stream: Stream<'a>| {
                stream
                    .apply_binder(ParseInCurlyBrackets::default())
                    .ok_map(|x| InSomeEnclosure::CurlyBrackets(x))
            },
            &|stream: Stream<'a>| {
                stream
                    .apply_binder(ParseInSquareBrackets::default())
                    .ok_map(|x| InSomeEnclosure::SquareBrackets(x))
            },
            &|stream: Stream<'a>| {
                stream
                    .apply_binder(ParseInRoundBrackets::default())
                    .ok_map(|x| InSomeEnclosure::RoundBrackets(x))
            },
            &|stream: Stream<'a>| {
                stream
                    .apply_binder(ParseInAngleBrackets::default())
                    .ok_map(|x| InSomeEnclosure::AngleBrackets(x))
            },
        ])
    }
}
