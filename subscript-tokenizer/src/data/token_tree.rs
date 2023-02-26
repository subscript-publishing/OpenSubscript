use std::collections::VecDeque;

use either::Either::{self, Left, Right};
use itertools::Itertools;
use tree_formatter::{ToDisplayTree, DisplayTree};
use crate::{SrcString, Newline, SrcChar, MutState, Space, SrcGroup, SomePrimitive, SomeBracket, SomeOpenBracket, SomeCloseBracket, SomeIdentifier1, SomeIdentifier2, SomeSectionHeader};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SPECIAL
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct CloseTokenRequest {
    pub close: SomeCloseBracket,
}

impl OpenEnclosure {
    pub fn is_valid_close_token(&self, request: &CloseTokenRequest) -> bool {
        match (self.open, request.close) {
            (SomeOpenBracket::CurlyBracket(_), SomeCloseBracket::CurlyBracket(_)) => true,
            (SomeOpenBracket::SquareBracket(_), SomeCloseBracket::SquareBracket(_)) => true,
            (SomeOpenBracket::RoundBracket(_), SomeCloseBracket::RoundBracket(_)) => true,
            (SomeOpenBracket::AngleBracket(_), SomeCloseBracket::AngleBracket(_)) => true,
            _ => false,
        }
    }
    pub fn unify(self, request: &CloseTokenRequest) -> ClosedEnclosure {
        assert!(self.is_valid_close_token(request));
        ClosedEnclosure { open: self.open, content: self.content, close: request.close.clone() }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum ActiveInsertionSite {
    Unclosed(OpenEnclosure),
}

// pub type ActiveInsertionSiteRef = MutState<ActiveInsertionSite>;

impl ActiveInsertionSite {
    pub fn try_close_token_request(&mut self, request: &CloseTokenRequest) -> Option<ClosedEnclosure> {
        match self {
            Self::Unclosed(open) if open.is_valid_close_token(request) => {
                Some(open.clone().unify(request))
            }
            Self::Unclosed(open) => {
                None
            }
        }
    }
}

impl ToDisplayTree for ActiveInsertionSite {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Unclosed(open) => {
                let label = "ActiveInsertionSite";
                let children = [open.to_display_tree()];
                DisplayTree::branch(label, children)
            }
        }
    }
}
impl<T> ToDisplayTree for MutState<T> where T: ToDisplayTree {
    fn to_display_tree(&self) -> DisplayTree {
        self.map_ref(|x| x.to_display_tree())
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum FinalizedSite {
    Closed(ClosedEnclosure),
}

impl ToDisplayTree for FinalizedSite {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Closed(x) => x.to_display_tree(),
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum SomeInsertionSite {
    Active(ActiveInsertionSite),
    Finalized(FinalizedSite)
}

pub type SomeInsertionSiteRef = MutState<SomeInsertionSite>;

impl SomeInsertionSiteRef {
    pub fn is_active(&self) -> bool {
        self.map_ref(|site| match site {
            SomeInsertionSite::Active(_) => true,
            SomeInsertionSite::Finalized(_) => false,
        })
    }
}
impl ToDisplayTree for SomeInsertionSite {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Active(x) => {
                let label = "SomeInsertionSite::Active";
                let children = [x.to_display_tree()];
                DisplayTree::branch(label, children)
            }
            Self::Finalized(x) => {
                let label = "SomeInsertionSite::Finalized";
                let children = [x.to_display_tree()];
                DisplayTree::branch(label, children)
            }
        }
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct OpenEnclosure {
    pub open: SomeOpenBracket,
    pub content: TokenTree,
}

#[derive(Debug, Clone)]
pub struct OpenEnclosureRef(pub MutState<OpenEnclosure>);

impl OpenEnclosureRef {
    pub fn pack(open: OpenEnclosure) -> Self {
        OpenEnclosureRef(MutState::new(open))
    }
    pub fn new(open: SomeOpenBracket) -> Self {
        Self::pack(OpenEnclosure { open, content: Default::default() })
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct ClosedEnclosure {
    pub open: SomeOpenBracket,
    pub content: TokenTree,
    pub close: SomeCloseBracket,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Branch {
    pub left: TokenTree,
    pub right: TokenTree,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Default)]
pub struct Fragment {
    pub fragment: VecDeque<TokenTree>,
}

impl Fragment {
    pub fn from_iter(xs: impl IntoIterator<Item=TokenTree>) -> Self {
        let xs = xs
            .into_iter()
            .flat_map(|x| match x {
                TokenTree::Fragment(xs) => Vec::from_iter(xs.fragment),
                x => vec![x],
            })
            .collect();
        Self { fragment: xs }
    }
    pub fn rightmost_mut(&mut self) -> Option<&mut TokenTree> {
        self.fragment.back_mut()
    }
    pub fn rightmost_ref(&self) -> Option<&TokenTree> {
        self.fragment.back()
    }
    pub fn push(&mut self, new: TokenTree) {
        match new {
            TokenTree::Fragment(y) => {
                let ys = y.fragment
                    .into_iter()
                    .flat_map(|a| a.flatten());
                self.fragment.extend(ys);
            }
            y => {
                self.fragment.extend(y.flatten());
            }
        }
    }
    pub fn for_rightmost_or(
        &mut self,
        f: impl FnOnce(&mut TokenTree),
        or: impl FnOnce() -> TokenTree,
    ) {
        if let Some(right) = self.rightmost_mut() {
            f(right)
        } else {
            self.push(or())
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEBUG
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl ToDisplayTree for OpenEnclosure {
    fn to_display_tree(&self) -> DisplayTree {
        let label = "OpenEnclosure";
        let children = [
            DisplayTree::singleton("open", self.open.to_display_tree()),
            DisplayTree::singleton("content", self.content.to_display_tree()),
        ];
        DisplayTree::branch(label, children)
    }
}
impl ToDisplayTree for OpenEnclosureRef {
    fn to_display_tree(&self) -> DisplayTree {
        self.0.map_ref(|x| x.to_display_tree())
    }
}
impl ToDisplayTree for ClosedEnclosure {
    fn to_display_tree(&self) -> DisplayTree {
        let label = "ClosedEnclosure";
        let children = [
            DisplayTree::singleton("open", self.open.to_display_tree()),
            DisplayTree::singleton("content", self.content.to_display_tree()),
            DisplayTree::singleton("close", self.close.to_display_tree()),
        ];
        DisplayTree::branch(label, children)
    }
}
impl ToDisplayTree for Branch {
    fn to_display_tree(&self) -> DisplayTree {
        let label = "Branch";
        let children = [
            DisplayTree::singleton("left", self.left.to_display_tree()),
            DisplayTree::singleton("right", self.right.to_display_tree()),
        ];
        DisplayTree::branch(label, children)
    }
}
impl ToDisplayTree for Fragment {
    fn to_display_tree(&self) -> DisplayTree {
        let label = "Fragment";
        let children = self.fragment.iter().map(|x| x.to_display_tree()).collect_vec();
        DisplayTree::branch(label, children)
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION - TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum TokenTree {
    Empty,
    SomeSymbol(SomePrimitive),
    Id1(SomeIdentifier1),
    Id2(SomeIdentifier2),
    SectionHeader(SomeSectionHeader),
    Newline(Newline),
    Space(Space),
    PlainText(SrcString),
    InsertionSite(SomeInsertionSiteRef),
    Fragment(Fragment),
}

impl TokenTree {
    pub fn fragment(xs: impl IntoIterator<Item=Self>) -> Self {
        Self::Fragment(Fragment::from_iter(xs))
    }
    // pub fn branch(left: TokenTree, right: TokenTree) -> Self { Self::Branch(Box::new(Branch{left, right})) }
    pub fn open(begin: OpenEnclosure) -> Self {
        // Self::Open(OpenEnclosureRef::pack(begin))
        unimplemented!()
    }
    // pub fn closed(begin: ClosedEnclosure) -> Self { Self::ClosedEnclosure(Box::new(begin)) }
    pub fn plain_text(value: SrcString) -> Self { Self::PlainText(value) }
    // pub fn from_any_whitespace(value: SrcString) -> Vec<Self> {
    //     value.string
    //         .into_iter()
    //         .map(|x| -> Either<SrcString, SrcString> {
    //             if x.char == ' ' {
    //                 return Left(SrcString::singleton(x))
    //             }
    //             if x.char == '\n' {
    //                 return Right(SrcString::singleton(x))
    //             }
    //             panic!("NOT POSSIBLE - INVALID INPUT")
    //         })
    //         .coalesce(|x, y| -> Result<Either<SrcString, SrcString>, (Either<SrcString, SrcString>, Either<SrcString, SrcString>)> {
    //             match (x, y) {
    //                 (Left(mut x), Left(y)) => {
    //                     x.append(y);
    //                     Ok(Left(x))
    //                 }
    //                 (Right(mut x), Right(y)) => {
    //                     x.append(y);
    //                     Ok(Right(x))
    //                 }
    //                 (x, y) => Err((x, y))
    //             }
    //         })
    //         .map(|x| x.map_left(|x| BeginColumn::new(x)).map_right(|x| Newline::new(x)))
    //         .map(|x| x.map_left(|x| Self::BeginColumn(x)).map_right(|x| Self::Newline(x)))
    //         .map(|x| x.into_inner())
    //         .collect_vec()
    // }
    pub fn get_all_active_sites(&self, sink: &mut VecDeque<SomeInsertionSiteRef>) {
        match self {
            Self::InsertionSite(x) => {
                if x.is_active() {
                    sink.push_back(x.clone());
                }
                x.map_ref(|site| match site {
                    SomeInsertionSite::Active(ActiveInsertionSite::Unclosed(open)) => {
                        open.content.get_all_active_sites(sink);
                    }
                    SomeInsertionSite::Finalized(FinalizedSite::Closed(closed)) => {
                        closed.content.get_all_active_sites(sink);
                    }
                });
            }
            Self::Fragment(x) => {
                if let Some(last) = x.rightmost_ref() {
                    last.get_all_active_sites(sink);
                }
            }
            Self::Id1(_) => (),
            Self::Id2(_) => (),
            Self::SectionHeader(_) => (),
            Self::Empty => (),
            Self::Newline(x) => (),
            // Self::BeginColumn(x) => (),
            Self::Space(x) => (),
            Self::PlainText(x) => (),
            // Self::ClosedEnclosure(x) => (),
            Self::SomeSymbol(x) => (),
        }
    }
    pub fn insert_at_active_site(&mut self, node: TokenTree) {
        let mut actives: VecDeque<SomeInsertionSiteRef> = Default::default();
        self.get_all_active_sites(&mut actives);
        if let Some(lastmost) = actives.back() {
            lastmost.map_mut(|target| {
                match target {
                    SomeInsertionSite::Active(ActiveInsertionSite::Unclosed(open)) => {
                        open.content.unchecked_push(node);
                    }
                    SomeInsertionSite::Finalized(_) => panic!("NOT POSSIBLE")
                }
            });
            return ()
        }
        self.unchecked_push(node);
    }
    pub fn process_close_request(&mut self, request: &CloseTokenRequest) {
        let mut sites: VecDeque<SomeInsertionSiteRef> = Default::default();
        self.get_all_active_sites(&mut sites);
        enum OpResult {
            NoOp,
            Done
        }
        for site in sites.into_iter().rev() {
            let result = site.map_mut(|site| {
                match site {
                    SomeInsertionSite::Active(ActiveInsertionSite::Unclosed(open)) => {
                        if open.is_valid_close_token(request) {
                            let new = open.clone().unify(request);
                            let new = FinalizedSite::Closed(new);
                            *site = SomeInsertionSite::Finalized(new);
                            return OpResult::Done
                        }
                        OpResult::NoOp
                    }
                    SomeInsertionSite::Finalized(_) => panic!("NOT POSSIBLE")
                }
            });
            match result {
                OpResult::Done => {
                    break;
                }
                OpResult::NoOp => {
                    continue;
                }
            }
        }
    }
    pub fn unchecked_push(&mut self, node: TokenTree) {
        match self {
            Self::Empty => {
                *self = node;
            }
            Self::Fragment(x) => {
                x.push(node);
            }
            left => {
                let new_fragment = [left.clone().flatten(), node.flatten()]
                    .concat()
                    .into_iter()
                    .flat_map(|x| x.flatten())
                    .collect::<Vec<_>>();
                if new_fragment.is_empty() {
                    return ()
                }
                if new_fragment.len() == 1 {
                    let new_item = new_fragment.first().map(Clone::clone).unwrap();
                    *left = new_item;
                }
                *left = Self::Fragment(Fragment { fragment: FromIterator::from_iter(new_fragment) });
            }
        }
    }
    pub fn push_group(&mut self, group: SrcGroup) {
        match group {
            SrcGroup::SomePrimitive(x) => {
                match x {
                    SomePrimitive::Bracket(SomeBracket::Open(open)) => {
                        let new = OpenEnclosure{open, content: Default::default()};
                        let new = ActiveInsertionSite::Unclosed(new);
                        let new = SomeInsertionSite::Active(new);
                        let new = SomeInsertionSiteRef::new(new);
                        let new = Self::InsertionSite(new);
                        self.insert_at_active_site(new);
                    }
                    SomePrimitive::Bracket(SomeBracket::Close(close)) => {
                        let request = CloseTokenRequest{close};
                        self.process_close_request(&request);
                    }
                    SomePrimitive::QuotationMark(x) => {
                        unimplemented!()
                    }
                    SomePrimitive::Backslash(x) => {
                        unimplemented!()
                    }
                    SomePrimitive::Pipe(x) => {
                        unimplemented!()
                    }
                    SomePrimitive::AtSign(x) => {
                        unimplemented!()
                    }
                    SomePrimitive::Backslash2(x) => {
                        unimplemented!()
                    }
                    SomePrimitive::Pipe2(x) => {
                        unimplemented!()
                    }
                }
            }
            SrcGroup::Newline(x) => {
                let new = Self::Newline(x);
                self.insert_at_active_site(new);
            }
            SrcGroup::Space(x) => {
                let new = Self::Space(x);
                self.insert_at_active_site(new);
            }
            SrcGroup::Id1(x) => {
                let new = Self::Id1(x);
                self.insert_at_active_site(new);
            }
            SrcGroup::Id2(x) => {
                let new = Self::Id2(x);
                self.insert_at_active_site(new);
            }
            SrcGroup::SectionHeader(x) => {
                let new = Self::SectionHeader(x);
                self.insert_at_active_site(new);
            }
            SrcGroup::PlainText(x) => {
                let new = Self::PlainText(x);
                self.insert_at_active_site(new);
            }
        }
    }
    pub fn flatten(self) -> Vec<Self> {
        match self {
            // Self::ClosedEnclosure(x) => vec![Self::ClosedEnclosure(x)],
            Self::Empty => Default::default(),
            Self::Newline(x) => vec![Self::Newline(x)],
            // Self::BeginColumn(x) => vec![Self::BeginColumn(x)],
            Self::Space(x) => vec![Self::Space(x)],
            Self::PlainText(x) => vec![Self::PlainText(x)],
            Self::InsertionSite(x) => vec![Self::InsertionSite(x)],
            Self::Id1(x) => vec![Self::Id1(x)],
            Self::Id2(x) => vec![Self::Id2(x)],
            Self::SectionHeader(x) => vec![Self::SectionHeader(x)],
            Self::SomeSymbol(x) => vec![Self::SomeSymbol(x)],
            Self::Fragment(x) => Vec::from_iter(x.fragment),
        }
    }
}

impl Default for TokenTree {
    fn default() -> Self { Self::Empty }
}
impl ToDisplayTree for TokenTree {
    fn to_display_tree(&self) -> DisplayTree {
        match self {
            Self::Empty => DisplayTree::leaf("Empty"),
            Self::Newline(x) => x.to_display_tree(),
            Self::Space(x) => x.to_display_tree(),
            // Self::BeginColumn(x) => x.to_display_tree(),
            Self::PlainText(x) => x.to_display_tree(),
            Self::InsertionSite(x) => x.to_display_tree(),
            // Self::ClosedEnclosure(x) => x.to_display_tree(),
            Self::Fragment(x) => x.to_display_tree(),
            Self::SomeSymbol(x) => x.to_display_tree(),
            Self::Id1(x) => x.to_display_tree(),
            Self::Id2(x) => x.to_display_tree(),
            Self::SectionHeader(x) => x.to_display_tree(),
        }
    }
}
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEV
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

pub fn token_tree_dev1(source_code: &str) {
    // let source_code = "    * A\n    * B\n    * C\n    * D";
    // let src_buffer = crate::pipeline::src_input::string_input_pipeline(source_code).to_vec();
    let mut token_tree = TokenTree::default();
    let tokens = crate::pipeline::tokenizer::run_tokenizer(source_code);
    for token in tokens {
        // token_tree.push_unit(unit);
        token_tree.push_group(token);
    }
    // let src_buffer = SrcChar::to_indexed_chars(source_code);
    // let src_buffer = SrcString::from_iter(src_buffer);
    // token_tree.to_display_tree().pretty_print();
    // for unit in src_buffer {
    //     match unit {
    //         SrcUnit::AnyWhitespace()
    //     }
    // }
    println!("DONE");
    token_tree.to_display_tree().pretty_print()
}
