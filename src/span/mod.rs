#[derive(Clone, Debug)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub const fn from_range(start: usize, end: usize) -> Self {
        Self(start, end)
    }

    pub fn to(&self, other: &Self) -> Self {
        Span::from_range(self.0, other.1)
    }
}
