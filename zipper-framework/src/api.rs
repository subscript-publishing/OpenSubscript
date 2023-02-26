use super::*;

impl<T, C> Default for Zipper<T, C> {
    fn default() -> Self {
        Zipper {
            leading: LeadingZipper::Empty,
            center: None,
            trailing: TrailingZipper::Empty,
        }
    }
}

impl<T, C> Zipper<T, C> {
    pub fn new() -> Self { Self::default() }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b c d _ e f
    /// ```
    pub fn forward(mut self, iso: Iso<T, C>) -> Result<Self, Self> {
        // match (self.current.take(), self.trailing.pop_front()) {
        //     (Some(current), Some(right)) => {
        //         self.leading.push_back(iso.from(current));
        //         self.current = Some(iso.to(right));
        //     }
        //     (Some(current), None) => {
        //         self.leading.push_back(iso.from(current));
        //     }
        //     (None, Some(right)) => {
        //         self.current = Some(iso.to(right));
        //     }
        //     (None, None) => {}
        // }
        // // self.current.as_mut()
        // Some(())
        unimplemented!()
    }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b _ c d e f
    /// ```
    pub fn backward(&mut self, iso: Iso<T, C>) -> Option<()> {
        // match (self.leading.pop_back(), self.current.take()) {
        //     (Some(leading), Some(current)) => {
        //         self.trailing.push_front(iso.from(current));
        //         self.current = Some(iso.to(leading));
        //     }
        //     (None, Some(current)) => {
        //         self.trailing.push_front(iso.from(current));
        //     }
        //     (Some(leading), None) => {
        //         self.current = Some(iso.to(leading));
        //     }
        //     (None, None) => {}
        // }
        // Some(())
        unimplemented!()
    }
    pub fn current_view<'a>(&'a mut self) -> Option<ZipperView<'a, T, C>> {
        if let Some(center) = self.center.as_mut() {
            return Some(ZipperView {
                leading: self.leading.as_view(),
                focus: center,
                trailing: self.trailing.as_view(),
            })
        }
        None
    }
}

impl<T, C> LeadingZipper<T, C> {
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b c d _ e f
    /// ```
    pub fn forward(self, iso: Iso<T, C>, new: C) -> Self {
        match self {
            Self::Empty => Self::Singleton { focus_left: new },
            Self::Singleton { focus_left } => Self::Saturated {
                leftward: vec![iso.from(focus_left)],
                focus_left: new,
            },
            Self::Saturated { leftward, focus_left } if leftward.is_empty() => Self::Saturated {
                leftward: vec![
                    iso.from(focus_left)
                ],
                focus_left: new,
            },
            Self::Saturated { mut leftward, focus_left } => Self::Saturated {
                leftward: {
                    leftward.push(iso.from(focus_left));
                    leftward
                },
                focus_left: new,
            },
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
    pub fn backward(self, iso: Iso<T, C>) -> (LeadingZipper<T, C>, Option<C>) {
        match self {
            Self::Empty => {
                (Self::Empty, None)
            }
            Self::Singleton { focus_left } => {
                (Self::Empty, Some(focus_left))
            }
            Self::Saturated { mut leftward, focus_left } if leftward.is_empty() => {
                (Self::Empty, Some(focus_left))
            }
            Self::Saturated { mut leftward, focus_left } if leftward.len() == 1 => {
                let new_focus = leftward.pop().map(|x| iso.to(x)).unwrap();
                (Self::Singleton { focus_left: new_focus }, Some(focus_left))
            }
            Self::Saturated { mut leftward, focus_left } => {
                let new_focus = leftward.pop().map(|x| iso.to(x)).unwrap();
                (Self::Saturated { leftward, focus_left: new_focus }, Some(focus_left))
            }
        }
    }
    pub fn as_view<'a>(&'a mut self) -> ZipperLeadingView<'a, T, C> {
        match self {
            Self::Empty => ZipperLeadingView::Empty,
            Self::Singleton { focus_left } => ZipperLeadingView::Singleton { focus_left },
            Self::Saturated { leftward, focus_left } => ZipperLeadingView::Saturated {
                leftward: leftward.as_mut_slice(),
                focus_left
            },
        }
    }
}

impl<T, C> TrailingZipper<T, C> {
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b _ c d e f
    /// ```
    pub fn backward(self, iso: Iso<T, C>, new: C) -> Self {
        match self {
            Self::Empty => Self::Singleton { focus_right: new },
            Self::Singleton { focus_right } => Self::Saturated {
                focus_right: new,
                rightward: vec![
                    iso.from(focus_right),
                ],
            },
            Self::Saturated { focus_right, mut rightward } if rightward.is_empty() => Self::Saturated {
                focus_right: new,
                rightward: vec![
                    iso.from(focus_right),
                ],
            },
            Self::Saturated { focus_right, mut rightward } => Self::Saturated {
                focus_right: unimplemented!(),
                rightward: unimplemented!(),
            },
        }
    }
    /// Initial State
    /// ```text
    /// a b c _ d e f
    /// ```
    /// Final State
    /// ```text
    /// a b c d _ e f
    /// ```
    pub fn forward(self, iso: Iso<T, C>) -> (Option<C>, TrailingZipper<T, C>) {
        match self {
            Self::Empty => {
                (None, Self::Empty)
            }
            Self::Singleton { focus_right } => {
                (Some(focus_right), Self::Empty)
            }
            Self::Saturated { focus_right, rightward } if rightward.is_empty() => {
                (Some(focus_right), Self::Empty)
            }
            Self::Saturated { focus_right, mut rightward } if rightward.len() == 1 => {
                let new_focus = iso.to(rightward.remove(0));
                (Some(focus_right), Self::Singleton { focus_right: new_focus })
            }
            Self::Saturated { focus_right, mut rightward } => {
                let new_focus = iso.to(rightward.remove(0));
                (Some(focus_right), Self::Saturated { focus_right: new_focus, rightward })
            }
        }
    }
    pub fn as_view<'a>(&'a mut self) -> ZipperTrailingView<'a, T, C> {
        match self {
            Self::Empty => ZipperTrailingView::Empty,
            Self::Singleton { focus_right } => ZipperTrailingView::Singleton { focus_right },
            Self::Saturated { focus_right, rightward } => ZipperTrailingView::Saturated {
                focus_right,
                rightward: rightward.as_mut_slice(),
            },
        }
    }
}