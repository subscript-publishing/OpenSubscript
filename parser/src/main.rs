#![allow(unused)]
#![feature(try_trait_v2)]
mod stream;

use std::{rc::Rc, fmt::{Debug, Display}};

pub type RcString = Rc<String>;

#[allow(unused)]
macro_rules! todo {
    ($($x:tt)*) => {
        unimplemented($($x)*)
    };
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub index: usize,
    pub length: usize,
}

impl Range {
    pub fn len(self) -> usize {
        self.length
    }
    pub fn valid_delta(self, delta: usize) -> bool {
        delta <= self.len()
    }
    pub fn seek_forward(self, delta: usize) -> Option<Range> {
        if !self.valid_delta(delta) {
            return None
        }
        Some(Self { index: self.index + delta, length: self.length - delta })
    }
    pub fn slice_range(self) -> std::ops::Range<usize> {
        self.index..self.index+self.length
    }
    // pub fn difference_between(self, other: Delta)
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub index: usize,
    pub column: usize,
    pub line: usize,
}

impl Cursor {
    pub fn forward_sync_for<T: AsRef<char>>(self, span: &[T]) -> Cursor {
        let index = self.index + span.len();
        let mut column = self.column;
        let mut line = self.line;
        for x in span.into_iter().map(|x| *x.as_ref()) {
            if x == '\n' {
                line = line + 1;
                column = 0;
                continue;
            }
            column = column + 1;
        }
        Self { index, column, line }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Span<T> {
    pub source: T,
    pub range: Range,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Indexed<A> {
    pub index: usize,
    pub value: A,
}

impl From<Indexed<char>> for char {
    fn from(value: Indexed<char>) -> Self { value.value }
}
impl AsRef<char> for Indexed<char> {
    fn as_ref(&self) -> &char {
        &self.value
    }
}

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
#[derive(Clone, Copy)]
pub struct CodeViewSlice<'a> {
    pub source: &'a str,
    pub range: Range,
}


pub type CodeViewResult<'a, T> = Result<(T, CodeViewSlice<'a>), CodeViewSlice<'a>>;

impl<'a> CodeViewSlice<'a> {
    pub fn init_root(source: &'a str) -> Self {
        Self {
            range: Range { index: 0, length: source.len() },
            source,
        }
    }
    pub fn len(&self) -> usize { self.range.len() }
    pub fn is_empty(&self) -> bool { self.range.len() == 0 }
    pub fn seek_forward(&self, delta: usize) -> Option<Self> {
        self.range.seek_forward(delta).map(|range| Self {
            source: self.source.clone(),
            range,
        })
    }
    pub fn leading_difference_between(a: CodeViewSlice<'a>, b: CodeViewSlice<'a>) -> Option<CodeViewSlice<'a>> {
        assert!(b.range.index >= a.range.index);
        let range = Range {
            index: a.range.index,
            length: b.range.index - a.range.index,
        };
        Some(CodeViewSlice { source: a.source, range })
    }
    pub fn view(&self) -> &str {
        self.source.get(self.range.slice_range()).unwrap()
    }
}

impl<'a> Debug for CodeViewSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.view())
    }
}
impl<'a> Display for CodeViewSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.view())
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct StreamSlice<'a> {
    pub source: &'a [Indexed<char>],
    pub cursor: Cursor,
}

pub type InContextOpt<V, Ctx> = InContext<Option<V>, Ctx>;

