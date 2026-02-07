#[derive(Debug)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub const fn from_range(start: usize, end: usize) -> Self {
        Self(start, end)
    }
}
