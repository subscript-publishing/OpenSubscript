use crate::binders::StreamBinder;
use crate::character::{TakeWhileSpec, UnconsSpec};
use crate::output::Output;
use crate::stream::{IndexedChar, Stream};
use crate::syntax::ast_parser;
use crate::syntax::parts::{LabeledEnclosure, ParseLabeledEnclosure};
use crate::syntax::{
    parts::{InCurlyBrackets, InSquareBrackets},
    RootAst,
};
use crate::token::TokenView;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct BackslashCmdIdentifier<'a> {
    pub backslash_token: IndexedChar,
    pub identifier: TokenView<'a>,
}

#[derive(Default)]
pub struct ParseBackslashCmdIdentifier;

impl StreamBinder for ParseBackslashCmdIdentifier {
    type Ok<'a> = BackslashCmdIdentifier<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream
            .static_twosome((
                &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match('\\')),
                &|stream: Stream<'a>| stream.apply_binder(TakeWhileSpec::IDENTIFIER),
            ))
            .ok_map(|(x, y)| BackslashCmdIdentifier {
                backslash_token: x,
                identifier: y,
            })
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TrailingLabeledArguments<'a> {
    pub arguments: Vec<LabeledEnclosure<'a>>,
}

#[derive(Default)]
pub struct ParseTrailingLabeledArguments;

impl StreamBinder for ParseTrailingLabeledArguments {
    type Ok<'a> = TrailingLabeledArguments<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        fn parser<'a>(stream: Stream<'a>) -> Output<'a, LabeledEnclosure<'a>, ()> {
            stream.apply_binder(ParseLabeledEnclosure::default())
        }
        stream
            .one_or_more(|stream| stream.between_opt_whitespace(true, &parser))
            .ok_map(|xs| TrailingLabeledArguments { arguments: xs })
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum BackslashCmd<'a> {
    Default {
        identifier: BackslashCmdIdentifier<'a>,
        attributes: Option<InSquareBrackets<RootAst<'a>>>,
        argument: Option<InCurlyBrackets<RootAst<'a>>>,
        trailing: Option<TrailingLabeledArguments<'a>>,
    },
    LineModeSugar {
        identifier: BackslashCmdIdentifier<'a>,
        colon: IndexedChar,
        rest_of_line: Vec<RootAst<'a>>,
        trailing: Option<TrailingLabeledArguments<'a>>,
    },
}

#[derive(Default)]
pub struct ParseBackslashCmd;

impl StreamBinder for ParseBackslashCmd {
    type Ok<'a> = BackslashCmd<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        fn line_mode_sugar<'a>(stream: Stream<'a>) -> Output<'a, BackslashCmd<'a>, ()> {
            fn parse_header<'a>(stream: Stream<'a>) -> Output<'a, (BackslashCmdIdentifier<'a>, IndexedChar), ()> {
                stream.static_twosome((
                    &|stream| stream.apply_binder(ParseBackslashCmdIdentifier),
                    &|stream| stream.apply_binder(UnconsSpec::must_match(':')),
                ))
            }
            stream
                .static_threesome((
                    &|stream| stream.between_opt_whitespace(true, &parse_header),
                    &|stream| stream.apply_binder(ast_parser::ParseRestOfLine::default()),
                    &|stream| stream.apply_optional_binder(ParseTrailingLabeledArguments::default()),
                ))
                .ok_map(|((a, b), y, z)| BackslashCmd::LineModeSugar { identifier: a, colon: b, rest_of_line: y, trailing: z })
        }
        fn parse_default<'a>(stream: Stream<'a>) -> Output<'a, BackslashCmd<'a>, ()> {
            stream
                .static_foursome((
                    &|stream| stream.between_opt_whitespace(true, |stream| stream.apply_binder(ParseBackslashCmdIdentifier)),
                    &|stream| stream.between_opt_whitespace(true, |stream| stream.apply_optional_binder(ast_parser::ParseInSquareBrackets)),
                    &|stream| stream.between_opt_whitespace(true, |stream| stream.apply_optional_binder(ast_parser::ParseInCurlyBrackets)),
                    &|stream| stream.apply_optional_binder(ParseTrailingLabeledArguments::default()),
                ))
                .ok_map(|(x, y, z, a)| BackslashCmd::Default { identifier: x, attributes: y, argument: z, trailing: a })
        }
        stream.static_alternatives(&[
            &line_mode_sugar,
            &parse_default,
        ])
    }
}

