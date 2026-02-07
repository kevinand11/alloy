use crate::span::Span;

#[derive(Debug)]
pub enum ExpressionKind {
    LiteralInt(isize),
    LiteralFloat(f32),
    LiteralBool(bool),
    Ident(String),

    Prefix {
        op: PrefixOp,
        rh: Box<Expression>,
    },

    Infix {
        op: InfixOp,
        lh: Box<Expression>,
        rh: Box<Expression>,
    },

    Block(Vec<Expression>),
}

#[derive(Debug)]
pub enum PrefixOp {
    Not,
    Negate,
}

#[derive(Debug)]
pub enum InfixOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    Equals,
    NotEquals,
}

#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,
    span: Span,
}

impl Expression {
    pub const fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}
