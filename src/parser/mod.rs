use std::iter::Peekable;

use crate::lexer::Lexer;
use ast::{Ast, AstError};

pub mod ast;
pub mod expression;
pub mod precedence;

pub struct Parser<'a> {
    src: &'a str,
    lexer: Peekable<Lexer<'a>>,
    in_fn: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let lexer = Lexer::new(src).peekable();
        Self {
            src,
            lexer,
            in_fn: false,
        }
    }

	pub fn parse(&mut self) -> Result<Ast, AstError> {
		Ok(todo!())
	}
}