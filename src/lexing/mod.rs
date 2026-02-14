use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use token::{Token, TokenKind::*};

use crate::{lexing::token::TokenKind, module::module::Module};

pub mod token;

pub struct Lexer<'a> {
    module: &'a Module,
    char_peeker: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(module: &'a Module) -> Self {
        Self {
            module: module,
            char_peeker: module.iter().enumerate().peekable(),
        }
    }

    fn consume(&self, kind: TokenKind, cur_idx: usize, size: usize) -> Token {
        Token::new(kind, &self.module.slice(cur_idx, cur_idx + size), cur_idx)
    }

    fn read_number(&mut self, start: usize) -> usize {
        let mut last = start;
        let mut seen_dot = false;

        while let Some((_, c)) = self.char_peeker.peek() {
            match *c {
                '0'..='9' => {
                    let (l, _) = self.char_peeker.next().unwrap();
                    last = l;
                }
                '_' => {
                    let (l, _) = self.char_peeker.next().unwrap();
                    last = l;
                }
                '.' if !seen_dot => {
                    let mut lookahead = self.char_peeker.clone();
                    lookahead.next();
                    if lookahead.peek().is_some_and(|(_, n)| n.is_ascii_digit()) {
                        let (l, _) = self.char_peeker.next().unwrap();
                        last = l;
                        seen_dot = true;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        last
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self
            .char_peeker
            .peek()
            .is_some_and(|(_, c)| c.is_whitespace())
        {
            self.char_peeker.next();
        }

        let Some((cur_idx, cur_char)) = self.char_peeker.peek() else {
            return None;
        };

        let cur_idx = *cur_idx;

        let mut call_next = true;

        let token = match cur_char {
            '#' => {
                self.char_peeker.next();
                let mut size = 1;
                while let Some((_, c)) = self.char_peeker.next() {
                    size = size + 1;
                    if c == '\n' {
                        break;
                    }
                }
                self.consume(Comment, cur_idx, size)
            }

            '+' => self.consume(Plus, cur_idx, 1),
            '-' => self.consume(Minus, cur_idx, 1),
            '*' => self.consume(Asterisk, cur_idx, 1),
            '/' => self.consume(Slash, cur_idx, 1),
            '^' => self.consume(Caret, cur_idx, 1),

            '=' => {
                self.char_peeker.next();
                if self.char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    self.consume(DoubleEquals, cur_idx, 2)
                } else {
                    call_next = false;
                    self.consume(Equals, cur_idx, 1)
                }
            }
            '<' => {
                self.char_peeker.next();
                if self.char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    self.consume(LessThanOrEqual, cur_idx, 2)
                } else {
                    call_next = false;
                    self.consume(LessThan, cur_idx, 1)
                }
            }
            '>' => {
                self.char_peeker.next();
                if self.char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    self.consume(GreaterThanOrEqual, cur_idx, 2)
                } else {
                    call_next = false;
                    self.consume(GreaterThan, cur_idx, 1)
                }
            }
            '!' => {
                self.char_peeker.next();
                if self.char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    self.consume(NotEquals, cur_idx, 2)
                } else {
                    call_next = false;
                    self.consume(Exclamation, cur_idx, 1)
                }
            }

            '{' => self.consume(LBrace, cur_idx, 1),
            '}' => self.consume(RBrace, cur_idx, 1),
            '(' => self.consume(LParen, cur_idx, 1),
            ')' => self.consume(RParen, cur_idx, 1),
            ':' => self.consume(Colon, cur_idx, 1),
            ',' => self.consume(Comma, cur_idx, 1),

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut last = cur_idx;
                while self
                    .char_peeker
                    .peek()
                    .is_some_and(|(_, c)| c.is_ascii_alphanumeric() || c == &'_')
                {
                    let (l, _) = self.char_peeker.next().unwrap();
                    last = l;
                }
                let chars = self.module.slice(cur_idx, last + 1);
                call_next = false;
                match chars {
                    "true" => self.consume(Boolean, cur_idx, 4),
                    "false" => self.consume(Boolean, cur_idx, 5),
                    "type" => self.consume(Type, cur_idx, 4),
                    _ => self.consume(Ident, cur_idx, chars.len()),
                }
            }
            '0'..='9' => {
                let last = self.read_number(cur_idx);
                call_next = false;
                self.consume(Number, cur_idx, last + 1 - cur_idx)
            }

            '.' => {
                self.char_peeker.next();
                call_next = false;
                if self
                    .char_peeker
                    .peek()
                    .is_some_and(|(_, c)| c.is_ascii_digit())
                {
                    let last = self.read_number(cur_idx);
                    self.consume(Number, cur_idx, last + 1 - cur_idx)
                } else {
                    self.consume(Dot, cur_idx, 1)
                }
            }

            _ => self.consume(Illegal, cur_idx, 1),
        };

        if call_next {
            self.char_peeker.next();
        }

        Some(token)
    }
}
