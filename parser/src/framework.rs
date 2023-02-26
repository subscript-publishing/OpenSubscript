use std::rc::Rc;

pub struct StringSpan {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

pub enum Item {
    Plain(StringSpan),
    Token
}


