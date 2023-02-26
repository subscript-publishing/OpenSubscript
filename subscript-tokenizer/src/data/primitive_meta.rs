//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn range_to(self, other: Position) -> Range {
        Range::from(self, other)
    }
    pub fn to_singleton_range(self) -> Range {
        Range::single(self)
    }
    pub const ZERO: Position = Position{index: 0, line: 0, column: 0};
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub position: Position,
    pub length: usize,
}

impl Range {
    pub fn single(position: Position) -> Self { Self { position: position, length: 0 }}
    pub fn from(start: Position, end: Position) -> Self {
        let length = end.index - start.index;
        Self { position: start, length }
    }
}