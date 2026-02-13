use crate::lexing::token::{Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    NoPrefixParse(Token),
    Expected(Token, Vec<TokenKind>),
    Syntax(Token, String),
    UnexpectedEof,
}

impl ParseError {
    pub fn expected(token: &Token, exp: Vec<TokenKind>) -> Self {
        ParseError::Expected(token.clone(), exp)
    }

    pub fn syntax(token: &Token, s: &str) -> Self {
        ParseError::Syntax(token.clone(), format!("syntax error: {s}"))
    }

    pub fn eof() -> Self {
        ParseError::UnexpectedEof
    }

    pub fn no_prefix_parse(token: &Token) -> Self {
        ParseError::NoPrefixParse(token.clone())
    }
}
