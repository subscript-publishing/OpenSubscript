use std::collections::VecDeque;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug)]
pub struct ZipperViewMut<'a, T, C=T> {
    pub leading: LeadingZipperViewMut<'a, T, C>,
    pub focus: &'a mut C,
    pub trailing: TrailingZipperViewMut<'a, T, C>,
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug)]
pub enum LeadingZipperViewMut<'a, T, C=T> {
    Empty,
    Singleton {
        focus_left: &'a mut C,
    },
    Saturated {
        leftward: &'a mut VecDeque<T>,
        focus_left: &'a mut C,
    },
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug)]
pub enum TrailingZipperViewMut<'a, T, C=T> {
    Empty,
    Singleton {
        focus_right: &'a mut C,
    },
    Saturated {
        focus_right: &'a mut C,
        rightward: &'a mut VecDeque<T>,
    },
}