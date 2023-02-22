use crate::stream::{Stream, Cursor};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct InContext<V, Ctx> {
    pub value: V,
    pub context: Ctx,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct IO<'a, Val=()> {
    pub context: Stream<'a>,
    pub value: Val,
}

impl<'a, A> IO<'a, A> {
    pub fn new(value: A, context: Stream<'a>) -> IO<'a, A> { IO { value, context } }
    pub fn map<B>(self, f: impl FnOnce(A) -> B) -> IO<'a, B> {
        IO { context: self.context, value: f(self.value) }
    }
    pub fn map_context(self, f: impl FnOnce(Stream<'a>) -> Stream<'a>) -> IO<'a, A> {
        IO { context: f(self.context), value: self.value }
    }
    pub fn and<B>(self, f: impl FnOnce(Stream<'a>) -> IO<'a, B>) -> IO<'a, (A, B)> {
        let IO { value: a, context } = self;
        let IO { value: b, context } = f(context);
        IO { context, value: (a, b) }
    }
}

impl<'a> IO<'a> {
    pub fn no_op(context: Stream<'a>) -> IO<'a> {
        IO { value: (), context }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum Output<'a, Ok, Err=()> {
    Success(IO<'a, Ok>),
    Failure(IO<'a, Err>),
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INITIALIZATIONS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a, Ok, Err> Output<'a, Ok, Err> {
    pub fn success(io: IO<'a, Ok>) -> Self { Self::Success(io) }
    pub fn failure(io: IO<'a, Err>) -> Self { Self::Failure(io) }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TRANSFORM MATCH-RELATED COMPONENTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a, Ok, Err> Output<'a, Ok, Err> {
    pub fn ok_map<B>(self, f: impl FnOnce(Ok) -> B) -> Output<'a, B, Err> {
        match self {
            Output::Success(x) => Output::Success(x.map(f)),
            Output::Failure(x) => Output::Failure(x),
        }
    }
    pub fn ok_io_map<B>(self, f: impl FnOnce(IO<'a, Ok>) -> IO<'a, B>) -> Output<'a, B, Err> {
        match self {
            Output::Success(x) => Output::Success(f(x)),
            Output::Failure(x) => Output::Failure(x),
        }
    }
    pub fn ok_and_then<B>(self, f: impl FnOnce(IO<'a, Ok>) -> Output<'a, B, Err>) -> Output<'a, B, Err> {
        match self {
            Output::Success(x) => f(x),
            Output::Failure(x) => Output::Failure(x),
        }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TRANSFORM NO-MATCH-RELATED COMPONENTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a, Ok, Err> Output<'a, Ok, Err> {
    pub fn err_map<B>(self, f: impl FnOnce(Err) -> B) -> Output<'a, Ok, B> {
        match self {
            Output::Success(x) => Output::Success(x),
            Output::Failure(x) => Output::Failure(x.map(f)),
        }
    }
    pub fn err_and_then<B>(self, f: impl FnOnce(IO<'a, Err>) -> Output<'a, Ok, B>) -> Output<'a, Ok, B> {
        match self {
            Output::Success(x) => Output::Success(x),
            Output::Failure(x) => f(x),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// ALTERNATIVE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

// impl<'a, Val, Err> Output<'a, Val, Err> {
//     pub fn alternatives(
//         options: &'static [&'static dyn Fn(Stream<'a>) -> Output<'a, Val, Err>]
//     ) -> Output<'a, Val, Err> {
//         unimplemented!()
//     }
// }


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SEQUENCING
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a, A, Err> Output<'a, A, Err> {
    pub fn ok_and<B>(self, f: impl FnOnce(Stream<'a>) -> Output<'a, B, Err>) -> Output<'a, (A, B), Err> {
        match self {
            Output::Success(IO { value: a, context }) => match f(context) {
                Output::Success(IO { value: b, context }) => Output::Success(IO { value: (a, b), context }),
                Output::Failure(x) => Output::Failure(x),
            },
            Output::Failure(x) => Output::Failure(x),
        }
    }
    pub fn ok_and2<B, C>(
        self,
        f: impl FnOnce(Stream<'a>) -> Output<'a, B, Err>,
        g: impl FnOnce(Stream<'a>) -> Output<'a, C, Err>
    ) -> Output<'a, (A, B, C), Err> {
        self.ok_and(f).ok_and(g).ok_map(|((a, b), c)| (a, b, c))
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// PEEK
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a, A, Err> Output<'a, A, Err> {
    pub fn ok_try_peek<B>(self, f: impl FnOnce(Stream<'a>) -> Output<'a, B, Err>) -> Output<'a, (A, Option<B>), Err> {
        match self {
            Output::Success(IO { value: a, context }) => {
                let origional = context.clone();
                match f(context) {
                    Output::Success(IO { value: b, context: _ }) => Output::Success(IO { value: (a, Some(b)), context: origional }),
                    Output::Failure(IO { context: _, value: _ }) => Output::Success(IO { value: (a, None), context: origional }),
                }
            },
            Output::Failure(x) => Output::Failure(x),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEBUG
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a, A, Err> Output<'a, A, Err> {
    pub fn ok_inspect(self, f: impl FnOnce(&IO<'a, A>) -> ()) -> Self {
        match self {
            Output::Success(io) => {
                f(&io);
                Output::Success(io)
            }
            Output::Failure(x) => Output::Failure(x),
        }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// INDENT HELPERS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// impl<'a, A, Err> Output<'a, A, Err> {
//     pub fn and_many_indented<B>(self, baseline: Cursor, f: impl FnOnce(Stream<'a>) -> Output<'a, (A, Vec<B>), Err>) {
//         unimplemented!()
//     }
// }

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// MISCELLANEOUS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl<'a, A, Err> Output<'a, A, Err> {
    pub fn try_unwrap_success(self) -> Option<IO<'a, A>> {
        match self {
            Self::Success(x) => Some(x),
            _ => None,
        }
    }
}