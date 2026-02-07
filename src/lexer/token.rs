use crate::span::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Asterisk,
    Slash,

    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    DoubleEqual,
    NotEqual,

    LBrace,
    RBrace,
    Hash,

    LiteralInt,
    Indentifier,

    Illegal,
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, (start, end): (usize, usize)) -> Self {
        Token {
            kind,
            span: Span::from_range(start, end),
        }
    }
}
