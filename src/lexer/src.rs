use std::{iter::Peekable, str::CharIndices};

use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Src<'a>(pub &'a str);

impl<'a> Src<'a> {
    pub const fn from(src: &'a str) -> Self {
        Self(src)
    }

    pub fn chars(&self) -> Peekable<CharIndices<'a>> {
        self.0.char_indices().peekable()
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        // SAFETY: `start` and `end` are UTF-8 char indices, not byte indexes
        // for ascii, this should work fine though
        &self.0[start..end]
    }

    pub fn span_slice(&self, span: &Span) -> &'a str {
        self.slice(span.0, span.1)
    }

    pub fn ln(&self) -> usize {
        self.0.len()
    }
}
