#![allow(unused)]
mod ast;
mod pretty;

use std::collections::VecDeque;

pub use ast::*;
pub use pretty::*;


impl<T> ToDisplayTree for VecDeque<T> where T: ToDisplayTree {
    fn to_display_tree(&self) -> DisplayTree {
        let xs = self.iter().map(|x| x.to_display_tree()).collect::<Vec<_>>();
        DisplayTree::fragment(xs)
    }
}
impl<T> ToDisplayTree for Option<T> where T: ToDisplayTree {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::None => DisplayTree::leaf("None"),
            Self::Some(x) => x.to_display_tree(),
        }
    }
}

