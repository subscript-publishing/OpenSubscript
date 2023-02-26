use crate::{token_tree::{TokenTree, Fragment, SomeInsertionSiteRef, SomeInsertionSite, ActiveInsertionSite, FinalizedSite}, SomeIdentifier1, SomeIdentifier2, SomeSectionHeader, Newline, Space, SrcString, SomePrimitive, binary_token_tree::OpenEnclosure};

pub trait TTMutVisitor {
    fn some_root(&self, node: &mut TokenTree) {}
    fn symbol(&self, node: &mut SomePrimitive) {}
    fn id1(&self, node: &mut SomeIdentifier1) {}
    fn id2(&self, node: &mut SomeIdentifier2) {}
    fn section_header(&self, node: &mut SomeSectionHeader) {}
    fn newline(&self, node: &mut Newline) {}
    fn space(&self, node: &mut Space) {}
    fn plain_text(&self, node: &mut SrcString) {}
    fn insertion_site(&self, node: &mut SomeInsertionSiteRef) {}
    fn fragment(&self, node: &mut Fragment) {}
    // fn active_insertion_site(&self, &mut ActiveInsertionSite)
}

impl TokenTree {
    pub fn apply_mut_fragment_visitor<V>(&mut self, visitor: &V) where V: TTMutVisitor {
        match self {
            Self::SomeSymbol(x) => {
                visitor.symbol(x);
            }
            Self::Id1(x) => {
                visitor.id1(x);
            }
            Self::Id2(x) => {
                visitor.id2(x);
            }
            Self::SectionHeader(x) => {
                visitor.section_header(x);
            }
            Self::Newline(x) => {
                visitor.newline(x);
            }
            Self::Space(x) => {
                visitor.space(x);
            }
            Self::PlainText(x) => {
                visitor.plain_text(x);
            }
            Self::InsertionSite(x) => {
                x.map_mut(|x| {
                    match x {
                        SomeInsertionSite::Active(ActiveInsertionSite::Unclosed(x)) => {
                            x.content.apply_mut_fragment_visitor(visitor);
                        }
                        SomeInsertionSite::Finalized(FinalizedSite::Closed(x)) => {
                            x.content.apply_mut_fragment_visitor(visitor);
                        }
                    }
                });
                visitor.insertion_site(x);
            }
            Self::Fragment(xs) => {
                for x in xs.fragment.iter_mut() {
                    x.apply_mut_fragment_visitor(visitor);
                }
                visitor.fragment(xs);
            }
            Self::Empty => ()
        }
        visitor.some_root(self);
    }
}

