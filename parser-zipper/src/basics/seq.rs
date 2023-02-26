use std::{collections::VecDeque, rc::Rc, fmt::{Debug, format}};

use itertools::Itertools;
use tree_formatter::{ToDisplayTree, DisplayTree};

// use crate::pipeline::nested_seq_pass::TopLevel;

use super::Bucket;


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Seq<T> {
    pub sequence: VecDeque<T>
}

impl<T> Seq<T> {
    pub fn from_iter(xs: impl IntoIterator<Item=T>) -> Self {
        Self { sequence: xs.into_iter().collect() }
    }
    pub fn subseq_for_all_true(&mut self, predicate: impl Fn(&T) -> bool) -> Seq<T> {
        let mut leading: VecDeque<T> = Default::default();
        while let Some(item) = self.sequence.pop_front() {
            if predicate(&item) {
                leading.push_front(item);
            } else {
                self.sequence.push_front(item);
                return Seq::from_iter(leading)
            }
        }
        return Seq::from_iter(leading)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum NestedSeq<T> {
    Enclosed {
        header: T,
        content: Box<NestedSeq<T>>,
        closed: ClosedManager<T>,
    },
    Branch(Box<NestedSeq<T>>, Box<NestedSeq<T>>),
    Leaf(T),
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub enum ClosedManager<T> {
    Closed(T),
    Unclosed {
        manager: Rc<dyn Fn(&T) -> bool>
    },
}
impl<T> Debug for ClosedManager<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed(x) => x.fmt(f),
            Self::Unclosed { .. } => f.write_str("Unclosed"),
        }
    }
}

impl<T> ClosedManager<T> {
    pub fn evaluate(&mut self, node: T) -> Option<T> {
        match self {
            Self::Unclosed { manager } if (manager.as_ref())(&node) => {
                *self = ClosedManager::Closed(node);
                None
            }
            _ => Some(node)
        }
    }
    pub fn is_closed(&self) -> bool {
        match self { Self::Closed(_) => true, _ => false}
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T> NestedSeq<T> {
    pub fn push_primitive(&mut self, node: T) where T: Clone {
        match self {
            Self::Branch(l, r) => {
                r.push_primitive(node);
            }
            Self::Enclosed { header, content, closed } => {
                if let Some(unused) = closed.evaluate(node) {
                    content.push_primitive(unused);
                }
            }
            Self::Leaf(x) => {
                *self = Self::Branch(
                    Box::new(Self::Leaf(x.clone())),
                    Box::new(Self::Leaf(node)),
                )
            }
        }
    }
}

// impl NestedSeq<TopLevel> {
//     pub fn push_bucket(&mut self, item: Bucket) {
        
//     }
//     pub fn new_branch(&mut self) {

//     }
// }


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SEQ - DEBUG
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// impl<T: ToDisplayTree> ToDisplayTree for Seq<T> {
//     fn to_display_tree(&self) -> tree_formatter::DisplayTree {
//         let label = "Seq";
//         let children = self.sequence.iter().map(|x| x.to_display_tree()).collect_vec();
//         DisplayTree::branch(label, children)
//     }
// }
// impl<T: ToDisplayTree> ToDisplayTree for NestedSeq<T> {
//     fn to_display_tree(&self) -> tree_formatter::DisplayTree {
//         match self {
//             Self::Empty => DisplayTree::leaf("Empty"),
//             Self::Leaf(xs) => xs.to_display_tree(),
//             Self::Branch { header, content, closed } if closed.is_closed() => {
//                 let label = format!("ClosedBranch");
//                 let child1 = DisplayTree::branch("Header", vec![header.to_display_tree()]);
//                 let child2 = DisplayTree::branch("Content", vec![content.to_display_tree()]);
//                 DisplayTree::branch(label, [child1, child2])
//             }
//             Self::Branch { header, content, closed } => {
//                 let label = format!("OpenBranch");
//                 let child1 = DisplayTree::branch("Header", vec![header.to_display_tree()]);
//                 let child2 = DisplayTree::branch("Content", vec![content.to_display_tree()]);
//                 DisplayTree::branch(label, [child1, child2])
//             }
//         }
//     }
// }

