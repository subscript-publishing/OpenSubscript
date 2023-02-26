use std::collections::VecDeque;

use itertools::Itertools;

use crate::{Zipper, SrcGroup, SrcChar, SomePrimitive, zipper::TakeWhilePred, SomeSectionHeader};

impl Zipper<SrcGroup> {
    pub fn for_each_focus(&mut self) {
        self.try_coalesce_focus();
        self.try_coalesce_focus2();
        match self.focus.take() {
            Some(SrcGroup::SomePrimitive(SomePrimitive::Pipe2(open))) => {
                let results = self.take_where(|x| {
                    match x {
                        SrcGroup::SomePrimitive(SomePrimitive::Pipe2(x)) => {
                            if open.semantically_equal_to(x) {
                                TakeWhilePred::TerminateOk { include_take_this: Some(*x) }
                            } else {
                                TakeWhilePred::TerminateErr
                            }
                        },
                        SrcGroup::Newline(_) => TakeWhilePred::TerminateErr,
                        _ => TakeWhilePred::Continue,
                    }
                });
                match results {
                    Ok((xs, close)) => {
                        let close = close.unwrap();
                        let new = SomeSectionHeader {
                            open,
                            close,
                            contents: xs,
                        };
                        self.focus = Some(SrcGroup::SectionHeader(new));
                    }
                    Err(_) => {
                        self.focus = Some(SrcGroup::SomePrimitive(SomePrimitive::Pipe2(open)));
                    }
                }
            }
            Some(SrcGroup::SomePrimitive(SomePrimitive::Backslash2(open))) => {
                let results = self.take_where(|x| {
                    match x {
                        SrcGroup::SomePrimitive(SomePrimitive::Backslash2(x)) => {
                            if open.semantically_equal_to(x) {
                                TakeWhilePred::TerminateOk { include_take_this: Some(*x) }
                            } else {
                                TakeWhilePred::TerminateErr
                            }
                        },
                        SrcGroup::Newline(_) => TakeWhilePred::TerminateErr,
                        _ => TakeWhilePred::Continue,
                    }
                });
                match results {
                    Ok((xs, close)) => {
                        let close = close.unwrap();
                        let new = SomeSectionHeader {
                            open,
                            close,
                            contents: xs,
                        };
                        self.focus = Some(SrcGroup::SectionHeader(new));
                    }
                    Err(_) => {
                        self.focus = Some(SrcGroup::SomePrimitive(SomePrimitive::Backslash2(open)));
                    }
                }
            }
            x => {
                self.focus = x;
            }
        }
    }
}


pub fn run_tokenizer(source_code: &str) -> Vec<SrcGroup> {
    let buffer = SrcChar::to_indexed_chars(source_code)
        .into_iter()
        .map(|x| SrcGroup::from_src_char(x))
        .coalesce(SrcGroup::coalesce);
    let buffer = VecDeque::from_iter(buffer);
    let buffer_len = buffer.len();
    let mut zipper = Zipper{
        leading: Default::default(),
        focus: None,
        trailing: buffer,
    };
    for _ in 0 ..= buffer_len + 1 {
        zipper.for_each_focus();
        zipper.forward();
    }
    // zipper.to_display_tree().pretty_print();
    zipper.to_vec()
}

