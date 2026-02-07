#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn from_range(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
