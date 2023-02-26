use std::collections::VecDeque;

use itertools::Itertools;
use tree_formatter::ToDisplayTree;

use crate::{Zipper, SrcChar, SrcUnit, SrcString};

pub fn string_input_pipeline(source_code: &str) -> Zipper<SrcUnit> {
    let buffer = SrcChar::to_indexed_chars(source_code)
        .into_iter()
        .map(|x| SrcUnit::new(x))
        .coalesce(SrcUnit::coalesce);
    let buffer = VecDeque::from_iter(buffer);
    let buffer_len = buffer.len();
    let mut zipper = Zipper{
        leading: Default::default(),
        focus: None,
        trailing: buffer,
    };
    for _ in 0..=buffer_len {
        let _ = zipper.process();
        let _ = zipper.process_special();
        zipper.forward();
    }
    zipper
}

pub fn debug_print(source_code: &str) {
    let output = string_input_pipeline(source_code);
    output.to_display_tree().pretty_print();
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Zipper<SrcUnit> {
    pub fn process(&mut self) {
        match (self.focus.take(), self.next_two()) {
            (Some(l), Some((c, r))) => {
                match l.coalesce2(c, r) {
                    Ok(x) => {
                        self.focus = Some(x);
                    }
                    Err((l, c, r)) => {
                        self.focus = Some(l);
                        self.trailing.push_front(r);
                        self.trailing.push_front(c);
                    }
                }
            }
            (None, Some((c, r))) => {
                self.trailing.push_front(r);
                self.trailing.push_front(c);
            }
            (Some(l), None) => {
                self.focus = Some(l);
            }
            (None, None) => {}
        }
    }
    pub fn process_special(&mut self) {
        // match self.focus.take() {
        //     Some(SrcUnit::Pipe2(open)) => {
        //         match self.consume_until_close_pipe2() {
        //             Some((contents, (c1, c2))) => {
        //                 // let new_bucket = InputUnit::SectionHeader {
        //                 //     open_p1: o1,
        //                 //     open_p2: o2,
        //                 //     contents,
        //                 //     close_p1: c1,
        //                 //     close_p2: c2,
        //                 // };
        //                 let new_bucket = SrcUnit::SectionHeader(crate::SomeSectionHeader {
        //                     open,
        //                     contents,
        //                     close: (c1, c2).into(),
        //                 });
        //                 self.focus = Some(new_bucket);
        //             }
        //             None => {
        //                 self.focus = Some(SrcUnit::Pipe2(open));
        //             }
        //         }
        //     }
        //     Some(SrcUnit::Backslash2(open)) => {
        //         match self.consume_until_close_backslash2() {
        //             Some((contents, (c1, c2))) => {
        //                 // let new_bucket = InputUnit::SectionHeader {
        //                 //     open_p1: o1,
        //                 //     open_p2: o2,
        //                 //     contents,
        //                 //     close_p1: c1,
        //                 //     close_p2: c2,
        //                 // };
        //                 let new_bucket = SrcUnit::SectionHeader(crate::SomeSectionHeader {
        //                     open,
        //                     contents,
        //                     close: (c1, c2).into(),
        //                 });
        //                 self.focus = Some(new_bucket);
        //             }
        //             None => {
        //                 self.focus = Some(SrcUnit::Backslash2(open));
        //             }
        //         }
        //     }
        //     x => {
        //         self.focus = x;
        //     }
        // }
        eprintln!("OLD STUFF - USE NEW SYSTEM")
    }
    pub fn consume_until_close_pipe2(&mut self) -> Option<(SrcString, (SrcChar, SrcChar))> {
        let origional = self.trailing.clone();
        let mut contents: SrcString = Default::default();
        while let Some(x) = self.trailing.pop_front() {
            match x {
                SrcUnit::Pipe2(a) => {
                    return Some((contents, a.to_tuple()))
                }
                x => {
                    contents.append(x.istring());
                }
            }
        }
        self.trailing = origional;
        None
    }
    pub fn consume_until_close_backslash2(&mut self) -> Option<(SrcString, (SrcChar, SrcChar))> {
        let origional = self.trailing.clone();
        let mut contents: SrcString = Default::default();
        while let Some(x) = self.trailing.pop_front() {
            match x {
                SrcUnit::Backslash2(a) => {
                    return Some((contents, a.to_tuple()))
                }
                x => {
                    contents.append(x.istring());
                }
            }
        }
        self.trailing = origional;
        None
    }
    pub fn next_two(&mut self) -> Option<(SrcUnit, SrcUnit)> {
        if self.trailing.len() >= 2 {
            let first = self.trailing.pop_front().unwrap();
            let second = self.trailing.pop_front().unwrap();
            return Some((first, second))
        }
        None
    }
}