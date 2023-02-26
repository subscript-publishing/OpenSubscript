use crate::{binders::StreamBinder, output::Output, stream::{Stream, IndexedChar}};

use super::{
    cmd::{PipeCmd, BackslashCmd},
    plain_text::{ParsePlainText, PlainText},
};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum RootAst<'a> {
    BackslashCmd(Box<BackslashCmd<'a>>),
    PipeCmd(Box<PipeCmd<'a>>),
    PlainText(PlainText<'a>),
    CharError(IndexedChar),
}
