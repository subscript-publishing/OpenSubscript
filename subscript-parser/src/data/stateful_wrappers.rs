use std::{rc::Rc, cell::RefCell};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// MutState
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug)]
pub struct MutState<T>(pub Rc<RefCell<T>>);

impl<T> Clone for MutState<T> {
    fn clone(&self) -> Self {
        MutState(self.0.clone())
    }
}
impl<T> MutState<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
    pub fn map_mut<U>(&self, f: impl FnOnce(&mut T) -> U) -> U { f(&mut self.0.borrow_mut()) }
    pub fn map_ref<U>(&self, f: impl FnOnce(&T) -> U) -> U { f(&self.0.borrow()) }
}
impl<T: Default> Default for MutState<T> {
    fn default() -> Self {
        MutState(Rc::new(RefCell::new(T::default())))
    }
}