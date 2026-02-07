use crate::span::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
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
