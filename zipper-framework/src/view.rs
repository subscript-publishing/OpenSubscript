#[derive(Debug)]
pub struct ZipperView<'a, T, C=T> {
    pub leading: ZipperLeadingView<'a, T, C>,
    pub focus: &'a mut C,
    pub trailing: ZipperTrailingView<'a, T, C>,
}

#[derive(Debug)]
pub enum ZipperLeadingView<'a, T, C=T> {
    Empty,
    Singleton {
        focus_left: &'a mut C,
    },
    Saturated {
        leftward: &'a mut [T],
        focus_left: &'a mut C,
    },
}

#[derive(Debug)]
pub enum ZipperTrailingView<'a, T, C=T> {
    Empty,
    Singleton {
        focus_right: &'a mut C,
    },
    Saturated {
        focus_right: &'a mut C,
        rightward: &'a mut [T],
    },
}