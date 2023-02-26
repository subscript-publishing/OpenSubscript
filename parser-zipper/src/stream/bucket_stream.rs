use std::collections::VecDeque;
use itertools::Itertools;
use crate::basics::{Bucket, char_utils, IndexedString, IndexedChar};
use super::StreamZipper;

impl StreamZipper<Bucket> {
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
        match self.focus.take() {
            Some(Bucket::Pipe2(o1, o2)) => {
                match self.consume_until_close_pipe2() {
                    Some((contents, (c1, c2))) => {
                        let new_bucket = Bucket::SectionHeader {
                            open_p1: o1,
                            open_p2: o2,
                            contents,
                            close_p1: c1,
                            close_p2: c2,
                        };
                        self.focus = Some(new_bucket);
                    }
                    None => {
                        self.focus = Some(Bucket::Pipe2(o1, o2));
                    }
                }
            }
            Some(Bucket::Backslash2(o1, o2)) => {
                match self.consume_until_close_backslash2() {
                    Some((contents, (c1, c2))) => {
                        let new_bucket = Bucket::SectionHeader {
                            open_p1: o1,
                            open_p2: o2,
                            contents,
                            close_p1: c1,
                            close_p2: c2,
                        };
                        self.focus = Some(new_bucket);
                    }
                    None => {
                        self.focus = Some(Bucket::Backslash2(o1, o2));
                    }
                }
            }
            x => {
                self.focus = x;
            }
        }
    }
    pub fn consume_until_close_pipe2(&mut self) -> Option<(IndexedString, (IndexedChar, IndexedChar))> {
        let origional = self.trailing.clone();
        let mut contents: IndexedString = Default::default();
        while let Some(x) = self.trailing.pop_front() {
            match x {
                Bucket::Pipe2(x, y) => {
                    return Some((contents, (x, y)))
                }
                x => {
                    contents.append(x.into_string());
                }
            }
        }
        self.trailing = origional;
        None
    }
    pub fn consume_until_close_backslash2(&mut self) -> Option<(IndexedString, (IndexedChar, IndexedChar))> {
        let origional = self.trailing.clone();
        let mut contents: IndexedString = Default::default();
        while let Some(x) = self.trailing.pop_front() {
            match x {
                Bucket::Backslash2(x, y) => {
                    return Some((contents, (x, y)))
                }
                x => {
                    contents.append(x.into_string());
                }
            }
        }
        self.trailing = origional;
        None
    }
    pub fn next_two(&mut self) -> Option<(Bucket, Bucket)> {
        if self.trailing.len() >= 2 {
            let first = self.trailing.pop_front().unwrap();
            let second = self.trailing.pop_front().unwrap();
            return Some((first, second))
        }
        None
    }
}

pub fn dev_pipeline1() -> StreamZipper<Bucket> {
    let source_code = include_str!("../../samples/input/file1.txt");
    let buffer = char_utils::to_indexed_chars(source_code)
        .into_iter()
        .map(|x| Bucket::new(x))
        .coalesce(Bucket::coalesce);
    let buffer = VecDeque::from_iter(buffer);
    let buffer_len = buffer.len();
    let mut zipper = StreamZipper{
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

pub fn dev1() {
    let source_code = include_str!("../../samples/input/file1.txt");
    let buffer = char_utils::to_indexed_chars(source_code)
        .into_iter()
        .map(|x| Bucket::new(x))
        .coalesce(Bucket::coalesce);
    let buffer = VecDeque::from_iter(buffer);
    let buffer_len = buffer.len();
    let mut zipper = StreamZipper{
        leading: Default::default(),
        focus: None,
        trailing: buffer,
    };
    for _ in 0..=buffer_len {
        let _ = zipper.process();
        let _ = zipper.process_special();
        zipper.forward();
    }
    for x in zipper.into_vec() {
        println!("{x}");
    }
}
