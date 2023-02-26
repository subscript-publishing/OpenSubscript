use std::{marker::PhantomData, rc::Rc};

use crate::{
    character::{CharView, TakeWhileSpec},
    output::{Output, IO},
    stream::{Cursor, Stream},
    token::TokenView,
};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// BASICS - MISCELLANEOUS UTILS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub struct PhantomLifetime<'a>(PhantomData<&'a ()>);

impl<'a> Default for PhantomLifetime<'a> {
    fn default() -> Self {
        PhantomLifetime(PhantomData::default())
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// BINDABLE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// STREAM PARSER INTERFACE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub trait StreamBinder {
    type Ok<'a> where Self: 'a;
    type Err;
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err>;
}

impl<'a> Stream<'a> {
    pub fn apply_binder<Op: StreamBinder>(self, binder: Op) -> Output<'a, Op::Ok<'a>, Op::Err> {
        binder.bind_to(self)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// STREAM PARSER INTERFACE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub trait ComputeStreamBinder {
    type Ok;
    type Err;
    fn compute_bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok, Self::Err>;
}

impl<'a> Stream<'a> {
    pub fn apply_compute_binder<Op: ComputeStreamBinder>(
        self,
        binder: Op,
    ) -> Output<'a, Op::Ok, Op::Err> {
        match binder.compute_bind_to(self.clone()) {
            Output::Success(IO { context: _, value }) => Output::Success(IO {
                context: self,
                value,
            }),
            Output::Failure(IO { context: _, value }) => Output::Failure(IO {
                context: self,
                value,
            }),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<'a> Stream<'a> {
    pub fn apply_optional_binder<Op: StreamBinder>(
        self,
        binder: Op,
    ) -> Output<'a, Option<Op::Ok<'a>>, Op::Err> {
        match binder.bind_to(self.clone()) {
            Output::Success(IO { context, value }) => Output::Success(IO {
                context,
                value: Some(value),
            }),
            Output::Failure(IO {
                context: _,
                value: _,
            }) => Output::Success(IO {
                context: self,
                value: None,
            }),
        }
    }
}
