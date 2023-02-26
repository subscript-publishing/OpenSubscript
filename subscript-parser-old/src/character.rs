use std::{ops::Not, rc::Rc};

use crate::{
    binders::StreamBinder,
    output::{Output, IO},
    stream::{Cursor, IndexedChar, Stream},
    token::TokenView,
};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct CharView {
    pub span_index: usize,
    pub start_cursor: Cursor,
    pub value: IndexedChar,
}

impl CharView {
    pub fn is_first_char(&self) -> bool {
        self.span_index == 0
    }
}

pub trait CharPredicate {
    fn satisfies(&self, view: CharView) -> bool;
}

impl CharPredicate for () {
    fn satisfies(&self, view: CharView) -> bool {
        true
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub struct StaticCharPredicate {
    predicate: &'static dyn Fn(CharView) -> bool,
}

impl StaticCharPredicate {
    pub const fn new(f: &'static dyn Fn(CharView) -> bool) -> Self {
        Self { predicate: f }
    }
    pub fn is_valid(&self, view: CharView) -> bool {
        (self.predicate)(view)
    }
}

impl CharPredicate for StaticCharPredicate {
    fn satisfies(&self, view: CharView) -> bool {
        self.is_valid(view)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct DynamicCharPredicate {
    predicate: Rc<dyn Fn(CharView) -> bool>,
}

impl DynamicCharPredicate {
    pub fn new(f: impl Fn(CharView) -> bool + 'static) -> Self {
        Self {
            predicate: Rc::new(f),
        }
    }
    pub fn is_valid(&self, view: CharView) -> bool {
        (self.predicate)(view)
    }
}

impl CharPredicate for DynamicCharPredicate {
    fn satisfies(&self, view: CharView) -> bool {
        self.is_valid(view)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct TakeWhileSpec<Predicate> {
    pub while_true: Predicate,
    pub ignoring: Option<Predicate>,
}

impl<P: CharPredicate> TakeWhileSpec<P> {
    pub fn is_valid(&self, view: CharView) -> bool {
        if let Some(ignoring) = self.ignoring.as_ref() {
            self.while_true.satisfies(view) || ignoring.satisfies(view)
        } else {
            self.while_true.satisfies(view)
        }
    }
}

impl<P: CharPredicate + 'static> StreamBinder for TakeWhileSpec<P> {
    type Ok<'a> = TokenView<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        for (ix, char) in stream.slice.into_iter().enumerate() {
            let view = CharView {
                span_index: ix,
                start_cursor: stream.cursor,
                value: *char,
            };
            if !self.is_valid(view) {
                let value = &stream.slice[0..ix];
                if value.is_empty() {
                    return Output::failure(IO::no_op(stream));
                }
                return Output::success(IO {
                    context: Stream {
                        slice: &stream.slice[ix..],
                        cursor: stream.cursor.forward_sync_for(value),
                    },
                    value: Stream {
                        slice: value,
                        cursor: stream.cursor,
                    }
                    .to_token_view(),
                });
            }
        }
        let value = &stream.slice[0..];
        Output::success(IO {
            context: Stream {
                slice: &[],
                cursor: stream.cursor.forward_sync_for(value),
            },
            value: Stream {
                slice: value,
                cursor: stream.cursor,
            }
            .to_token_view(),
        })
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// MATCHES
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl StaticCharPredicate {
    /// TODO - CHECK ALL UNICODE WHITESPACE CHARS
    pub const IS_WHITESPACE: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            view.value.char == ' '
        }
        StaticCharPredicate::new(&predicate)
    };
    /// TODO - CHECK ALL UNICODE NEWLINES
    pub const IS_NEWLINE: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            view.value.char == '\n'
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_WHITESPACE_OR_NEWLINE: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            StaticCharPredicate::IS_WHITESPACE.is_valid(view) || StaticCharPredicate::IS_NEWLINE.is_valid(view)
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_NOT_NEWLINE: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            StaticCharPredicate::IS_NEWLINE.is_valid(view).not()
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_LETTER: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            view.value.char.is_alphabetic()
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_NUMBER: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            view.value.char.is_numeric()
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_LETTER_OR_NUMBER: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            view.value.char.is_alphanumeric()
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_IDENTIFIER: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            if view.is_first_char() {
                StaticCharPredicate::IS_LETTER.is_valid(view)
            } else {
                StaticCharPredicate::IS_LETTER_OR_NUMBER.is_valid(view)
            }
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_SYMBOL: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            match view.value.char {
                '\\' => true,
                '@' => true,
                '#' => true,
                '{' => true,
                '}' => true,
                '[' => true,
                ']' => true,
                '(' => true,
                ')' => true,
                '<' => true,
                '>' => true,
                '\n' => true,
                _ => false,
            }
        }
        StaticCharPredicate::new(&predicate)
    };
    pub const IS_NOT_SYMBOL: StaticCharPredicate = {
        fn predicate(view: CharView) -> bool {
            StaticCharPredicate::IS_SYMBOL.is_valid(view).not()
        }
        StaticCharPredicate::new(&predicate)
    };
}

impl TakeWhileSpec<StaticCharPredicate> {
    pub const IDENTIFIER: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_IDENTIFIER,
            ignoring: None,
        }
    };
    pub const SYMBOL: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_SYMBOL,
            ignoring: None,
        }
    };
    pub const NOT_SYMBOL: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_NOT_SYMBOL,
            ignoring: None,
        }
    };
    pub const WHITESPACE: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_WHITESPACE,
            ignoring: None,
        }
    };
    pub const NEWLINE: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_NEWLINE,
            ignoring: None,
        }
    };
    pub const WHITESPACE_OR_NEWLINE: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_WHITESPACE_OR_NEWLINE,
            ignoring: None,
        }
    };
    pub const NOT_NEWLINE: Self = {
        TakeWhileSpec {
            while_true: StaticCharPredicate::IS_NOT_NEWLINE,
            ignoring: None,
        }
    };
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub struct UnconsSpec<Predicate = ()> {
    pub filter: Option<Predicate>,
}

impl Default for UnconsSpec {
    fn default() -> Self {
        UnconsSpec { filter: None }
    }
}

impl UnconsSpec<DynamicCharPredicate> {
    pub fn must_match(char: char) -> Self {
        let predicate = DynamicCharPredicate::new(move |view| view.value.char == char);
        Self::with_filter(predicate)
    }
}

impl<P: CharPredicate> UnconsSpec<P> {
    pub fn with_filter(predicate: P) -> Self {
        Self {
            filter: Some(predicate),
        }
    }
}

impl UnconsSpec<()> {
    pub fn match_any() -> Self {
        Self { filter: None }
    }
}

impl<P: CharPredicate + 'static> StreamBinder for UnconsSpec<P> {
    type Ok<'a> = IndexedChar;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        if stream.slice.is_empty() {
            return Output::failure(IO::no_op(stream));
        }
        let head = stream.slice.first().unwrap().clone();
        let is_valid = self.filter.map_or(true, |p| {
            p.satisfies(CharView {
                span_index: 0,
                start_cursor: stream.cursor,
                value: head,
            })
        });
        if !is_valid {
            return Output::failure(IO::no_op(stream));
        }
        let rest = &stream.slice[1..];
        Output::success(IO {
            context: Stream {
                cursor: stream.cursor.forward_sync_for(&[head]),
                slice: rest,
            },
            value: head,
        })
    }
}
