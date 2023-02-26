#![allow(unused)]
pub mod full_zipper;
pub mod stream;
pub mod basics;
pub mod pipeline;
pub mod ast;

use std::{collections::VecDeque, fmt::{Display, Write, format}};

use basics::{IndexedChar, IndexedString};
use itertools::Itertools;



pub fn main() {
    // stream::dev1();
    // stream::bucket_stream::dev1();
    // stream::block_stream::dev1();
    pipeline::tree_builder_pass::dev1();
}