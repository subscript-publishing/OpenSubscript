use crate::binders::{ComputeStreamBinder, PhantomLifetime, StreamBinder};
use crate::character::{StaticCharPredicate, TakeWhileSpec, UnconsSpec};
use crate::output::{Output, IO};
use crate::stream::Cursor;
use crate::stream::{IndexedChar, Stream};
use crate::token::TokenView;
use std::rc::Rc;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub enum IndentInfo {
    Asterisk { cursor: Cursor },
}

impl IndentInfo {
    pub fn cursor(&self) -> Cursor {
        match self {
            Self::Asterisk { cursor } => *cursor,
        }
    }
    pub fn is_indented_from(&self, start: Cursor) -> bool {
        let this_cursor = self.cursor();
        let check1 = this_cursor.column > start.column;
        let check2 = this_cursor.line > start.line;
        check1 && check2
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComputeIndentLevel {
    pub parse_start_newline: bool,
}

impl ComputeStreamBinder for ComputeIndentLevel {
    type Ok = IndentInfo;
    type Err = ();
    fn compute_bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok, Self::Err> {
        let context = {
            if self.parse_start_newline {
                stream.apply_binder(TakeWhileSpec::NEWLINE).ok_map(|_| ())
            } else {
                Output::success(IO {
                    context: stream,
                    value: (),
                })
            }
        };
        context
            .ok_and_then(|IO { value: _, context }| -> Output<'a, _, _> {
                context.apply_binder(TakeWhileSpec::WHITESPACE)
            })
            .ok_and_then(
                |IO { value: _, context }| -> Output<'a, Self::Ok, Self::Err> {
                    let cursor = context.cursor;
                    context
                        .apply_binder(UnconsSpec::must_match('*'))
                        .ok_map(|_| IndentInfo::Asterisk { cursor })
                },
            )
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct IndentedAsteriskItem<'a> {
    pub asterisk: IndexedChar,
    pub line: TokenView<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseIndentedAsteriskItem {
    pub indent_start: Cursor,
}

impl StreamBinder for ParseIndentedAsteriskItem {
    type Ok<'a> = IndentedAsteriskItem<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream
            .apply_binder(TakeWhileSpec::WHITESPACE)
            .ok_and_then(
                |IO { value: _, context }| -> Output<'a, IndexedChar, Self::Err> {
                    if context.at_column(self.indent_start.column) {
                        context.apply_binder(UnconsSpec::must_match('*'))
                    } else {
                        Output::failure(IO::no_op(context))
                    }
                },
            )
            .ok_and_then(
                |IO {
                     value: asterisk,
                     context,
                 }|
                 -> Output<'a, (IndexedChar, TokenView<'a>), Self::Err> {
                    context
                        .apply_binder(TakeWhileSpec::NOT_NEWLINE)
                        .ok_map(|ws| (asterisk, ws))
                },
            )
            .ok_map(|(asterisk, line)| IndentedAsteriskItem { asterisk, line })
    }
}

#[derive(Clone)]
pub struct ParseIndentedStart {
    pub baseline: Cursor,
    pub indent_info: IndentInfo,
}
impl ParseIndentedStart {
    pub fn new(indent_start: Cursor, indent_info: IndentInfo) -> Self {
        Self {
            indent_info,
            baseline: indent_start,
        }
    }
}
impl StreamBinder for ParseIndentedStart {
    type Ok<'a> = (IndexedChar, PhantomLifetime<'a>);
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        stream.apply_binder(TakeWhileSpec::WHITESPACE).ok_and_then(
            |IO { value: _, context }| -> Output<'a, Self::Ok<'a>, Self::Err> {
                if context.at_column(self.indent_info.cursor().column) {
                    context
                        .apply_binder(UnconsSpec::must_match('*'))
                        .ok_map(|x| (x, PhantomLifetime::default()))
                } else {
                    Output::failure(IO::no_op(context))
                }
            },
        )
    }
}

