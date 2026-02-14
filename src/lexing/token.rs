use crate::common::span::Span;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String
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
    Exclamation,
    Colon,
    Equals,
    Dot,
    LParen,
    RParen,
    Comma,

    Number,
    Boolean,
    Ident,
    Type,

    Comment,
    Illegal,
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, text: &str, position: usize) -> Self {
        Token {
            kind,
            text: text.to_string(),
            span: Span::new(position, position + text.len()),
        }
    }
}
