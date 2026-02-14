use std::{path::PathBuf, str::Chars};

use crate::{common::span::Span, lexing::token::Token};

#[derive(Debug)]
pub struct Module {
    pub src: String,
    pub file_path: PathBuf,
}

type IterItem = (char, usize);

impl Module {
    pub fn new(src: String, file_path: PathBuf) -> Self {
        Self { src, file_path }
    }

    pub fn iter(&self) -> Chars<'_> {
        self.src.chars()
    }

    pub fn token(&self, token: &Token) -> &str {
        &self.src[token.span.0..token.span.1]
    }

    pub fn slice(&self, start: usize, end: usize) -> &str {
        // SAFETY: `start` and `end` are UTF-8 char indices, not byte indexes
        // for ascii, this should work fine though
        &self.src[start..end]
    }

    pub fn span_slice(&self, span: &Span) -> &str {
        self.slice(span.0, span.1)
    }

    pub fn ln(&self) -> usize {
        self.src.len()
    }
}
