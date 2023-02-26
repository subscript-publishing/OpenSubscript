use std::collections::VecDeque;
use itertools::Itertools;
use tree_formatter::ToDisplayTree;
use crate::basics::{Bucket, char_utils, BlockNode};
use super::{StreamZipper, bucket_stream::dev_pipeline1};

impl StreamZipper<BlockNode> {
    pub fn process(&mut self, level: VecDeque<BlockNode>) {
        
    }
}


pub fn dev1() {
    let mut content = dev_pipeline1().into_vec().into_iter().map(|x| BlockNode::Bucket(x));
    let mut zipper = StreamZipper {
        leading: Default::default(),
        focus: None,
        trailing: VecDeque::from_iter(content),
    };
    for x in zipper.into_vec() {
        x.to_display_tree().pretty_print();
    }
}