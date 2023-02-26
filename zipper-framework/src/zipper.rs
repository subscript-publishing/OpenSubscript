use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Zipper<T, C=T> {
    pub leading: LeadingZipper<T, C>,
    pub center: Option<C>,
    pub trailing: TrailingZipper<T, C>,
}

#[derive(Debug, Clone)]
pub enum LeadingZipper<T, C=T> {
    Empty,
    Singleton {
        focus_left: C,
    },
    Saturated {
        leftward: Vec<T>,
        focus_left: C,
    },
}

#[derive(Debug, Clone)]
pub enum TrailingZipper<T, C=T> {
    Empty,
    Singleton {
        focus_right: C,
    },
    Saturated {
        focus_right: C,
        rightward: Vec<T>,
    },
}
impl<T> Iso<T, T> {
    pub fn identity() -> Self { Self::new(|x| x, |x| x) }
}
pub struct Iso<A, B> {
    pub f: Rc<dyn Fn(A) -> B>,
    pub g: Rc<dyn Fn(B) -> A>,
}

impl<A, B> Clone for Iso<A, B> {
    fn clone(&self) -> Self {
        Iso {
            f: self.f.clone(),
            g: self.g.clone(),
        }
    }
}

impl<A, B> Iso<A, B> {
    pub fn new(to: impl Fn(A) -> B + 'static, from: impl Fn(B) -> A + 'static) -> Self {
        let to = Rc::new(move |x| to(x));
        let from = Rc::new(move |x| from(x));
        Self { f: to, g: from }
    }
    pub fn to(&self, x: A) -> B { (self.f)(x) }
    pub fn from(&self, y: B) -> A { (self.g)(y) }
}
