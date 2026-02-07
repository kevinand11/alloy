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
        while char_peeker.peek().is_some() {
            tokens.push(self.next_token(&mut char_peeker));
        }
        Peeker::new(tokens)
    }

    fn next_token(&self, char_peeker: &mut Peeker<char>) -> Token {
        while char_peeker.peek().is_some_and(|(c, _)| c.is_whitespace()) {
            char_peeker.next();
        }

        let Some((cur_char, cur_idx)) = char_peeker.peek() else {
            return Token::new(Eof, self.module.ln(), 10);
        };

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
                Token::new(Indentifier, cur_idx, chars.len())
            }
            '0'..='9' => {
                let mut last = cur_idx;
                while char_peeker.peek().is_some_and(|(c, _)| c.is_ascii_digit()) {
                    let (_, l) = char_peeker.next().unwrap();
                    last = l;
                }
                let chars = &self.module.slice(cur_idx, last + 1);
                Token::new(LiteralInt, cur_idx, chars.len())
            }

            _ => Token::new(Illegal, cur_idx, 1),
        };

        char_peeker.next();

        token
    }
}
