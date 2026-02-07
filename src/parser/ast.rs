use super::expression::Expression;

use crate::lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Ast(pub Vec<Expression>); // restructure to global scope expressions and main entry fn

pub enum AstError {
    NoPrefixParse(Token),
    Expected(Token, TokenKind),
    Syntax(Token, String),
    UnexpectedEof,
}

impl AstError {
    pub fn expected(token: Token, exp: TokenKind) -> Self {
        AstError::Expected(token, exp)
    }

    pub fn syntax(token: Token, s: &str) -> Self {
        AstError::Syntax(token, format!("syntax error: {s}"))
    }

    pub fn eof() -> Self {
        AstError::UnexpectedEof
    }

    pub fn no_prefix_parse(token: Token) -> Self {
        AstError::NoPrefixParse(token)
    }
}
