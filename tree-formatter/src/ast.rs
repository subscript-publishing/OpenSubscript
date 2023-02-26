use std::collections::VecDeque;
use ptree::{TreeBuilder, PrintConfig, Style, Color, print_config::{UTF_CHARS_BOLD, UTF_CHARS, StaticIndentChars}, print_tree_with, write_tree_with, item::StringItem};

const SYSTEM_CHARS: StaticIndentChars = StaticIndentChars {
    down_and_right: "├",
    down: "│",
    turn_right: "╰",
    right: "─",
    empty: " ",
};

const FRAGMENT_SYMBOL: &'static str = "⋯";

#[derive(Debug, Clone)]
pub enum DisplayTree {
    Leaf {
        value: String,
    },
    Branch {
        value: String,
        children: Vec<DisplayTree>,
    },
    Fragment {
        nodes: Vec<DisplayTree>,
    },
    Empty,
}

impl Default for DisplayTree {
    fn default() -> Self { Self::Empty }
}

impl DisplayTree {
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn leaf(label: impl Into<String>) -> Self {
        Self::Leaf { value: label.into() }
    }
    pub fn branch(label: impl Into<String>, children: impl IntoIterator<Item = DisplayTree>) -> Self {
        let children = children
            .into_iter()
            .collect::<Vec<_>>();
        Self::Branch { value: label.into().into(), children }
    }
    pub fn singleton(label: impl Into<String>, child: impl ToDisplayTree) -> Self {
        Self::branch(label, [child.to_display_tree()])
    }
    pub fn fragment(nodes: impl IntoIterator<Item=DisplayTree>) -> Self {
        Self::Fragment { nodes: nodes.into_iter().collect() }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf { .. } => true,
            _ => false,
        }
    }
    pub fn is_branch(&self) -> bool {
        match self {
            Self::Branch { .. } => true,
            _ => false,
        }
    }
    pub(crate) fn root_backend_p_tree(&self) -> ptree::TreeBuilder {
        match self {
            Self::Leaf { value } => {
                let mut builder = TreeBuilder::new(value.clone());
                builder
            }
            Self::Branch { value, children } => {
                let mut builder = TreeBuilder::new(value.clone());
                for child in children.iter() {
                    child.build_backend_p_tree(&mut builder);
                }
                builder
            }
            Self::Fragment { nodes } => {
                let mut builder = TreeBuilder::new(FRAGMENT_SYMBOL.into());
                for node in nodes.iter() {
                    node.build_backend_p_tree(&mut builder);
                }
                builder
            }
            Self::Empty => {
                let mut builder = TreeBuilder::new(format!("Empty"));
                builder
            }
        }
    }
    pub(crate) fn build_backend_p_tree(&self, builder: &mut ptree::TreeBuilder) {
        match self {
            Self::Leaf { value } => {
                let _ = builder.add_empty_child(value.clone());
            }
            Self::Branch { value, children } => {
                let _ = builder.begin_child(value.clone());
                for child in children.iter() {
                    child.build_backend_p_tree(builder);
                }
                builder.end_child();
            }
            Self::Fragment { nodes } => {
                let _ = builder.begin_child(String::from(FRAGMENT_SYMBOL));
                for node in nodes.iter() {
                    node.build_backend_p_tree(builder);
                }
                builder.end_child();
            }
            Self::Empty => unimplemented!(),
        }
    }
    pub(crate) fn fragment_wth_leading(
        head: DisplayTree,
        nodes: impl IntoIterator<Item=DisplayTree>,
    ) -> Self {
        let mut results: Vec<Vec<DisplayTree>> = vec![
            vec![head],
            nodes.into_iter().collect::<Vec<_>>(),
        ];
        Self::Fragment { nodes: results.concat() }
    }
    pub(crate) fn fragment_with_trailing(
        nodes: impl IntoIterator<Item=DisplayTree>,
        leading: DisplayTree,
    ) -> Self {
        let mut results: Vec<Vec<DisplayTree>> = vec![
            nodes.into_iter().collect::<Vec<_>>(),
            vec![leading],
        ];
        Self::Fragment { nodes: results.concat() }
    }
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            ( Self::Fragment { nodes: n1 }, Self::Fragment { nodes: n2 }) => {
                Self::Fragment { nodes: [n1, n2].concat() }
            }
            (Self::Branch { value: v1, children: c1 }, Self::Fragment { nodes: n2 }) => {
                Self::fragment_wth_leading(
                    Self::Branch { value: v1, children: c1 },
                    n2,
                )
            }
            (Self::Leaf { value: v1 }, Self::Fragment { nodes: n2 }) => {
                Self::fragment_wth_leading(
                    Self::Leaf { value: v1 },
                    n2,
                )
            }
            (Self::Empty, Self::Fragment { nodes: n2 }) => {
                Self::Fragment { nodes: n2 }
            }

            (Self::Fragment { nodes: n1 }, Self::Branch { value: v2, children: c2 }) => {
                Self::fragment_with_trailing(n1, Self::Branch { value: v2, children: c2 })
            }
            (Self::Branch { value: v1, children: c1 }, Self::Branch { value: v2, children: c2 }) => {
                Self::fragment([
                    Self::Branch { value: v1, children: c1 },
                    Self::Branch { value: v2, children: c2 },
                ])
            }
            (Self::Leaf { value: v1 }, Self::Branch { value: v2, children: c2 }) => {
                Self::fragment([
                    Self::Leaf { value: v1 },
                    Self::Branch { value: v2, children: c2 },
                ])
            }
            (Self::Empty, Self::Branch { value: v2, children: c2 }) => {
                Self::Branch { value: v2, children: c2 }
            }

            (Self::Fragment { nodes: n1 }, Self::Leaf { value: v2 }) => {
                Self::fragment_with_trailing(n1, Self::Leaf { value: v2 })
            }
            (Self::Branch { value: v1, children: c1 }, Self::Leaf { value: v2 }) => {
                Self::fragment([
                    Self::Branch { value: v1, children: c1 },
                    Self::Leaf { value: v2 },
                ])
            }
            (Self::Leaf { value: v1 }, Self::Leaf { value: v2 }) => {
                Self::fragment([
                    Self::Leaf { value: v1 },
                    Self::Leaf { value: v2 },
                ])
            }
            ( Self::Empty, Self::Leaf { value: v2 }) => {
                Self::Leaf { value: v2 }
            }

            (Self::Fragment { nodes: n1 }, Self::Empty) => {
                Self::Fragment { nodes: n1 }
            }
            (Self::Branch { value: v1, children: c1 }, Self::Empty) => {
                Self::Branch { value: v1, children: c1 }
            }
            (Self::Leaf { value: v1 }, Self::Empty) => {
                Self::Leaf { value: v1 }
            }
            (Self::Empty , Self::Empty) => {
                Self::Empty
            }
        }
    }
}

