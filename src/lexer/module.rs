use std::{iter::Peekable, str::CharIndices};

use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Module<'a>(pub &'a str);

impl<'a> Module<'a> {
    pub const fn new(src: &'a str) -> Self {
        Self(src)
    }

    pub fn content(&self) -> &str {
        self.0.as_ref()
    }

    pub fn slice(&self, start: usize, end: usize) -> &str {
        // SAFETY: `start` and `end` are UTF-8 char indices, not byte indexes
        // for ascii, this should work fine though
        &self.0[start..end]
    }

    pub fn span_slice(&self, span: &Span) -> &str {
        self.slice(span.0, span.1)
    }

    pub fn ln(&self) -> usize {
        self.0.len()
    }

    pub fn get_peeker(&self) -> Peekable<CharIndices<'a>> {
        self.0.char_indices().peekable()
    }
}
