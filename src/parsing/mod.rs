use std::iter::Peekable;

use crate::{
    common::span::Span,
    lexing::{
        Lexer, TokenIter,
        token::{Token, TokenKind},
    },
    parsing::{
        expression::{Expression, ExpressionKind, InfixOp, PrefixOp, TypeIdent},
        precedence::Precedence,
    },
};
use ast::{Ast, AstError};

pub mod ast;
pub mod expression;
pub mod precedence;

pub struct Parser<'a> {
    pub lexer: &'a Lexer<'a>,
    tokens: Peekable<TokenIter<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        let tokens = lexer.iter().peekable();
        Self { lexer, tokens }
    }

    pub fn parse(&mut self) -> Result<Ast, AstError> {
        let mut exprs = vec![];
        while self.tokens.peek().is_some() {
            exprs.push(self.parse_expression(&Precedence::Lowest)?);
        }
        Ok(Ast(exprs))
    }

    fn parse_expression(&mut self, precedence: &Precedence) -> Result<Expression, AstError> {
        let mut expr = self.get_first_expression()?;

        while &Precedence::of(self.peek_kind()) > precedence {
            expr = match self.peek_kind() {
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

                TokenKind::Comment => {
                    self.consume()?;
                    self.parse_expression(precedence)?
                }

                TokenKind::Dot => {
                    self.consume()?;
                    let fn_call = self.parse_function_call(&expr.span)?;
                    let total_span = expr.span.to(&fn_call.span);
                    match fn_call.kind {
                        ExpressionKind::FunctionCall { name, args } => Expression::new(
                            ExpressionKind::MethodCall {
                                caller: Box::new(expr),
                                name,
                                args,
                            },
                            total_span,
                        ),
                        _ => unreachable!(),
                    }
                }

                _ => return Err(AstError::syntax(&self.consume()?, "illegal token found")),
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
        let rhs = self.parse_expression(&precedence)?;
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

    fn get_first_expression(&mut self) -> Result<Expression, AstError> {
        match self.peek_kind() {
            TokenKind::Comment => {
                self.consume()?;
                self.get_first_expression()
            }
            TokenKind::Boolean => self.parse_boolean(),
            TokenKind::Number => self.parse_number(),
            TokenKind::Exclamation => self.parse_prefix_expression(PrefixOp::Not),
            TokenKind::LBrace => self.parse_block_expression(),
            TokenKind::Ident => {
                let token = self.consume()?;
                match self.peek_kind() {
                    TokenKind::Colon => self.parse_variable_declaration(token),
                    TokenKind::Equals => self.parse_variable_assignment(token),
                    _ => self.parse_variable_usage(token),
                }
            }
            _ => Err(AstError::no_prefix_parse(&self.consume()?)),
        }
    }

    fn parse_block_expression(&mut self) -> Result<Expression, AstError> {
        let start = self.expect(TokenKind::LBrace)?;
        let mut exprs = vec![];
        while self.peek_kind() != &TokenKind::RBrace {
            exprs.push(self.parse_expression(&Precedence::Lowest)?);
        }
        let end = self.expect(TokenKind::RBrace)?;
        Ok(Expression::new(
            ExpressionKind::Block(exprs),
            start.span.to(&end.span),
        ))
    }

    fn parse_prefix_expression(&mut self, op: PrefixOp) -> Result<Expression, AstError> {
        let start = self.expect(TokenKind::Exclamation)?;
        let expr = self.parse_expression(&Precedence::Prefix)?;
        let new_span = start.span.to(&expr.span);
        Ok(Expression::new(
            ExpressionKind::Prefix {
                op,
                rh: Box::new(expr),
            },
            new_span,
        ))
    }

    fn parse_number(&mut self) -> Result<Expression, AstError> {
        let token = self.expect(TokenKind::Number)?;

        let raw = self.lexer.module.span_slice(&token.span);
        let cleaned: String = raw.chars().filter(|c| *c != '_').collect();

        if cleaned.contains('.') {
            let num: f32 = cleaned
                .parse()
                .map_err(|_| AstError::syntax(&token, "invalid float"))?;
            Ok(Expression::new(ExpressionKind::LiteralFloat(num), token.span))
        } else {
            let num: isize = cleaned
                .parse()
                .map_err(|_| AstError::syntax(&token, "invalid int"))?;
            Ok(Expression::new(ExpressionKind::LiteralInt(num), token.span))
        }
    }

    fn parse_boolean(&mut self) -> Result<Expression, AstError> {
        let token = self.expect(TokenKind::Boolean)?;

        let value = match self.lexer.module.token(&token) {
            "true" => true,
            "false" => false,
            _ => return Err(AstError::syntax(&token, "invalid bool")),
        };

        Ok(Expression::new(ExpressionKind::LiteralBool(value), token.span))
    }

    fn parse_type(&mut self) -> Result<TypeIdent, AstError> {
        let token = self.expect(TokenKind::Ident)?;
        Ok(TypeIdent(self.lexer.module.token(&token).to_string()))
    }

    fn parse_variable_declaration(&mut self, start: Token) -> Result<Expression, AstError> {
        let name = self.lexer.module.token(&start).to_string();
        self.consume()?;
        let mut ty = None;
        if self.peek_kind() == &TokenKind::Ident {
            ty = Some(self.parse_type()?);
        }

        let mutable = match self.peek_kind() {
            TokenKind::Colon => false,
            TokenKind::Equals => true,
            _ => {
                return Err(AstError::expected(
                    &start,
                    vec![TokenKind::Colon, TokenKind::Equals],
                ));
            }
        };

        self.consume()?;
        let value = self.parse_expression(&Precedence::Lowest)?;
        let span = start.span.to(&value.span);
        Ok(Expression::new(
            ExpressionKind::VariableDecl {
                name,
                value: Box::new(value),
                mutable,
                ty,
            },
            span,
        ))
    }

    fn parse_variable_assignment(&mut self, start: Token) -> Result<Expression, AstError> {
        let name = self.lexer.module.token(&start).to_string();
        self.consume()?;
        let value = self.parse_expression(&Precedence::Lowest)?;
        let span = start.span.to(&value.span);
        Ok(Expression::new(
            ExpressionKind::VariableAssignment {
                name,
                value: Box::new(value),
            },
            span,
        ))
    }

    fn parse_variable_usage(&mut self, start: Token) -> Result<Expression, AstError> {
        let name = (*self).lexer.module.token(&start).to_string();
        Ok(Expression::new(ExpressionKind::Ident(name), start.span))
    }

    fn parse_function_call(&mut self, start: &Span) -> Result<Expression, AstError> {
        let ident = self.expect(TokenKind::Ident)?;
        self.expect(TokenKind::LParen)?;
        let mut args = vec![];
        loop {
            match self.peek_kind() {
                TokenKind::RParen => break,
                _ => {
                    args.push(self.parse_expression(&Precedence::Lowest)?);
                    if self.peek_kind() == &TokenKind::Comma {
                        self.consume()?;
                    }
                },
            }
        };
        let end = self.expect(TokenKind::RParen)?;
        let span = start.to(&end.span);
        Ok(Expression::new(
            ExpressionKind::FunctionCall {
                name: self.lexer.module.token(&ident).to_string(),
                args,
            },
            span,
        ))
    }

    fn expect(&mut self, exp: TokenKind) -> Result<Token, AstError> {
        let token = self.consume()?;
        if token.kind == exp {
            Ok(token)
        } else {
            Err((&AstError::expected)(&token, vec![exp]))
        }
    }

    fn consume(&mut self) -> Result<Token, AstError> {
        Ok(self.tokens.next().ok_or(AstError::eof())?)
    }

    fn peek_kind(&mut self) -> &TokenKind {
        self.tokens.peek().map_or(&TokenKind::Eof, |t| &t.kind)
    }
}