pub trait ToDisplayTree {
    fn to_display_tree(&self) -> DisplayTree;
}

impl ToDisplayTree for DisplayTree {
    fn to_display_tree(&self) -> DisplayTree { self.clone() }
}

impl DisplayTree {
    pub fn pretty_print(&self) {
        let (tree_result, config) = self.to_raw();
        let _ = print_tree_with(&tree_result, &config).unwrap();
    }
    pub fn to_pretty_string(&self) -> String {
        let (tree_result, config) = self.to_raw();
        let mut source: Vec<u8> = Vec::default();
        let _ = write_tree_with(&tree_result, &mut source, &config).unwrap();
        String::from_utf8(source).unwrap()
    }
    pub fn format<W: std::io::Write>(&self, mut f: W) -> std::io::Result<()> {
        let (tree_result, config) = self.to_raw();
        write_tree_with(&tree_result, f, &config)
    }
    pub fn to_raw(&self) -> (StringItem, PrintConfig) {
        let config = {
            let mut config = PrintConfig::from_env();
            config.branch = Style {
                foreground: Some(Color::Red),
                background: None,
                bold: false,
                ..Style::default()
            };
            config.leaf = Style {
                foreground: Some(Color::Cyan),
                bold: false,
                ..Style::default()
            };
            config.characters = SYSTEM_CHARS.into();
            config.indent = 3;
            config.padding = 1;
            config
        };
        let item = self
            .to_display_tree()
            .root_backend_p_tree()
            .build();
        (item, config)
    }
}

