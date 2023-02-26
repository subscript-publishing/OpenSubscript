use std::collections::VecDeque;

use super::{FullZipper, FullLeadingZipper, FullTrailingZipper};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T> FullZipper<T> {
    pub fn new() -> Self { Self::default() }
    pub fn fill_right(xs: impl IntoIterator<Item=T>) -> Self {
        let mut xs = xs.into_iter().collect::<VecDeque<_>>();
        if xs.is_empty() {
            unimplemented!("TODO")
        } else if xs.len() == 1 {
            let focus = xs.pop_front().unwrap();
            FullZipper {
                leading: FullLeadingZipper::Empty,
                focus: None,
                trailing: FullTrailingZipper::Saturated(focus, xs),
            }
        } else {
            let focus_1 = xs.pop_front().unwrap();
            let focus_2 = xs.pop_front().unwrap();
            FullZipper {
                leading: FullLeadingZipper::Empty,
                focus: Some(focus_1),
                trailing: FullTrailingZipper::Saturated(focus_2, xs),
            }
        }
    }
    pub fn fill_left(xs: impl IntoIterator<Item=T>) -> Self {
        let mut xs = xs.into_iter().collect::<VecDeque<_>>();
        if xs.is_empty() {
            unimplemented!("TODO")
        } else if xs.len() == 1 {
            let focus = xs.pop_back().unwrap();
            FullZipper {
                leading: FullLeadingZipper::Saturated(xs, focus),
                focus: None,
                trailing: FullTrailingZipper::Empty,
            }
        } else {
            let focus_1 = xs.pop_back().unwrap();
            let focus_2 = xs.pop_back().unwrap();
            FullZipper {
                leading: FullLeadingZipper::Saturated(xs, focus_2),
                focus: Some(focus_1),
                trailing: FullTrailingZipper::Empty,
            }
        }
    }
    // pub fn from_iter() -> Self {
    //     Self::default()
    // }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b c d _ e f
    /// ```
    pub fn forward(mut self) -> Self {
        match self.focus.take() {
            Some(center) => {
                match self.trailing.forward() {
                    (Some(right), trailing) => {
                        FullZipper {
                            leading: self.leading.forward(center),
                            focus: Some(right),
                            trailing,
                        }
                    },
                    (None, trailing) => {
                        FullZipper {
                            leading: self.leading.forward(center),
                            focus: None,
                            trailing,
                        }
                    },
                }
            }
            None => {
                match self.trailing.forward() {
                    (Some(right), trailing) => {
                        FullZipper {
                            leading: self.leading,
                            focus: Some(right),
                            trailing,
                        }
                    },
                    (None, trailing) => {
                        FullZipper {
                            leading: self.leading,
                            focus: None,
                            trailing,
                        }
                    },
                }
            }
        }
    }
    pub fn forward_mut(&mut self) where T: Clone {
        *self = self.clone().forward();
    }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b _ c d e f
    /// ```
    pub fn backward(mut self) -> Self {
        match self.focus.take() {
            Some(center) => {
                match self.leading.backward() {
                    (leading, Some(left)) => {
                        FullZipper {
                            leading,
                            focus: Some(left),
                            trailing: self.trailing.backward(center),
                        }
                    }
                    (leading, None) => {
                        FullZipper {
                            leading,
                            focus: None,
                            trailing: self.trailing.backward(center),
                        }
                    }
                }
            }
            None => {
                match self.leading.backward() {
                    (leading, Some(left)) => {
                        FullZipper {
                            leading,
                            focus: None,
                            trailing: self.trailing.backward(left),
                        }
                    }
                    (leading, None) => {
                        FullZipper {
                            leading,
                            focus: None,
                            trailing: self.trailing,
                        }
                    }
                }
            }
        }
    }
    pub fn backward_mut(&mut self) where T: Clone {
        *self = self.clone().backward();
    }
}



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T> FullLeadingZipper<T> {
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b c d _ e f
    /// ```
    pub fn forward(mut self, next: T) -> Self {
        match self {
            Self::Empty => Self::Singleton(next),
            Self::Singleton(x) => Self::Saturated(singleton(x), next),
            Self::Saturated(xs, x) => Self::Saturated(push_back(xs, x), next),
        }
    }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b _ c d e f
    /// ```
    pub fn backward(mut self) -> (Self, Option<T>) {
        match self {
            Self::Empty => Default::default(),
            Self::Singleton(x) => (Default::default(), Some(x)),
            Self::Saturated(ls, x) => {
                let ls = match unwrap_lastmost(ls) {
                    None => Self::Empty,
                    Some((ls, l)) if ls.is_empty() => Self::Singleton(l),
                    Some((ls, l)) => Self::Saturated(ls, l),
                };
                (ls, Some(x))
            }
        }
    }
    pub fn backward_mut(&mut self) -> Option<T> where T: Clone {
        let (r, x) = self.clone().backward();
        *self = r;
        x
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<T> FullTrailingZipper<T> {
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b c d _ e f
    /// ```
    pub fn forward(mut self) -> (Option<T>, Self) {
        match self {
            Self::Empty => Default::default(),
            Self::Singleton(x) => (Some(x), Self::Empty),
            Self::Saturated(x, rs) => {
                let rs  = match unwrap_firstmost(rs) {
                    Some((r, rs)) if rs.is_empty() => Self::Singleton(r),
                    Some((r, rs)) => Self::Saturated(r, rs),
                    None => Self::Empty,
                };
                (Some(x), rs)
            }
        }
    }
    pub fn forward_mut(&mut self) -> Option<T> where T: Clone {
        let (x, r) = self.clone().forward();
        *self = r;
        x
    }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b _ c d e f
    /// ```
    pub fn backward(self, next: T) -> Self {
        match self {
            Self::Empty => Self::Singleton(next),
            Self::Singleton(x) => Self::Saturated(next, singleton(x)),
            Self::Saturated(x, xs) => Self::Saturated(next, push_back(xs, x)),
        }
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

fn singleton<T>(x: T) -> VecDeque<T> {
    VecDeque::from([x])
}
fn push_front<T>(x: T, mut xs: VecDeque<T>) -> VecDeque<T> {
    xs.push_front(x);
    xs
}
fn push_back<T>(mut xs: VecDeque<T>, x: T) -> VecDeque<T> {
    xs.push_back(x);
    xs
}

fn unwrap_firstmost<T>(mut xs: VecDeque<T>) -> Option<(T, VecDeque<T>)> {
    if xs.is_empty() {
        return None;
    }
    let first = xs.pop_front().unwrap();
    Some((first, xs))
}

fn unwrap_lastmost<T>(mut xs: VecDeque<T>) -> Option<(VecDeque<T>, T)> {
    if xs.is_empty() {
        return None;
    }
    let last = xs.pop_back().unwrap();
    Some((xs, last))
}

fn unwrap_firstmost_mut<T>(mut xs: &mut VecDeque<T>) -> Option<T> {
    xs.pop_front()
}

fn unwrap_lastmost_mut<T>(mut xs: &mut VecDeque<T>) -> Option<T> {
    xs.pop_back()
}
