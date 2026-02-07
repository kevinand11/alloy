use std::iter::Peekable;

use crate::{
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::{
        expression::{Expression, ExpressionKind, InfixOp},
        precedence::Precedence,
    },
};
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
        let tokens = lexer.get_peekable();
        Self { lexer, tokens }
    }

    pub fn parse(&mut self) -> Result<Ast, AstError> {
        let mut exprs = vec![];
        while self.tokens.peek().is_some() {
            exprs.push(self.parse_expression(Precedence::Lowest)?);
        }
        Ok(Ast(exprs))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, AstError> {
		let mut expr = self.get_lh_parse_fn()?(self)?;

        while Precedence::of(self.peek_kind()) > precedence {

            expr = match self.peek_kind() {
                TokenKind::LiteralInt | TokenKind::LiteralFloat => {
                    unreachable!("lexed 2 numbers next to each other")
                }

                TokenKind::Plus => self.parse_infix_expression(expr, InfixOp::Add)?,
                TokenKind::Minus => self.parse_infix_expression(expr, InfixOp::Subtract)?,
                TokenKind::Asterisk => self.parse_infix_expression(expr, InfixOp::Multiply)?,
                TokenKind::Slash => self.parse_infix_expression(expr, InfixOp::Divide)?,
                TokenKind::Caret => self.parse_infix_expression(expr, InfixOp::Power)?,
                TokenKind::LessThan => self.parse_infix_expression(expr, InfixOp::LessThan)?,
                TokenKind::LessThanOrEqual => {
                    self.parse_infix_expression(expr, InfixOp::LessThanOrEqual)?
                }
                TokenKind::GreaterThan => {
                    self.parse_infix_expression(expr, InfixOp::GreaterThan)?
                }
                TokenKind::GreaterThanOrEqual => {
                    self.parse_infix_expression(expr, InfixOp::GreaterThanOrEqual)?
                }
                TokenKind::DoubleEquals => self.parse_infix_expression(expr, InfixOp::Equals)?,
                TokenKind::NotEquals => self.parse_infix_expression(expr, InfixOp::NotEquals)?,

                TokenKind::LBrace => todo!(),
                TokenKind::RBrace => todo!(),

                TokenKind::Indentifier => todo!(),
                TokenKind::Comment => {
                    self.consume()?;
                    self.parse_expression(precedence.clone())?
                }

                _ => return Err(AstError::syntax(self.consume()?, "illegal token found")),
            };
        }

        Ok(expr)
    }

    fn parse_infix_expression(
        &mut self,
        lhs: Expression,
        op: InfixOp,
    ) -> Result<Expression, AstError> {
        let token = self.consume()?;
        let precedence = Precedence::of(&token.kind);
        let rhs = self.parse_expression(precedence)?;
        let span = lhs.span.to(&rhs.span);
        Ok(Expression::new(
            ExpressionKind::Infix {
                op,
                lh: Box::new(lhs),
                rh: Box::new(rhs),
            },
            span,
        ))
    }

    fn get_lh_parse_fn(&mut self) -> Result<LHParseFn, AstError> {
        match self.peek_kind() {
            TokenKind::Comment => {
                self.expect(TokenKind::Comment)?;
                self.get_lh_parse_fn()
            }
            TokenKind::LiteralInt => Ok(Box::new(|parser| parser.parse_literal_number(false))),
            TokenKind::LiteralFloat => Ok(Box::new(|parser| parser.parse_literal_number(true))),
            _ => Err(AstError::no_prefix_parse(self.consume()?)),
        }
    }

    fn parse_literal_number(&mut self, float: bool) -> Result<Expression, AstError> {
        let token = self.expect(if float {
            TokenKind::LiteralFloat
        } else {
            TokenKind::LiteralInt
        })?;
        let span = token.span;
        if float {
            let num = self.lexer.module.span_slice(&span).parse().unwrap();
            Ok(Expression::new(ExpressionKind::LiteralFloat(num), span))
        } else {
            let num = self.lexer.module.span_slice(&span).parse().unwrap();
            Ok(Expression::new(ExpressionKind::LiteralInt(num), span))
        }
    }

    fn expect(&mut self, exp: TokenKind) -> Result<Token, AstError> {
        let token = self.consume()?;
        if token.kind == exp {
            let token = self.tokens.next().ok_or(AstError::eof())?;
            Ok(token)
        } else {
            Err((&AstError::expected)(token.clone(), exp))
        }
    }

    fn consume(&mut self) -> Result<Token, AstError> {
        self.tokens.next().ok_or(AstError::eof())
    }

    fn peek_kind(&mut self) -> &TokenKind {
        self.tokens.peek().map_or(&TokenKind::Eof, |t| &t.kind)
    }
}
