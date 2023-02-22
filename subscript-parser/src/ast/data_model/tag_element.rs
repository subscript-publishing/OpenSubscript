use std::rc::Rc;

use crate::output::IO;
use crate::character::{UnconsSpec, StaticCharPredicate};
use crate::character::TakeWhileSpec;
use crate::output::Output;
use crate::binders::{StreamBinder, ComputeStreamBinder, GeneralStreamBinder};
use crate::token::TokenView;
use crate::stream::{IndexedChar, Stream, Cursor};

use super::InCurlyBrackets;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TAG - START
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum BeginTagHeader {
    Backslash {
        backslash_char: IndexedChar,
    },
    Pipe {
        pipe_char: IndexedChar,
    },
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum EndTagHeader<'a> {
    Colon {
        colon_char: IndexedChar,
        rest_of_line: TokenView<'a>,
    },
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagHeader<'a> {
    pub begin_type: BeginTagHeader,
    pub identifier: TokenView<'a>,
    // pub attributes: Option<InSquareBrackets<'a>>,
    pub end_type: Option<EndTagHeader<'a>>,
}



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// pub struct EnclosedTagBody<'a> {}
// pub struct IndentedTagBody<'a> {}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub enum TagBody<'a> {
    Indented {

    },
    Enclosed {
        body: InCurlyBrackets<'a>
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct TagElement<'a> {
    pub header: TagHeader<'a>,
    // pub body: 
}

