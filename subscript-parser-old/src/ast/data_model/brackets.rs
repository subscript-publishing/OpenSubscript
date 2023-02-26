use crate::{
    binders::StreamBinder,
    character::UnconsSpec,
    output::Output,
    stream::{IndexedChar, Stream},
};

use super::{ParseAst, ParseRootAst};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum InSomeEnclosure<'a> {
    /// `{` and '}'
    CurlyBrackets(InCurlyBrackets<'a>),
    /// (`[` and `]`
    SquareBrackets(InSquareBrackets<'a>),
    /// (`(` and `)`
    RoundBrackets(InRoundBrackets<'a>),
    /// (`<` and `>`
    AngleBrackets(InAngleBrackets<'a>),
}

#[derive(Default)]
pub struct ParseInSomeEnclosure {}
impl StreamBinder for ParseInSomeEnclosure {
    type Ok<'a> = InSomeEnclosure<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.static_alternatives(&[
            &|stream: Stream<'a>| -> Output<'a, InSomeEnclosure<'a>, ()> {
                stream
                    .apply_binder(ParseInCurlyBrackets::default())
                    .ok_map(|x| InSomeEnclosure::CurlyBrackets(x))
            },
            &|stream: Stream<'a>| -> Output<'a, InSomeEnclosure<'a>, ()> {
                stream
                    .apply_binder(ParseInSquareBrackets::default())
                    .ok_map(|x| InSomeEnclosure::SquareBrackets(x))
            },
            &|stream: Stream<'a>| -> Output<'a, InSomeEnclosure<'a>, ()> {
                stream
                    .apply_binder(ParseInRoundBrackets::default())
                    .ok_map(|x| InSomeEnclosure::RoundBrackets(x))
            },
            &|stream: Stream<'a>| -> Output<'a, InSomeEnclosure<'a>, ()> {
                stream
                    .apply_binder(ParseInAngleBrackets::default())
                    .ok_map(|x| InSomeEnclosure::AngleBrackets(x))
            },
        ])
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InCurlyBrackets<'a> {
    pub open: IndexedChar,
    pub content: ParseAst<'a>,
    pub close: IndexedChar,
}

impl<'a> InCurlyBrackets<'a> {
    const OPEN_CHAR: char = '{';
    const CLOSE_CHAR: char = '{';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InSquareBrackets<'a> {
    pub open: IndexedChar,
    pub content: ParseAst<'a>,
    pub close: IndexedChar,
}

impl<'a> InSquareBrackets<'a> {
    const OPEN_CHAR: char = '[';
    const CLOSE_CHAR: char = ']';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InRoundBrackets<'a> {
    pub open: IndexedChar,
    pub content: ParseAst<'a>,
    pub close: IndexedChar,
}

impl<'a> InRoundBrackets<'a> {
    const OPEN_CHAR: char = '(';
    const CLOSE_CHAR: char = ')';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InAngleBrackets<'a> {
    pub open: IndexedChar,
    pub content: ParseAst<'a>,
    pub close: IndexedChar,
}

impl<'a> InAngleBrackets<'a> {
    const OPEN_CHAR: char = '<';
    const CLOSE_CHAR: char = '>';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default, Debug)]
pub struct ParseInCurlyBrackets {}
#[derive(Default, Debug)]
pub struct ParseInSquareBrackets {}
#[derive(Default, Debug)]
pub struct ParseInRoundBrackets {}
#[derive(Default, Debug)]
pub struct ParseInAngleBrackets {}

impl StreamBinder for ParseInCurlyBrackets {
    type Ok<'a> = InCurlyBrackets<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (InCurlyBrackets::OPEN_CHAR, InCurlyBrackets::CLOSE_CHAR);
        // stream
        //     .apply_binder(UnconsSpec::must_match(open))
        //     .ok_and(|stream| stream.apply_binder(ParseRootAst::default()))
        //     .ok_and(|stream| stream.apply_binder(UnconsSpec::must_match(close)))
        //     .ok_map(|((open, content), close)| -> InCurlyBrackets {
        //         InCurlyBrackets { open, content, close }
        //     })
        unimplemented!()
    }
}
impl StreamBinder for ParseInSquareBrackets {
    type Ok<'a> = InSquareBrackets<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        // let (open, close) = (InSquareBrackets::OPEN_CHAR, InSquareBrackets::CLOSE_CHAR);
        // stream
        //     .apply_binder(UnconsSpec::must_match(open))
        //     .ok_and(|stream| stream.apply_binder(ParseRootAst::default()))
        //     .ok_and(|stream| stream.apply_binder(UnconsSpec::must_match(close)))
        //     .ok_map(|((open, content), close)| -> InSquareBrackets {
        //         InSquareBrackets { open, content, close }
        //     })
        unimplemented!()
    }
}
impl StreamBinder for ParseInRoundBrackets {
    type Ok<'a> = InRoundBrackets<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        // let (open, close) = (InRoundBrackets::OPEN_CHAR, InRoundBrackets::CLOSE_CHAR);
        // stream
        //     .apply_binder(UnconsSpec::must_match(open))
        //     .ok_and(|stream| stream.apply_binder(ParseRootAst::default()))
        //     .ok_and(|stream| stream.apply_binder(UnconsSpec::must_match(close)))
        //     .ok_map(|((open, content), close)| -> InRoundBrackets {
        //         InRoundBrackets { open, content, close }
        //     })
        unimplemented!()
    }
}
impl StreamBinder for ParseInAngleBrackets {
    type Ok<'a> = InAngleBrackets<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        // let (open, close) = (InAngleBrackets::OPEN_CHAR, InAngleBrackets::CLOSE_CHAR);
        // stream
        //     .apply_binder(UnconsSpec::must_match(open))
        //     .ok_and(|stream| stream.apply_binder(ParseRootAst::default()))
        //     .ok_and(|stream| stream.apply_binder(UnconsSpec::must_match(close)))
        //     .ok_map(|((open, content), close)| -> InAngleBrackets {
        //         InAngleBrackets { open, content, close }
        //     })
        unimplemented!()
    }
}