impl<'a> StreamSlice<'a> {
    pub fn uncons(self) -> InContextOpt<Indexed<char>, StreamSlice<'a>> {
        if self.source.is_empty() {
            return InContextOpt{value: None, context: self};
        }
        let item = self.source.first().unwrap().clone();
        let context = &self.source[1..];
        let cursor = self.cursor.forward_sync_for(&[item]);
        let context = StreamSlice{source: context, cursor};
        InContextOpt{value: Some(item), context}
    }
    pub fn take_while_true(self, f: impl Fn(char) -> bool) -> InContextOpt<StreamSlice<'a>, StreamSlice<'a>> {
        for (ix, char) in self.source.into_iter().enumerate() {
            if !f(char.value) {
                let subview = &self.source[0..ix];
                let cursor = self.cursor.forward_sync_for(subview);
                let subview = StreamSlice{source: subview, cursor};
                return InContextOpt{value: Some(subview), context: self}
            }
        }
        InContextOpt{value: None, context: self}
    }
    pub fn take_prefix(self, prefix: &str) -> InContextOpt<StreamSlice<'a>, StreamSlice<'a>> {
        // if prefix.len() > self.source.len()
        let iter1 = self.source.into_iter();
        let iter2 = prefix.chars().into_iter().collect::<Vec<_>>();
        let iter2_len = iter2.len();
        if iter2.len() > iter1.len() {
            return InContextOpt{value: None, context: self}
        }
        for (l, r) in iter1.zip(iter2.into_iter()) {
            if l.value != r {
                return InContextOpt{value: None, context: self}
            }
        }
        let subview = &self.source[0..iter2_len];
        let subview = StreamSlice{
            source: subview,
            cursor: self.cursor.forward_sync_for(subview),
        };
        let context = &self.source[iter2_len..];
        let cursor = self.cursor.forward_sync_for(&self.source[0..iter2_len]);
        let context = StreamSlice{source: context, cursor};
        return InContextOpt{value: Some(subview), context}
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct IO<'a> {
    pub view: CodeViewSlice<'a>
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct Output<'a, A> {
    pub io: IO<'a>,
    pub output: A,
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct Bind<A> {
    pub binder: Rc<dyn Fn(IO) -> Output<A>>,
}

impl<A: 'static + Clone> Bind<A> {
    pub fn pure(value: A) -> Bind<A> {
        Self { binder: Rc::new(move |io| Output { io, output: value.clone() }) }
    }
    pub fn new(binder: impl Fn(IO) -> Output<A> + 'static) -> Bind<A> {
        Self {
            binder: Rc::new(binder),
        }
    }
    pub fn and_then<B: 'static + Clone>(&self, f: impl Fn(A) -> Bind<B> + 'static) -> Bind<B> {
        let this = self.clone();
        Bind::new(move |io| {
            let Output { io, output: value } = this.clone().bind_to(io);
            let binder = f(value);
            let Output { io, output: value } = binder.bind_to(io);
            Output { io, output: value }
        })
    }
    pub fn map<B: 'static + Clone>(&self, f: impl Fn(A) -> B + 'static) -> Bind<B> {
        let this = self.clone();
        Bind::new(move |io| {
            let Output { io, output: value } = this.clone().bind_to(io);
            let value = f(value.clone());
            Output { io, output: value }
        })
    }
    pub fn update(&self, f: impl Fn(IO) -> IO + 'static) -> Bind<A> {
        let this = self.clone();
        Bind::new(move |io| {
            let Output { io, output: value } = this.clone().bind_to(io);
            let io = f(io);
            Output { io, output: value }
        })
    }
    pub fn trans<B: 'static + Clone>(&self, f: impl Fn(A, IO) -> (B, IO) + 'static) -> Bind<B> {
        let this = self.clone();
        Bind::new(move |io| {
            let Output { io, output: value } = this.clone().bind_to(io);
            let (value, io) = f(value.clone(), io);
            Output { io, output: value }
        })
    }
}

impl<A> Bind<A> {
    fn bind_to(self, io: IO) -> Output<A> {
        (self.binder)(io)
    }
}

impl<A> Clone for Bind<A> {
    fn clone(&self) -> Self {
        Self { binder: self.binder.clone() }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

#[derive(Clone)]
pub enum Parser<A> {
    Binder {
        binder: Rc<dyn Fn(IO) -> Option<Output<A>>>,
    },
    Terminate,
}

impl<A> Parser<A> {
    fn bind_to(self, io: IO) -> Option<Output<A>> {
        match self {
            Self::Binder { binder } => (binder)(io),
            Self::Terminate => None,
        }
    }
}

impl<A: 'static + Clone> Parser<A> {
    pub fn pure(value: A) -> Self {
        Self::Binder { binder: Rc::new(move |io| Some(Output { io, output: value.clone() })) }
    }
    pub fn maybe_io(binder: impl Fn(IO) -> Option<Output<A>> + 'static) -> Parser<A> {
        Self::Binder { binder: Rc::new(binder) }
    }
    pub fn just_io(binder: impl Fn(IO) -> Output<A> + 'static) -> Parser<A> {
        Self::Binder { binder: Rc::new(move |io| Some(binder(io))) }
    }
    pub fn and_then<B: 'static + Clone>(self, f: impl Fn(A) -> Parser<B> + 'static) -> Parser<B> {
        match self {
            Self::Binder { binder } => {
                Parser::maybe_io(move |io| {
                    let Output { io, output: value } = (binder)(io)?;
                    let binder = f(value);
                    let Output { io, output: value } = binder.bind_to(io)?;
                    Some(Output { io, output: value })
                })
            },
            Self::Terminate => Parser::Terminate,
        }
    }
}


// pub mod parse {
//     use crate::*;

//     pub fn identifier(stream: StreamSlice) -> StreamSlice {

//     }
// }


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

fn main() {
    stream::dev1();
    // let source_view = CodeView::init_root(source_code_1);
    // println!("RESULT: {:?}", source_view.seek_forward(10));
    // for (ix, char) in source_code_1.char_indices() {
    //     let result = source_view.seek_forward(ix).unwrap();
    //     // let diff = result.difference_between(source_view);
    //     let diff = CodeView::leading_difference_between(source_view, result);
    //     println!("[{char}]\tresult [{ix}]: {diff:?}\tfor {result:?}");
    // }
    // let start: usize = 0;
    // let length: usize = source_code_1.len();
    // let result = source_code_1.get(source_view.range.slice_range());
    // println!("RESULT: {:?}", );
}
