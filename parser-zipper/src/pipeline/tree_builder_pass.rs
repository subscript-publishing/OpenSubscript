use std::{fmt::Display, collections::VecDeque};

use tree_formatter::{DisplayTree, ToDisplayTree};

use crate::{stream::bucket_stream::dev_pipeline1, basics::{BlockNode, Bucket, NestedSeq, Seq, IndexedString, TreeBuilder, OpenEnclosure}};

// #[derive(Debug, Clone)]
// pub enum TopLevel {
//     Bucket(Bucket),
//     Tree(Box<NestedSeq<TopLevel>>),
// }

fn indent(xs: &mut VecDeque<TreeBuilder<Bucket>>) {
    for x in xs {
        match x {
            TreeBuilder::Fragment(xs) => {
                
            }
            TreeBuilder::Leaf(x) => {
                
            },
            TreeBuilder::Open(x) => {
                
            }
            TreeBuilder::Closed(x) => {
                
            }
            TreeBuilder::Empty => {}
            TreeBuilder::Indented(x) => {}
        }
    }
}
fn indent2(xs: &mut VecDeque<IndexedString>) {

}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub fn dev1() {
    let mut source: VecDeque<Bucket> = dev_pipeline1().into_vec().into_iter().collect::<VecDeque<_>>();
    let mut tree_builder = TreeBuilder::<Bucket>::Empty;
    while let Some(node) = source.pop_front() {
        if node.is_open_bracket() {
            let new_open = OpenEnclosure{open: node, content: TreeBuilder::Empty};
            tree_builder.insert_open(new_open);
            continue;
        }
        match node.unwrap_close_bracket() {
            Ok(close) => {
                let guard = &|x: &Bucket| -> bool {
                    if !x.is_any_open() {
                        return false
                    }
                    let open = x.clone().unwrap_open_bracket().unwrap();
                    match (open.char, close.char) {
                        ('{', '}') => true,
                        ('[', ']') => true,
                        ('(', ')') => true,
                        ('<', '>') => true,
                        _ => false,
                    }
                };
                tree_builder.try_close_with(Bucket::CloseBracket(close), guard);
            }
            Err(node) => {
                tree_builder.insert_primitive(node);
            }
        }
    }
    tree_builder.defragment();
    tree_builder.for_all_seq(&indent);
    tree_builder.to_display_tree().pretty_print();
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// impl Display for TopLevel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Bucket(x) => {
//                 let format = format!("{x}");
//                 f.write_str(&format)
//             }
//             Self::Tree(xs) => {
//                 // let format = xs.to_display_tree().to_pretty_string();
//                 // f.write_str(&format)
//                 unimplemented!()
//             }
//         }
//     }
// }

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// impl ToDisplayTree for TopLevel {
//     fn to_display_tree(&self) -> DisplayTree {
//         match self {
//             Self::Bucket(x) => {
//                 x.to_display_tree()
//             }
//             Self::Tree(xs) => {
//                 // xs.to_display_tree()
//                 unimplemented!()
//             }
//         }
//     }
// }