#[derive(Clone)]
pub struct ParseIndentedBody {
    pub baseline: Cursor,
    pub indent_info: IndentInfo,
}
impl ParseIndentedBody {
    pub fn new(indent_start: Cursor, indent_info: IndentInfo) -> Self {
        Self {
            indent_info,
            baseline: indent_start,
        }
    }
}
impl StreamBinder for ParseIndentedBody {
    type Ok<'a> = TokenView<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, TokenView<'a>, Self::Err> {
        stream.apply_binder(TakeWhileSpec::NOT_NEWLINE)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct IndentedList<'a> {
    pub header: TokenView<'a>,
    pub colon: IndexedChar,
    pub items: Vec<(IndexedChar, TokenView<'a>)>,
}

#[derive(Debug, Clone, Default)]
pub struct ParseIndentedList {}

impl StreamBinder for ParseIndentedList {
    type Ok<'a> = IndentedList<'a>;
    type Err = ();
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err> {
        // let start_cursor = stream.cursor;
        // stream
        //     .apply_binder(TakeWhileSpec::IDENTIFIER)
        //     .ok_and(|stream| stream.apply_binder(UnconsSpec::must_match(':')))
        //     .ok_and(|stream| stream.apply_binder(UnconsSpec::with_filter(StaticCharPredicate::IS_NEWLINE)))
        //     .ok_and(|stream| stream.apply_compute_binder(ComputeIndentLevel{parse_start_newline: false}))
        //     .ok_map(|(((a, b), c), d)| { (a, b, c, d) })
        //     .ok_and_then(|IO { context, value: (a, b, _, indent_info) }| -> Output<'a, Self::Ok<'a>, Self::Err> {
        //         context
        //             .parse_many_sequenced(
        //                 ParseManySequenced{
        //                     start_parser: ParseIndentedStart::new(
        //                         start_cursor.clone(),
        //                         indent_info.clone(),
        //                     ),
        //                     indent_body: ParseIndentedBody::new(
        //                         start_cursor.clone(),
        //                         indent_info.clone(),
        //                     ),
        //                     sep_by: UnconsSpec::with_filter(StaticCharPredicate::IS_NEWLINE),
        //                 }
        //             )
        //             .ok_map(|xs| {
        //                 let xs = xs.into_iter().map(|(a, b, _)| (a, b)).collect::<Vec<_>>();
        //                 IndentedList { header: a, colon: b, items: xs }
        //             })
        //     })
        unimplemented!()
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Clone, Copy)]
pub struct ParseManySequenced<A, B, C>
where
    A: StreamBinder + Clone,
    B: StreamBinder + Clone,
    C: StreamBinder + Clone,
{
    pub start_parser: A,
    pub indent_body: B,
    pub sep_by: C,
}

impl<'a> Stream<'a> {
    pub fn parse_many_sequenced<
        A: StreamBinder + Clone,
        B: StreamBinder + Clone,
        C: StreamBinder + Clone,
    >(
        self,
        ParseManySequenced {
            start_parser,
            indent_body,
            sep_by,
        }: ParseManySequenced<A, B, C>,
    ) -> Output<'a, Vec<(A::Ok<'a>, B::Ok<'a>, Option<C::Ok<'a>>)>, ()> {
        let mut results: Vec<(A::Ok<'a>, B::Ok<'a>, Option<C::Ok<'a>>)> = Default::default();
        let mut stream: Stream<'a> = self;
        while let Some(IO {
            context: start,
            value: a,
        }) = start_parser
            .clone()
            .bind_to(stream.clone())
            .try_unwrap_success()
        {
            if let Some(IO {
                context: mid,
                value: b,
            }) = indent_body
                .clone()
                .bind_to(start.clone())
                .try_unwrap_success()
            {
                if let Some(IO {
                    context: end,
                    value: c,
                }) = sep_by.clone().bind_to(mid.clone()).try_unwrap_success()
                {
                    stream = end;
                    results.push((a, b, Some(c)));
                    continue;
                }
                stream = mid;
                results.push((a, b, None));
                continue;
            }
            break;
        }
        Output::success(IO {
            context: stream,
            value: results,
        })
    }
}
