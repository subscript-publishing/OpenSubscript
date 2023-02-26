use std::{fmt::Display, collections::VecDeque};

use super::{IndexedChar, Bucket};
use itertools::Itertools;
use tree_formatter::{DisplayTree, ToDisplayTree};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum BlockNode {
    Bucket(Bucket),
    InCurlyBraces {
        open: IndexedChar,
        content: Vec<BlockNode>,
        close: IndexedChar,
    },
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl BlockNode {
    pub fn process_sublevel(self, open: IndexedChar, level: &mut VecDeque<BlockNode>) {
        match self {
            BlockNode::Bucket(Bucket::CloseBracket(close)) if open.is_valid_close_token(close) => {}
            _ => unimplemented!()
        }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Display for BlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bucket(x) => {
                let format = format!("{x}");
                f.write_str(&format)
            }
            Self::InCurlyBraces { open, content, close } => {
                let format = format!("{open}{content:?}{close}");
                f.write_str(&format)
            },
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl ToDisplayTree for BlockNode {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Bucket(x) => {
                x.to_display_tree()
            }
            Self::InCurlyBraces { open, content, close } => {
                let children = content.into_iter().map(|x| x.to_display_tree()).collect_vec();
                DisplayTree::branch(
                    format!("InCurlyBraces"),
                    children
                )
            },
        }
    }
}
