use ptree::{TreeBuilder, PrintConfig, Style, Color, print_config::{UTF_CHARS_BOLD, UTF_CHARS, StaticIndentChars}, print_tree_with};
const SYSTEM_CHARS: StaticIndentChars = StaticIndentChars {
    down_and_right: "├",
    down: "│",
    turn_right: "╰",
    right: "─",
    empty: " ",
};

use crate::ast::{DisplayTree, ToDisplayTree};

impl DisplayTree {
    pub(crate) fn to_p_tree(self) -> Option<TreeBuilder> {
        match self {
            Self::Fragment { nodes } => {
                unimplemented!()
            }
            Self::Branch { value, children } => {
                let mut builder = TreeBuilder::new(value.clone());
                for child in children {
                    child.build_p_tree(&mut builder);
                }
                builder.end_child();
                Some(builder)
            }
            Self::Leaf { value } => {
                Some(TreeBuilder::new(value.clone()))
            }
            Self::Empty => {
                None
            }
        }
    }
    fn build_p_tree(self, builder: &mut TreeBuilder) {
        match self {
            Self::Fragment { nodes } => {
                unimplemented!()
            }
            Self::Branch { value, children } => {
                builder.begin_child(value.clone());
                for child in children {
                    child.build_p_tree(builder);
                }
                builder.end_child();
            }
            Self::Leaf { value } => {
                builder.add_empty_child(value.clone());
            }
            Self::Empty => {}
        }
    }
}

