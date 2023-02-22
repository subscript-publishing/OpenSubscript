use crate::{stream::{IndexedChar, Stream, Cursor}, token::TokenView, syntax::RootAst, binders::ComputeStreamBinder, output::{Output, IO}, character::{TakeWhileSpec, UnconsSpec}};

use super::InSquareBrackets;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub enum IndentInfo {
    Asterisk { cursor: Cursor }
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
                Output::success(IO { context: stream, value: () })
            }
        };
        context
            .ok_and_then(|IO { value: _, context }| -> Output<'a, _, _> {
                context.apply_binder(TakeWhileSpec::WHITE_SPACE)
            })
            .ok_and_then(|IO { value: _, context }| -> Output<'a, Self::Ok, Self::Err> {
                let cursor = context.cursor;
                context.apply_binder(UnconsSpec::must_match('*')).ok_map(|_| IndentInfo::Asterisk { cursor })
            })
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// BASICS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct AstLine<'a> {
    pub content: RootAst<'a>,
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub enum SomeIndentedListItem<'a> {
    Asterisk(AsteriskListItem<'a>),
    Labeled(LabeledListItem<'a>),
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct AsteriskListItem<'a> {
    pub asterisk_token: IndexedChar,
    pub content: AstLine<'a>,
}
pub struct LabeledListItem<'a> {
    pub label: InSquareBrackets<TokenView<'a>>,
    pub content: AstLine<'a>,
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub enum SomeIndentedList<'a> {
    AsteriskList {
        enumerated: Vec<AsteriskListItem<'a>>,
    },
    LabeledList {
        enumerated: Vec<LabeledListItem<'a>>,
    },
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct IndentedBlock<'a> {
    pub content: Vec<RootAst<'a>>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub enum SomeIndentedChunk<'a> {
    Block(IndentedBlock<'a>),
    List(SomeIndentedList<'a>),
}
