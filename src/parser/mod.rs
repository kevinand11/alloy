use std::iter::Peekable;

use crate::{lexer::{Lexer, token::{Token, TokenKind}}, parser::{expression::{Expression, ExpressionKind}, precedence::Precedence}};
use ast::{Ast, AstError};

pub mod ast;
pub mod expression;
pub mod precedence;

type LHParseFn = Box<dyn Fn(&mut Parser) -> Result<Expression, AstError>>;

pub struct Parser<'a> {
	lexer: Lexer<'a>,
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let tokens = lexer.clone().peekable();
        Self { lexer, tokens }
    }

	pub fn parse(&mut self) -> Result<Ast, AstError> {
		let mut exprs = vec!();
		while self.tokens.peek().is_some() {
			exprs.push(self.parse_expression(Precedence::Lowest)?);
		};
		Ok(Ast(exprs))
	}

	fn parse_expression(&mut self, _: Precedence) -> Result<Expression, AstError> {
		todo!()
	}

	fn get_lh_parse_fn(&mut self) -> Result<LHParseFn, AstError> {
		match self.peek()?.kind {
			TokenKind::LiteralInt => Ok(Box::new(|parser| parser.parse_literal_int())),
			_ => todo!()
		}
	}

	fn parse_literal_int(&mut self) -> Result<Expression, AstError> {
		let token = self.expect(TokenKind::LiteralInt)?;
        let span = token.span;
		let num = self.lexer.src.span_slice(&span).parse().unwrap();
		Ok(Expression::new(ExpressionKind::LiteralInt(num), span))
	}

	fn expect(&mut self, exp: TokenKind) -> Result<Token, AstError> {
		let kind = self.peek()?.kind.clone();
        if  kind == exp {
			self.tokens.next();
            Ok(self.cur()?)
        } else {
            Err((&AstError::expected)(exp, kind))
        }
	}

	fn cur(&mut self) -> Result<Token, AstError> {
		let token = self.tokens.next().ok_or(AstError::UnexpectedEof)?;
		Ok(token)
	}

	fn peek(&mut self) -> Result<&Token, AstError> {
		let token = self.tokens.peek().ok_or(AstError::UnexpectedEof)?;
		Ok(token)
	}

}