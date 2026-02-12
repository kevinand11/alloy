use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use token::{Token, TokenKind::*};

use crate::lexing::module::Module;

pub mod module;
pub mod token;

pub struct Lexer<'a> {
    pub module: &'a Module,
}

pub struct TokenIter<'a> {
    lexer: &'a Lexer<'a>,
    char_peeker: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(module: &'a Module) -> Self {
        Self { module }
    }

    pub fn iter(&self) -> TokenIter<'_> {
        TokenIter {
            lexer: self,
            char_peeker: self.module.iter().enumerate().peekable(),
        }
    }

    fn next_token(&self, char_peeker: &mut Peekable<Enumerate<Chars<'_>>>) -> Option<Token> {
        while char_peeker.peek().is_some_and(|(_, c)| c.is_whitespace()) {
            char_peeker.next();
        }

        let Some((cur_idx, cur_char)) = char_peeker.peek() else {
            return None;
        };

        let cur_idx = *cur_idx;

        let mut call_next = true;

        let token = match cur_char {
            '#' => {
                char_peeker.next();
                let mut size = 1;
                while let Some((_, c)) = char_peeker.next() {
                    size = size + 1;
                    if c == '\n' {
                        break;
                    }
                }
                Token::new(Comment, cur_idx, size)
            }

            '+' => Token::new(Plus, cur_idx, 1),
            '-' => Token::new(Minus, cur_idx, 1),
            '*' => Token::new(Asterisk, cur_idx, 1),
            '/' => Token::new(Slash, cur_idx, 1),
            '^' => Token::new(Caret, cur_idx, 1),

            '=' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    Token::new(DoubleEquals, cur_idx, 2)
                } else {
                    call_next = false;
                    Token::new(Equals, cur_idx, 1)
                }
            }
            '<' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    Token::new(LessThanOrEqual, cur_idx, 2)
                } else {
                    call_next = false;
                    Token::new(LessThan, cur_idx, 1)
                }
            }
            '>' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    Token::new(GreaterThanOrEqual, cur_idx, 2)
                } else {
                    call_next = false;
                    Token::new(GreaterThan, cur_idx, 1)
                }
            }
            '!' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(_, c)| c == &'=') {
                    Token::new(NotEquals, cur_idx, 2)
                } else {
                    call_next = false;
                    Token::new(Exclamation, cur_idx, 1)
                }
            }

            '{' => Token::new(LBrace, cur_idx, 1),
            '}' => Token::new(RBrace, cur_idx, 1),
            '(' => Token::new(LParen, cur_idx, 1),
            ')' => Token::new(RParen, cur_idx, 1),
            ':' => Token::new(Colon, cur_idx, 1),
            ',' => Token::new(Comma, cur_idx, 1),

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut last = cur_idx;
                while char_peeker
                    .peek()
                    .is_some_and(|(_, c)| c.is_ascii_alphanumeric() || c == &'_')
                {
                    let (l, _) = char_peeker.next().unwrap();
                    last = l;
                }
                let chars = self.module.slice(cur_idx, last + 1);
                call_next = false;
                match chars {
                    "true" => Token::new(Boolean, cur_idx, 4),
                    "false" => Token::new(Boolean, cur_idx, 5),
                    _ => Token::new(Ident, cur_idx, chars.len()),
                }
            }
            '0'..='9' => {
                let last = self.read_number(char_peeker, cur_idx);
                let chars = &self.module.slice(cur_idx, last + 1);
                call_next = false;
                Token::new(Number, cur_idx, chars.len())
            }

            '.' => {
                char_peeker.next();
                call_next = false;
                if char_peeker.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
                    let last = self.read_number(char_peeker, cur_idx);
                    let chars = &self.module.slice(cur_idx, last + 1);
                    Token::new(Number, cur_idx, chars.len())
                } else {
                    Token::new(Dot, cur_idx, 1)
                }
            }

            _ => Token::new(Illegal, cur_idx, 1),
        };

        if call_next {
            char_peeker.next();
        }

        Some(token)
    }

    fn read_number(&self, char_peeker: &mut Peekable<Enumerate<Chars<'_>>>, start: usize) -> usize {
        let mut last = start;
        let mut seen_dot = false;

        while let Some((_, c)) = char_peeker.peek() {
            match *c {
                '0'..='9' => {
                    let (l, _) = char_peeker.next().unwrap();
                    last = l;
                }
                '_' => {
                    let (l, _) = char_peeker.next().unwrap();
                    last = l;
                }
                '.' if !seen_dot => {
                    let mut lookahead = char_peeker.clone();
                    lookahead.next();
                    if lookahead.peek().is_some_and(|(_, n)| n.is_ascii_digit()) {
                        let (l, _) = char_peeker.next().unwrap();
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

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_token(&mut self.char_peeker)
    }
}
