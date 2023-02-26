use super::{FullZipper, ZipperViewMut, FullLeadingZipper, LeadingZipperViewMut, FullTrailingZipper, TrailingZipperViewMut};

impl<T> FullZipper<T> {
    pub fn current_view<'a, C>(&'a mut self) -> ZipperViewMut<'a, T> {
        unimplemented!()
    }
}

impl<T> FullLeadingZipper<T> {
    pub fn current_view<'a, C>(&'a mut self) -> LeadingZipperViewMut<'a, T> {
        match self {
            Self::Empty => LeadingZipperViewMut::Empty,
            Self::Singleton(c) => LeadingZipperViewMut::Singleton { focus_left: c },
            Self::Saturated(xs, c) => LeadingZipperViewMut::Saturated { leftward: xs, focus_left: c },
        }
    }
}

impl<T> FullTrailingZipper<T> {
    pub fn current_view<'a, C>(&'a mut self) -> TrailingZipperViewMut<'a, T> {
        match self {
            Self::Empty => TrailingZipperViewMut::Empty,
            Self::Singleton(c) => TrailingZipperViewMut::Singleton { focus_right: c },
            Self::Saturated(c, xs)  => TrailingZipperViewMut::Saturated { focus_right: c, rightward: xs },
        }
    }
}
