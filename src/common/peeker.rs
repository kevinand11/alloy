#[derive(Clone, Debug)]
pub struct Peeker<T: Clone> {
    items: Vec<T>,
    iter_idx: usize,
}

impl<T: Clone> Peeker<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, iter_idx: 0 }
    }

    pub fn all(&self) -> &Vec<T> {
        &self.items
    }

    pub fn peek(&mut self) -> Option<(&T, usize)> {
        let idx = self.iter_idx;
        self.items.get(idx).map(|c| (c, idx))
    }

    pub fn next(&mut self) -> Option<(T, usize)> {
        let idx = self.iter_idx;
        self.items.get(idx).map(|c| {
            self.iter_idx = self.iter_idx + 1;
            (c.clone(), idx)
        })
    }
}
