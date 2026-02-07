use super::expression::Expression;

use crate::lexer::token::TokenKind;

#[derive(Debug)]
pub struct Ast(pub Vec<Expression>); // restructure to global scope expressions and main entry fn

pub enum AstError {
    NoToken,
    NoPrefixParse(TokenKind),
    Expected { exp: String, got: String },
    Syntax(String),
    UnexpectedEof,
    IllegalGlobalExpression(Expression),
}

impl AstError {
    pub fn expected<'a>(exp: &str, got: &str) -> Self {
        Self::Expected {
            exp: exp.to_string(),
            got: got.to_string(),
        }
    }

    pub fn syntax_err(s: &str) -> Self {
        Self::Syntax(format!("syntax error: {s}"))
    }
}
