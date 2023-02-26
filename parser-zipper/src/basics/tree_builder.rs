use std::collections::VecDeque;

use itertools::Itertools;
use tree_formatter::{ToDisplayTree, DisplayTree};


pub type BoxedTree<T> = Box<TreeBuilder<T>>;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum TreeBuilder<T> {
    Empty,
    Leaf(T),
    // Branch(BoxedTree<T>, BoxedTree<T>),
    Fragment(VecDeque<TreeBuilder<T>>),
    Open(Box<OpenEnclosure<T>>),
    Closed(Box<ClosedEnclosure<T>>),
    Indented(Box<IndentedBlock<T>>)
}

#[derive(Debug, Clone)]
pub struct OpenEnclosure<T> {
    pub open: T,
    pub content: TreeBuilder<T>,
}

#[derive(Debug, Clone)]
pub struct ClosedEnclosure<T> {
    pub open: T,
    pub content: TreeBuilder<T>,
    pub close: T,
}

#[derive(Debug, Clone)]
pub struct IndentedBlock<T> {
    pub content: TreeBuilder<T>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T> TreeBuilder<T> {
    pub fn fragment(xs: impl IntoIterator<Item=TreeBuilder<T>>) -> Self {
        Self::Fragment(FromIterator::from_iter(xs))
    }
    pub fn has_open(&self) -> bool {
        match self {
            // Self::Branch(l, r) => r.has_open(),
            Self::Fragment(xs) => {
                // xs.last().map(|x| x.has_open()).unwrap_or(false)
                xs.back().map(|x| x.has_open()).unwrap_or(false)
            }
            Self::Open(_) => true,
            Self::Closed(_) => false,
            Self::Leaf(_) => false,
            Self::Empty => false,
            Self::Indented(x) => x.content.has_open()
        }
    }
    pub fn insert_primitive(&mut self, node: T) where T: Clone {
        match self {
            Self::Fragment(xs) => {
                if let Some(r) = xs.back_mut() {
                    r.insert_primitive(node)
                } else {
                    xs.push_back(Self::Leaf(node));
                }
            }
            Self::Open(open) => {
                open.content.insert_primitive(node);
            }
            Self::Closed(x) => {
                *self = Self::fragment(vec![
                    Self::Closed(x.clone()),
                    Self::Leaf(node),
                ]);
            }
            Self::Leaf(x) => {
                *self = Self::fragment(vec![
                    Self::Leaf(x.clone()),
                    Self::Leaf(node),
                ]);
            },
            Self::Empty => {
                *self = Self::Leaf(node);
            }
            Self::Indented(x) => {
                x.content.insert_primitive(node)
            }
        }
    }
    pub fn insert_open(&mut self, node: OpenEnclosure<T>) where T: Clone {
        match self {
            // Self::Branch(l, r) => {
            //     r.insert_open(node);
            // }
            Self::Fragment(xs) => {
                if let Some(r) = xs.back_mut() {
                    r.insert_open(node)
                } else {
                    xs.push_back(Self::Open(Box::new(node)));
                }
            }
            Self::Open(open) => {
                open.content.insert_open(node);
            }
            Self::Closed(x) => {
                // *self = Self::Branch(
                //     Box::new(Self::Closed(x.clone())),
                //     Box::new(Self::Open(Box::new(node))),
                // )
                *self = Self::fragment(vec![
                    Self::Closed(x.clone()),
                    Self::Open(Box::new(node))
                ]);
            }
            Self::Leaf(x) => {
                // *self = Self::Branch(
                //     Box::new(Self::Leaf(x.clone())),
                //     Box::new(Self::Open(Box::new(node))),
                // )
                *self = Self::fragment(vec![
                    Self::Leaf(x.clone()),
                    Self::Open(Box::new(node))
                ]);
            },
            Self::Empty => {
                *self = Self::Open(Box::new(node));
            }
            Self::Indented(x) => {
                x.content.insert_open(node)
            }
        }
    }
    pub fn try_close_with<'f>(&mut self, node: T, guard: &'f dyn Fn(&T) -> bool) -> Option<T> where T: Clone {
        match self {
            Self::Fragment(xs) => {
                if let Some(r) = xs.back_mut() {
                    r.try_close_with(node, guard)
                } else {
                    Some(node)
                }
            }
            Self::Open(open) if open.content.has_open() => {
                if let Some(failed) = open.content.try_close_with(node, guard) {
                    if guard(&open.open) {
                        let node = failed;
                        let OpenEnclosure { open, content } = open.as_ref().clone();
                        *self = Self::Closed(Box::new(ClosedEnclosure { open, content, close: node }));
                        return None
                    }
                    Some(failed)
                } else {
                    None
                }
            }
            Self::Open(open) if guard(&open.open) => {
                let OpenEnclosure { open, content } = open.as_ref().clone();
                *self = Self::Closed(Box::new(ClosedEnclosure { open, content, close: node }));
                return None
            }
            Self::Open(open) => {
                open.content.try_close_with(node, guard)
            }
            Self::Closed(x) => {
                Some(node)
            }
            Self::Leaf(x) => {
                Some(node)
            },
            Self::Empty => Some(node),
            Self::Indented(x) => {
                x.content.try_close_with(node, guard)
            }
        }
    }
    pub fn defragment(&mut self) {
        match self {
            Self::Fragment(xs) => {
                let mut sink: Vec<TreeBuilder<T>> = Default::default();
                for mut x in xs.drain(..) {
                    x.defragment();
                    x.repopulate(&mut sink)
                }
                *self = Self::fragment(sink);
            }
            Self::Open(x) => {
                x.content.defragment();
            }
            Self::Closed(x) => {
                x.content.defragment();
            }
            Self::Leaf(x) => {},
            Self::Empty => {}
            Self::Indented(x) => {
                x.content.defragment()
            }
        }
    }
    pub fn repopulate(self, parent: &mut Vec<TreeBuilder<T>>) {
        match self {
            Self::Fragment(mut xs) => {
                parent.extend(xs);
            }
            Self::Open(x) => {
                parent.push(Self::Open(x));
            }
            Self::Closed(x) => {
                parent.push(Self::Closed(x));
            }
            Self::Leaf(x) => {
                parent.push(Self::Leaf(x));
            },
            Self::Empty => {}
            Self::Indented(x) => {}
        }
    }
    pub fn for_all_seq<'f>(&mut self, f: &'f dyn Fn(&mut VecDeque<TreeBuilder<T>>) -> ()) {
        match self {
            Self::Empty => {}
            Self::Leaf(x) => {
                
            },
            Self::Fragment(xs) => {
                for x in xs.iter_mut() {
                    x.for_all_seq(&f)
                }
                f(xs);
            }
            Self::Open(x) => {
                x.content.for_all_seq(f);
            }
            Self::Closed(x) => {
                x.content.for_all_seq(f);
            }
            Self::Indented(x) => {
                x.content.for_all_seq(f);
            }
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl<T: ToDisplayTree> ToDisplayTree for TreeBuilder<T> {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        match self {
            Self::Empty => {
                DisplayTree::leaf("Empty")
            }
            Self::Leaf(x) => {
                let format = x.to_display_tree();
                DisplayTree::branch("Leaf", vec![format])
            }
            // Self::Branch(l, r) => {
            //     let l = l.to_display_tree();
            //     let r = r.to_display_tree();
            //     DisplayTree::branch("Branch", vec![l, r])
            // }
            Self::Fragment(xs) => {
                let xs = xs.iter().map(|x| x.to_display_tree()).collect_vec();
                DisplayTree::branch("Fragment", xs)
            }
            Self::Open(x) => x.to_display_tree(),
            Self::Closed(x) => x.to_display_tree(),
            Self::Indented(x) => x.to_display_tree(),
        }
    }
}

impl<T: ToDisplayTree> ToDisplayTree for OpenEnclosure<T> {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let open = self.open.to_display_tree();
        let content = self.content.to_display_tree();
        DisplayTree::branch("OpenEnclosure", vec![open, content])
    }
}
impl<T: ToDisplayTree> ToDisplayTree for ClosedEnclosure<T> {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let open = self.open.to_display_tree();
        let content = self.content.to_display_tree();
        let close = self.close.to_display_tree();
        DisplayTree::branch("ClosedEnclosure", vec![open, content, close])
    }
}
impl<T: ToDisplayTree> ToDisplayTree for IndentedBlock<T> {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        let content = self.content.to_display_tree();
        DisplayTree::branch("IndentedBlock", vec![content])
    }
}

