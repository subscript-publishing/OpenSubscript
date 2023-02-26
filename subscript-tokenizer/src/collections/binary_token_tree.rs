use tree_formatter::{ToDisplayTree, DisplayTree};


pub type BoxedTree<T> = Box<BinaryTokenTree<T>>;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum BinaryTokenTree<T> {
    Empty,
    Leaf(T),
    Branch(BoxedTree<T>, BoxedTree<T>),
    Open(Box<OpenEnclosure<T>>),
    Closed(Box<ClosedEnclosure<T>>),
    Indented(Box<IndentedBlock<T>>)
}

#[derive(Debug, Clone)]
pub struct OpenEnclosure<T> {
    pub open: T,
    pub content: BinaryTokenTree<T>,
}

#[derive(Debug, Clone)]
pub struct ClosedEnclosure<T> {
    pub open: T,
    pub content: BinaryTokenTree<T>,
    pub close: T,
}

#[derive(Debug, Clone)]
pub struct IndentedBlock<T> {
    pub content: BinaryTokenTree<T>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T> BinaryTokenTree<T> {
    // pub fn fragment(xs: impl IntoIterator<Item=TokenFolder<T>>) -> Self {
    //     let xs = xs.into_iter().reduce(|x, y| -> TokenFolder<T> {
    //         TokenFolder::Branch(Box::new(x), Box::new(y))
    //     });
    //     xs.unwrap()
    // }
    pub fn branch(left: BinaryTokenTree<T>, right: BinaryTokenTree<T>) -> Self {
        Self::Branch(Box::new(left), Box::new(right))
    }
    pub fn has_open(&self) -> bool {
        match self {
            Self::Branch(_, r) => r.has_open(),
            // Self::Fragment(xs) => {
            //     // xs.last().map(|x| x.has_open()).unwrap_or(false)
            //     xs.back().map(|x| x.has_open()).unwrap_or(false)
            // }
            Self::Open(_) => true,
            Self::Closed(_) => false,
            Self::Leaf(_) => false,
            Self::Empty => false,
            Self::Indented(x) => x.content.has_open()
        }
    }
    pub fn insert_primitive(&mut self, node: T) where T: Clone {
        match self {
            // Self::Fragment(xs) => {
            //     if let Some(r) = xs.back_mut() {
            //         r.insert_primitive(node)
            //     } else {
            //         xs.push_back(Self::Leaf(node));
            //     }
            // }
            Self::Branch(_, r) => r.insert_primitive(node),
            Self::Open(open) => {
                open.content.insert_primitive(node);
            }
            Self::Closed(x) => {
                // *self = Self::fragment(vec![
                //     Self::Closed(x.clone()),
                //     Self::Leaf(node),
                // ]);
                *self = Self::branch(Self::Closed(x.clone()), Self::Leaf(node));
            }
            Self::Leaf(x) => {
                // *self = Self::fragment(vec![
                //     Self::Leaf(x.clone()),
                //     Self::Leaf(node),
                // ]);
                *self = Self::branch(Self::Leaf(x.clone()), Self::Leaf(node));
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
            Self::Branch(_, r) => {
                r.insert_open(node);
            }
            // Self::Fragment(xs) => {
            //     if let Some(r) = xs.back_mut() {
            //         r.insert_open(node)
            //     } else {
            //         xs.push_back(Self::Open(Box::new(node)));
            //     }
            // }
            Self::Open(open) => {
                open.content.insert_open(node);
            }
            Self::Closed(x) => {
                *self = Self::Branch(
                    Box::new(Self::Closed(x.clone())),
                    Box::new(Self::Open(Box::new(node))),
                )
                // *self = Self::fragment(vec![
                //     Self::Closed(x.clone()),
                //     Self::Open(Box::new(node))
                // ]);
            }
            Self::Leaf(x) => {
                *self = Self::Branch(
                    Box::new(Self::Leaf(x.clone())),
                    Box::new(Self::Open(Box::new(node))),
                )
                // *self = Self::fragment(vec![
                //     Self::Leaf(x.clone()),
                //     Self::Open(Box::new(node))
                // ]);
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
            // Self::Fragment(xs) => {
            //     if let Some(r) = xs.back_mut() {
            //         r.try_close_with(node, guard)
            //     } else {
            //         Some(node)
            //     }
            // }
            Self::Branch(_, r) => {
                r.try_close_with(node, guard)
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
            Self::Closed(_) => {
                Some(node)
            }
            Self::Leaf(_) => {
                Some(node)
            },
            Self::Empty => Some(node),
            Self::Indented(x) => {
                x.content.try_close_with(node, guard)
            }
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

impl<T: ToDisplayTree> ToDisplayTree for BinaryTokenTree<T> {
    fn to_display_tree(&self) -> tree_formatter::DisplayTree {
        match self {
            Self::Empty => {
                DisplayTree::leaf("Empty")
            }
            Self::Leaf(x) => {
                let format = x.to_display_tree();
                DisplayTree::branch("Leaf", vec![format])
            }
            Self::Branch(l, r) => {
                let l = l.to_display_tree();
                let r = r.to_display_tree();
                DisplayTree::branch("Branch", vec![l, r])
            }
            // Self::Fragment(xs) => {
            //     let xs = xs.iter().map(|x| x.to_display_tree()).collect_vec();
            //     DisplayTree::branch("Fragment", xs)
            // }
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

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TESTS
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// #[cfg(test)]
// mod tests {
//     use crate::Bucket;

//     use super::*;

//     #[test]
//     fn it_works() {
//         let item1 = 1;
//         let item2 = 1;
//         // let token_tree = TokenFolder::fragment(1, 2);
//     }
// }
