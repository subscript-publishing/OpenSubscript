use crate::{stream::{IndexedChar, Stream}, syntax::{RootAst, ParseRootAst}, output::Output, binders::StreamBinder, character::UnconsSpec};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum InSomeEnclosure<T> {
    /// `{` and '}'
    CurlyBrackets(InCurlyBrackets<T>),
    /// (`[` and `]`
    SquareBrackets(InSquareBrackets<T>),
    /// (`(` and `)`
    RoundBrackets(InRoundBrackets<T>),
    /// (`<` and `>`
    AngleBrackets(InAngleBrackets<T>),
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InCurlyBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InCurlyBrackets<T> {
    const OPEN_CHAR: char = '{';
    const CLOSE_CHAR: char = '}';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InSquareBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InSquareBrackets<T> {
    const OPEN_CHAR: char = '[';
    const CLOSE_CHAR: char = ']';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InRoundBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InRoundBrackets<T> {
    const OPEN_CHAR: char = '(';
    const CLOSE_CHAR: char = ')';
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct InAngleBrackets<T> {
    pub open: IndexedChar,
    pub content: T,
    pub close: IndexedChar,
}

impl<T> InAngleBrackets<T> {
    const OPEN_CHAR: char = '<';
    const CLOSE_CHAR: char = '>';
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// AST BASED INDIVIDUAL PARSERS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Default)]
pub struct ParseAstNodeInCurlyBrackets;
#[derive(Default)]
pub struct ParseAstNodeInSquareBrackets;
#[derive(Default)]
pub struct ParseAstNodeInRoundBrackets;
#[derive(Default)]
pub struct ParseAstNodeInAngleBrackets;

impl StreamBinder for ParseAstNodeInCurlyBrackets {
    type Ok<'a> = InCurlyBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (InCurlyBrackets::<()>::OPEN_CHAR, InCurlyBrackets::<()>::CLOSE_CHAR);
        stream.static_threesome((
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(open)),
            &|stream: Stream<'a>| stream.apply_binder(ParseRootAst::default()),
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(close)),
        ))
        .ok_map(|(open, content, close)| Self::Ok{open, content, close})
    }
}
impl StreamBinder for ParseAstNodeInSquareBrackets {
    type Ok<'a> = InSquareBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (InSquareBrackets::<()>::OPEN_CHAR, InSquareBrackets::<()>::CLOSE_CHAR);
        stream.static_threesome((
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(open)),
            &|stream: Stream<'a>| stream.apply_binder(ParseRootAst::default()),
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(close)),
        ))
        .ok_map(|(open, content, close)| Self::Ok{open, content, close})
    }
}
impl StreamBinder for ParseAstNodeInRoundBrackets {
    type Ok<'a> = InRoundBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (InRoundBrackets::<()>::OPEN_CHAR, InRoundBrackets::<()>::CLOSE_CHAR);
        stream.static_threesome((
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(open)),
            &|stream: Stream<'a>| stream.apply_binder(ParseRootAst::default()),
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(close)),
        ))
        .ok_map(|(open, content, close)| Self::Ok{open, content, close})
    }
}
impl StreamBinder for ParseAstNodeInAngleBrackets {
    type Ok<'a> = InAngleBrackets<RootAst<'a>>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        let (open, close) = (InAngleBrackets::<()>::OPEN_CHAR, InAngleBrackets::<()>::CLOSE_CHAR);
        stream.static_threesome((
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(open)),
            &|stream: Stream<'a>| stream.apply_binder(ParseRootAst::default()),
            &|stream: Stream<'a>| stream.apply_binder(UnconsSpec::must_match(close)),
        ))
        .ok_map(|(open, content, close)| Self::Ok{open, content, close})
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
            &|stream: Stream<'a>| stream
                .apply_binder(ParseAstNodeInCurlyBrackets::default())
                .ok_map(|x| InSomeEnclosure::CurlyBrackets(x)),
            &|stream: Stream<'a>| stream
                .apply_binder(ParseAstNodeInSquareBrackets::default())
                .ok_map(|x| InSomeEnclosure::SquareBrackets(x)),
            &|stream: Stream<'a>| stream
                .apply_binder(ParseAstNodeInRoundBrackets::default())
                .ok_map(|x| InSomeEnclosure::RoundBrackets(x)),
            &|stream: Stream<'a>| stream
                .apply_binder(ParseAstNodeInAngleBrackets::default())
                .ok_map(|x| InSomeEnclosure::AngleBrackets(x)),
        ])
    }
}
