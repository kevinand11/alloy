#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self(start, end)
    }

    pub fn to(&self, other: &Self) -> Self {
        Span::new(self.0, other.1)
    }
}
