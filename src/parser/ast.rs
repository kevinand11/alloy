use super::expression::Expression;

use crate::lexer::token::TokenKind;

#[derive(Debug)]
pub struct Ast(pub Vec<Expression>); // restructure to global scope expressions and main entry fn

pub enum AstError {
    NoToken,
    NoPrefixParse(TokenKind),
    Expected { exp: TokenKind, got: TokenKind },
    Syntax(String),
    UnexpectedEof,
    IllegalGlobalExpression(Expression),
}

impl AstError {
    pub fn expected(exp: TokenKind, got: TokenKind) -> Self {
        Self::Expected { exp, got }
    }

    pub fn syntax_err(s: &str) -> Self {
        Self::Syntax(format!("syntax error: {s}"))
    }
}
