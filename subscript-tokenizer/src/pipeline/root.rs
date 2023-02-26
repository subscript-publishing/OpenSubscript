use std::collections::VecDeque;
use crate::{SrcUnit, BinaryTokenTree};
use super::{src_input::string_input_pipeline, fold_to_tree::fold_units_into_binary_token_tree};

pub fn run_pipeline(source: &str) -> BinaryTokenTree<SrcUnit> {
    let source: VecDeque<SrcUnit> = string_input_pipeline(source).to_vec().into_iter().collect::<VecDeque<_>>();
    fold_units_into_binary_token_tree(source)
}
