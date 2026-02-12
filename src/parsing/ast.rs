use super::expression::Expression;

use crate::lexing::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Ast(pub Vec<Expression>); // restructure to global scope expressions and main entry fn

#[derive(Debug)]
pub enum AstError {
    NoPrefixParse(Token),
    Expected(Token, Vec<TokenKind>),
    Syntax(Token, String),
    UnexpectedEof,
}

impl AstError {
    pub fn expected(token: &Token, exp: Vec<TokenKind>) -> Self {
        AstError::Expected(token.clone(), exp)
    }

    pub fn syntax(token: &Token, s: &str) -> Self {
        AstError::Syntax(token.clone(), format!("syntax error: {s}"))
    }

    pub fn eof() -> Self {
        AstError::UnexpectedEof
    }

    pub fn no_prefix_parse(token: &Token) -> Self {
        AstError::NoPrefixParse(token.clone())
    }
}
