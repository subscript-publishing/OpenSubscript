use itertools::Itertools;
use tree_formatter::ToDisplayTree;

use crate::{SrcGroup, token_tree::TokenTree};

pub fn fresh_run_tree_builder(source_code: &str) {
    let tokens = crate::pipeline::tokenizer::run_tokenizer(source_code);
    run_tree_builder(tokens)
}


pub fn run_tree_builder(tokens: impl IntoIterator<Item = SrcGroup>) {
    let tokens = tokens.into_iter().collect_vec();
    let mut token_tree = TokenTree::default();
    for token in tokens {
        // token_tree.push_unit(unit);
        token_tree.push_group(token);
    }
    println!("DONE");
    token_tree.to_display_tree().pretty_print();
}

