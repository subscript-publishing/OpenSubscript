use std::collections::VecDeque;

use itertools::Itertools;

use crate::basics::{char_utils, IndexedChar, Bucket, IndexedString};


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct StreamZipper<T> {
    pub leading: VecDeque<T>,
    pub focus: Option<T>,
    pub trailing: VecDeque<T>
}

impl<T> StreamZipper<T> {
    pub fn forward(&mut self) {
        let _ = self.focus
            .take()
            .map(|x| {
                self.leading.push_back(x);
            });
        self.focus = self.trailing.pop_front();
    }
    pub fn into_vec(self) -> Vec<T> {
        let mut results: Vec<_> = Default::default();
        results.extend(self.leading);
        self.focus.map(|x| results.push(x));
        results.extend(self.trailing);
        results
    }
}



