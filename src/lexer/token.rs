use crate::common::span::Span;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,

    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    DoubleEquals,
    NotEquals,

    LBrace,
    RBrace,
    Exclamation,

    LiteralInt,
    LiteralFloat,
    Indentifier,

    Comment,
    Illegal,
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, position: usize, len: usize) -> Self {
        Token {
            kind,
            span: Span::new(position, position + len),
        }
    }
}
