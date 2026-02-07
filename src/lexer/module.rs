use crate::{common::{peeker::Peeker, span::Span}, lexer::token::Token};

#[derive(Clone, Debug)]
pub struct Module {
    pub src: String,
}

type IterItem = (char, usize);

impl Module {
    pub fn new(src: String) -> Self {
        Self { src }
    }

    pub fn get_peeker(&self) -> Peeker<char> {
        let chars: Vec<char> = self.src.chars().collect();
        Peeker::new(chars)
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
