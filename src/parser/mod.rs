use crate::{
    common::peeker::Peeker,
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::{
        expression::{Expression, ExpressionKind, InfixOp, PrefixOp},
        precedence::Precedence,
    },
};
use ast::{Ast, AstError};

pub mod ast;
pub mod expression;
pub mod precedence;

type LHParseFn = Box<dyn Fn(&mut Parser) -> Result<Expression, AstError>>;

pub struct Parser {
    lexer: Lexer,
    tokens: Peeker<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let tokens = lexer.get_peeker();
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
                TokenKind::Number => {
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
            TokenKind::Number => Ok(Box::new(|parser| parser.parse_literal_number())),
            TokenKind::Exclamation => Ok(Box::new(|parser| {
                parser.parse_prefix_expression(PrefixOp::Not)
            })),
            TokenKind::LBrace => Ok(Box::new(|parser| parser.parse_block_expression())),
            _ => Err(AstError::no_prefix_parse(self.consume()?)),
        }
    }

    fn parse_block_expression(&mut self) -> Result<Expression, AstError> {
        let start = self.expect(TokenKind::LBrace)?;
        let mut exprs = vec![];
        while self.peek_kind() != &TokenKind::RBrace {
            exprs.push(self.parse_expression(Precedence::Lowest)?);
        }
        let end = self.expect(TokenKind::RBrace)?;
        Ok(Expression::new(
            ExpressionKind::Block(exprs),
            start.span.to(&end.span),
        ))
    }

    fn parse_prefix_expression(&mut self, op: PrefixOp) -> Result<Expression, AstError> {
        let start = self.expect(TokenKind::Exclamation)?;
        let expr = self.parse_expression(Precedence::Prefix)?;
        let new_span = start.span.to(&expr.span);
        Ok(Expression::new(
            ExpressionKind::Prefix {
                op,
                rh: Box::new(expr),
            },
            new_span,
        ))
    }

    fn parse_literal_number(&mut self) -> Result<Expression, AstError> {
        let token = self.expect(TokenKind::Number)?;
		let span = token.span;

        let raw = self.lexer.module.span_slice(&span);
        let cleaned: String = raw.chars().filter(|c| *c != '_').collect();

        if cleaned.contains('.') {
            let num: f32 = cleaned
                .parse()
                .map_err(|_| AstError::syntax(token, "invalid float"))?;
            Ok(Expression::new(ExpressionKind::LiteralFloat(num), span))
        } else {
            let num: isize = cleaned
                .parse()
                .map_err(|_| AstError::syntax(token, "invalid int"))?;
            Ok(Expression::new(ExpressionKind::LiteralInt(num), span))
        }
    }

    fn expect(&mut self, exp: TokenKind) -> Result<Token, AstError> {
        let token = self.consume()?;
        if token.kind == exp {
            Ok(token)
        } else {
            Err((&AstError::expected)(token, exp))
        }
    }

    fn consume(&mut self) -> Result<Token, AstError> {
        Ok(self.tokens.next().ok_or(AstError::eof())?.0)
    }

    fn peek_kind(&mut self) -> &TokenKind {
        self.tokens.peek().map_or(&TokenKind::Eof, |(t, _)| &t.kind)
    }
}
