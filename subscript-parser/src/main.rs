#![allow(unused)]
// use subscript_tokenizer::pipeline;

use subscript_tokenizer::token_tree::token_tree_dev1;

fn main() {
    // println!("Hello, world!");
    let source_code = include_str!("../samples/input/file1.txt");
    // let source_code = include_str!("../samples/misc/file1.txt");
    // let output = subscript_tokenizer::pipeline::src_input::string_input_pipeline(source_code);
    // let output = subscript_tokenizer::pipeline::fold_to_tree::fold_units_into_binary_token_tree(output.into_vec());
    // let output = subscript_tokenizer::pipeline::fold_to_tree::debug_print(source_code);
    // let _ = subscript_tokenizer::pipeline::src_input::debug_print(source_code);
    // let _ = subscript_tokenizer::pipeline::tokenizer::run_tokenizer(source_code);
    // subscript_tokenizer
    // println!("{:#?}", token_tree);
    token_tree_dev1(source_code);
}
