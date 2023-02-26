use crate::{stream::{IndexedChar, Stream}, syntax::{RootAst, ast_parser}, token::TokenView, binders::StreamBinder, output::Output, character::{UnconsSpec, TakeWhileSpec}};

use super::InSomeEnclosure;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum LabelIdentifier<'a> {
    Single {
        at_sign: IndexedChar,
        identifier: TokenView<'a>,
    },
    Double {
        at_sign: IndexedChar,
        first_identifier: TokenView<'a>,
        second_identifier: TokenView<'a>,
    },
}


#[derive(Default)]
pub struct ParseLabelIdentifier;

impl StreamBinder for ParseLabelIdentifier {
    type Ok<'a> = LabelIdentifier<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        fn double<'a>(stream: Stream<'a>) -> Output<'a, LabelIdentifier<'a>, ()> {
            stream
                .static_foursome((
                    &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match('@')),
                    &|stream: Stream<'a>| stream.apply_binder(TakeWhileSpec::IDENTIFIER),
                    &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(' ')),
                    &|stream: Stream<'a>| stream.apply_binder(TakeWhileSpec::IDENTIFIER),
                ))
                .ok_map(|(x, y, _, z)| LabelIdentifier::Double { at_sign: x, first_identifier: y, second_identifier: z })
        }
        fn single<'a>(stream: Stream<'a>) -> Output<'a, LabelIdentifier<'a>, ()> {
            stream
                .static_twosome((
                    &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match('@')),
                    &|stream: Stream<'a>| stream.apply_binder(TakeWhileSpec::IDENTIFIER),
                ))
                .ok_map(|(x, y)| LabelIdentifier::Single { at_sign: x, identifier: y } )
        }
        stream.static_alternatives(&[
            &double,
            &single,
        ])
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct LabeledEnclosure<'a> {
    pub identifier: LabelIdentifier<'a>,
    pub enclosure: InSomeEnclosure<RootAst<'a>>,
}

#[derive(Default)]
pub struct ParseLabeledEnclosure;

impl StreamBinder for ParseLabeledEnclosure {
    type Ok<'a> = LabeledEnclosure<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream
            .static_twosome((
                &|stream: Stream| stream.between_opt_whitespace(false, |stream| stream.apply_binder(ParseLabelIdentifier::default())),
                &|stream: Stream| stream.apply_binder(ast_parser::ParseAstNodeInSomeEnclosure::default()),
            ))
            .ok_map(|(x, y)| LabeledEnclosure{identifier: x, enclosure: y})
    }
}
