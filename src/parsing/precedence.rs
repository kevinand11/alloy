use crate::lexing::token::TokenKind::{self, *};

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equality,   // == or !=
    Comparison, // <, <=, >, >=
    Sum,        // + or -
    Product,    // * or /
    Order,      // ^
    Group,      // { }
    Prefix,     // !X or -X
}

impl Precedence {
    pub fn of(kind: &TokenKind) -> Self {
        match kind {
            DoubleEquals | NotEquals => Precedence::Equality,
            LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual => Precedence::Comparison,
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            Caret => Precedence::Order,
            LBrace => Precedence::Group,
            _ => Precedence::Lowest,
        }
    }
}
