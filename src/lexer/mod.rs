use token::{Token, TokenKind::*};

use crate::{common::peeker::Peeker, lexer::module::Module};

pub mod module;
pub mod token;

#[derive(Clone)]
pub struct Lexer {
    pub module: Module,
}

impl Lexer {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn get_peeker(&self) -> Peeker<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut char_peeker = self.module.get_peeker();
        while let Some(token) = self.next_token(&mut char_peeker) {
            tokens.push(token);
        }
        Peeker::new(tokens)
    }

    fn next_token(&self, char_peeker: &mut Peeker<char>) -> Option<Token> {
        while char_peeker.peek().is_some_and(|(c, _)| c.is_whitespace()) {
            char_peeker.next();
        }

        let Some((cur_char, cur_idx)) = char_peeker.peek() else {
            return None;
        };

        let mut call_next = true;

        let token = match cur_char {
            '#' => {
                char_peeker.next();
                let mut size = 1;
                while let Some((c, _)) = char_peeker.next() {
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
                if char_peeker.peek().is_some_and(|(c, _)| c == &'=') {
                    Token::new(DoubleEquals, cur_idx, 2)
                } else {
                    Token::new(Illegal, cur_idx, 1)
                }
            }
            '<' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(c, _)| c == &'=') {
                    Token::new(LessThanOrEqual, cur_idx, 2)
                } else {
                    Token::new(LessThan, cur_idx, 1)
                }
            }
            '>' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(c, _)| c == &'=') {
                    Token::new(GreaterThanOrEqual, cur_idx, 2)
                } else {
                    Token::new(GreaterThan, cur_idx, 1)
                }
            }
            '!' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(c, _)| c == &'=') {
                    Token::new(NotEquals, cur_idx, 2)
                } else {
                    Token::new(Exclamation, cur_idx, 1)
                }
            }

            '{' => Token::new(LBrace, cur_idx, 1),
            '}' => Token::new(RBrace, cur_idx, 1),

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut last = cur_idx;
                while char_peeker
                    .peek()
                    .is_some_and(|(c, _)| c.is_ascii_alphanumeric() || c == &'_')
                {
                    let (_, l) = char_peeker.next().unwrap();
                    last = l;
                }
                let chars = &self.module.slice(cur_idx, last + 1);
                call_next = false;
                Token::new(Indentifier, cur_idx, chars.len())
            }
            '0'..='9' => {
                let last = self.read_number(char_peeker, cur_idx);
                let chars = &self.module.slice(cur_idx, last + 1);
                call_next = false;
                Token::new(Number, cur_idx, chars.len())
            }

            '.' => {
                char_peeker.next();
                if char_peeker.peek().is_some_and(|(c, _)| c.is_ascii_digit()) {
                    let last = self.read_number(char_peeker, cur_idx);
                    let chars = &self.module.slice(cur_idx, last + 1);
                    call_next = false;
                    Token::new(Number, cur_idx, chars.len())
                } else {
                    Token::new(Illegal, cur_idx, 1)
                }
            }

            _ => Token::new(Illegal, cur_idx, 1),
        };

        if call_next {
            char_peeker.next();
        }

        Some(token)
    }

    fn read_number(&self, char_peeker: &mut Peeker<char>, start: usize) -> usize {
        let mut last = start;
        let mut seen_dot = false;

        while let Some((c, _)) = char_peeker.peek() {
            match *c {
                '0'..='9' => {
                    let (_, l) = char_peeker.next().unwrap();
                    last = l;
                }
                '_' => {
                    let (_, l) = char_peeker.next().unwrap();
                    last = l;
                }
                '.' if !seen_dot => {
                    let (_, l) = char_peeker.next().unwrap();
                    last = l;
                    seen_dot = true;
                }
                _ => break,
            }
        }

        last
    }
}
