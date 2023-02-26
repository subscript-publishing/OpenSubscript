use std::collections::VecDeque;

use tree_formatter::{ToDisplayTree, DisplayTree};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct Zipper<T> {
    pub leading: VecDeque<T>,
    pub focus: Option<T>,
    pub trailing: VecDeque<T>,
}

impl<T> Zipper<T> {
    pub fn forward(&mut self) {
        let _ = self.focus
            .take()
            .map(|x| {
                self.leading.push_back(x);
            });
        self.focus = self.trailing.pop_front();
    }
    pub fn to_vec(self) -> Vec<T> {
        let mut results: Vec<_> = Default::default();
        results.extend(self.leading);
        self.focus.map(|x| results.push(x));
        results.extend(self.trailing);
        results
    }
    pub fn take_next(&mut self) -> Option<T> {
        if !self.trailing.is_empty() {
            let first = self.trailing.pop_front().unwrap();
            return Some(first)
        }
        None
    }
    pub fn take_next_pair(&mut self) -> Option<(T, T)> {
        if self.trailing.len() >= 2 {
            let first = self.trailing.pop_front().unwrap();
            let second = self.trailing.pop_front().unwrap();
            return Some((first, second))
        }
        None
    }
    pub fn take_where<U>(&mut self, f: impl Fn(&T) -> TakeWhilePred<U>) -> Result<(Vec<T>, Option<U>), ()> {
        let mut results: Vec<T> = Default::default();
        while let Some(next) = self.trailing.pop_front() {
            match f(&next) {
                TakeWhilePred::TerminateOk { include_take_this } => {
                    if include_take_this.is_some() {
                        return Ok((results, include_take_this));
                    } else {
                        self.trailing.push_front(next);
                        return Ok((results, None));
                    }
                }
                TakeWhilePred::TerminateErr => {
                    for result in results.into_iter().rev() {
                        self.trailing.push_front(result)
                    }
                    return Err(())
                }
                TakeWhilePred::Continue => {
                    results.push(next);
                    continue;
                }
            }
        }
        for result in results.into_iter().rev() {
            self.trailing.push_front(result)
        }
        return Err(())
    }
}

pub enum TakeWhilePred<U> {
    Continue,
    TerminateOk {
        include_take_this: Option<U>,
    },
    TerminateErr,
}

impl<T> Zipper<T> {
    pub fn try_coalesce_focus(&mut self) where T: ZipperTryCoalesce {
        if let Some((left, right)) = self.focus.take().zip(self.take_next()) {
            match T::try_coalesce_focus(left, right) {
                Ok(new) => {
                    self.focus = Some(new);
                }
                Err((left, right)) => {
                    self.focus = Some(left);
                    self.trailing.push_front(right);
                }
            }
        }
    }
    pub fn try_coalesce_focus2(&mut self) where T: ZipperTryCoalesce2 {
        if let Some((left, (between, right))) = self.focus.take().zip(self.take_next_pair()) {
            match T::try_coalesce_focus2(left, between, right) {
                Ok(new) => {
                    self.focus = Some(new);
                }
                Err((left, between, right)) => {
                    self.focus = Some(left);
                    self.trailing.push_front(right);
                    self.trailing.push_front(between);
                }
            }
        }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct Zipper3View<'a, T> {
    pub leading: &'a mut VecDeque<T>,
    pub focus: Zipper3Focus<T>,
    pub trailing: &'a mut VecDeque<T>,
}

pub struct Zipper3Focus<T> {
    pub left: T,
    pub center: T,
    pub right: T,
}

pub trait ZipperTryCoalesce where Self: Sized {
    fn try_coalesce_focus(self, next: Self) -> Result<Self, (Self, Self)> {
        Err((self, next))
    }
}

pub trait ZipperTryCoalesce2 where Self: Sized {
    fn try_coalesce_focus2(self, between: Self, next: Self) -> Result<Self, (Self, Self, Self)> {
        Err((self, between, next))
    }
}




//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// DEBUG
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T: ToDisplayTree> ToDisplayTree for Zipper<T> {
    fn to_display_tree(&self) -> DisplayTree {
        let leading = DisplayTree::branch("leading", vec![self.leading.to_display_tree()]);
        let focus = DisplayTree::branch("focus", vec![self.focus.to_display_tree()]);
        let trailing = DisplayTree::branch("trailing", vec![self.trailing.to_display_tree()]);
        tree_formatter::DisplayTree::branch("Zipper", [leading, focus, trailing])
    }
}
