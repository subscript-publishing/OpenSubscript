use std::rc::Rc;

use crate::{stream::{Stream, Cursor}, output::{Output, IO}, character::{TakeWhileSpec, CharView}, token::TokenView};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// STREAM PARSER INTERFACE
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub trait StreamBinder {
    type Ok<'a> where Self: 'a;
    type Err;
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok<'a>, Self::Err>;
}
pub trait GeneralStreamBinder {
    type Ok;
    type Err;
    fn bind_to<'a>(self, stream: Stream<'a>) -> Output<'a, Self::Ok, Self::Err>;
}

impl<'a> Stream<'a> {
    pub fn apply_binder<Op: StreamBinder>(self, binder: Op) -> Output<'a, Op::Ok<'a>, Op::Err> {
        binder.bind_to(self)
    }
    pub fn apply_general_binder<Op: GeneralStreamBinder>(self, binder: Op) -> Output<'a, Op::Ok, Op::Err> {
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
    pub fn apply_compute_binder<Op: ComputeStreamBinder>(self, binder: Op) -> Output<'a, Op::Ok, Op::Err> {
        match binder.compute_bind_to(self.clone()) {
            Output::Success(IO { context: _, value }) => Output::Success(IO { context: self, value }),
            Output::Failure(IO { context: _, value }) => Output::Failure(IO { context: self, value }),
        }
    }
}


