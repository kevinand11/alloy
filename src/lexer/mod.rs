use std::{iter::Peekable, str::CharIndices};

use token::{Token, TokenKind, TokenKind::*};

use crate::lexer::module::Module;

pub mod module;
pub mod token;

#[derive(Clone)]
pub struct Lexer<'a> {
    pub module: &'a Module<'a>,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(module: &'a Module) -> Self {
        let chars = module.get_peeker();
        Self { module, chars }
    }

    pub fn get_peekable(&self) -> Peekable<Self> {
        // TODO: quite expensive, refactor
        self.clone().peekable()
    }

    fn read_token(&mut self) -> Token {
        while self.peek().is_some_and(|(_, c)| c.is_whitespace()) {
            self.next();
        }

        let Some((cur_idx, cur_char)) = self.peek() else {
            let src_ln = self.module.ln();
            return Token::new(Eof, (src_ln, src_ln));
        };

        match cur_char {
            '#' => {
                self.next();
                let mut size = 1;
                while let Some((_, c)) = self.peek() {
                    size = size + 1;
                    self.next();
                    if c == '\n' {
                        break;
                    }
                }
                self.consume_token(Comment, cur_idx, size)
            }

            '+' => self.consume_token(Plus, cur_idx, 1),
            '-' => self.consume_token(Minus, cur_idx, 1),
            '*' => self.consume_token(Asterisk, cur_idx, 1),
            '/' => self.consume_token(Slash, cur_idx, 1),
            '^' => self.consume_token(Caret, cur_idx, 1),

            '=' => {
                self.next();
                if self.peek().is_some_and(|(_, char)| char == '=') {
                    self.consume_token(DoubleEquals, cur_idx, 2)
                } else {
                    self.consume_token(Illegal, cur_idx, 1)
                }
            }
            '<' => {
                self.next();
                if self.peek().is_some_and(|(_, char)| char == '=') {
                    self.consume_token(LessThanOrEqual, cur_idx, 2)
                } else {
                    self.consume_token(LessThan, cur_idx, 1)
                }
            }
            '>' => {
                self.next();
                if self.peek().is_some_and(|(_, char)| char == '=') {
                    self.consume_token(GreaterThanOrEqual, cur_idx, 2)
                } else {
                    self.consume_token(GreaterThan, cur_idx, 1)
                }
            }
            '!' => {
                self.next();
                if self.peek().is_some_and(|(_, char)| char == '=') {
                    self.consume_token(NotEquals, cur_idx, 2)
                } else {
                    self.consume_token(Illegal, cur_idx, 1)
                }
            }

            '{' => self.consume_token(LBrace, cur_idx, 1),
            '}' => self.consume_token(RBrace, cur_idx, 1),

            'a'..='z' | 'A'..='Z' | '_' => {
                let chars = self.read_identifier(cur_idx);
                Token::new(Indentifier, (cur_idx, cur_idx + chars.len()))
            }
            '0'..='9' => {
                let chars = self.read_number(cur_idx);
                Token::new(LiteralInt, (cur_idx, cur_idx + chars.len()))
            }

            _ => self.consume_token(Illegal, cur_idx, 1),
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn consume_token(&mut self, kind: TokenKind, position: usize, len: usize) -> Token {
        let token = Token::new(kind, (position, position + len));
        self.next();
        token
    }

    fn read_number(&mut self, position: usize) -> &str {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
            let (l, _) = self.next().unwrap();
            last = l;
        }
        &self.module.slice(position, last + 1)
    }

    fn read_identifier(&mut self, position: usize) -> &str {
        let mut last = position;
        while self
            .peek()
            .is_some_and(|(_, c)| c.is_ascii_alphanumeric() || c == '_')
        {
            let (l, _) = self.next().unwrap();
            last = l;
        }
        &self.module.slice(position, last + 1)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.read_token();
        match tok.kind {
            TokenKind::Eof => None,
            _ => Some(tok),
        }
    }
}
