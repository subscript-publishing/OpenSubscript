use std::collections::VecDeque;
use tree_formatter::ToDisplayTree;
use crate::{SrcUnit, BinaryTokenTree, binary_token_tree};
use super::src_input::string_input_pipeline;

pub fn fold_units_into_binary_token_tree(source: impl IntoIterator<Item=SrcUnit>) -> BinaryTokenTree<SrcUnit> {
    let mut source = source.into_iter().collect::<VecDeque<_>>();
    let mut tree_builder = BinaryTokenTree::<SrcUnit>::Empty;
    while let Some(node) = source.pop_front() {
        if node.is_open_bracket() {
            let new_open = binary_token_tree::OpenEnclosure{open: node, content: BinaryTokenTree::Empty};
            tree_builder.insert_open(new_open);
            continue;
        }
        match node.unwrap_close_bracket() {
            Ok(close) => {
                let guard = &|x: &SrcUnit| -> bool {
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
                tree_builder.try_close_with(SrcUnit::CloseBracket(close), guard);
            }
            Err(node) => {
                tree_builder.insert_primitive(node);
            }
        }
    }
    // tree_builder.defragment();
    // tree_builder.for_all_seq(&indent);
    tree_builder
}

pub fn debug_print(source: &str) {
    let source: VecDeque<SrcUnit> = string_input_pipeline(source).to_vec().into_iter().collect::<VecDeque<_>>();
    let output = fold_units_into_binary_token_tree(source);
    output.to_display_tree().pretty_print();
}
