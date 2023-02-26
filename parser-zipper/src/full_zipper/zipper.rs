use std::collections::VecDeque;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct FullZipper<T> {
    pub leading: FullLeadingZipper<T>,
    pub focus: Option<T>,
    pub trailing: FullTrailingZipper<T>,
}

impl<T> Default for FullZipper<T> {
    fn default() -> Self {
        Self { leading: Default::default(), focus: None, trailing: Default::default() }
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum FullLeadingZipper<T> {
    Saturated(VecDeque<T>, T),
    Singleton(T),
    Empty,
}

impl<T> Default for FullLeadingZipper<T> {
    fn default() -> Self { FullLeadingZipper::Empty }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum FullTrailingZipper<T> {
    Empty,
    Singleton(T),
    Saturated(T, VecDeque<T>),
}

impl<T> Default for FullTrailingZipper<T> {
    fn default() -> Self { FullTrailingZipper::Empty }
}

