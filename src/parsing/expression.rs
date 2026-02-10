use crate::{checking::scope::TypeId, common::span::Span};

#[derive(Clone, Debug)]
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

    VariableDecl {
        name: String,
        value: Box<Expression>,
        mutable: bool,
        ty: Option<TypeIdent>,
    },
}

#[derive(Clone, Debug)]
pub enum PrefixOp {
    Not,
}

#[derive(Clone, Debug)]
pub enum InfixOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
    pub ty: TypeId,
}

impl Expression {
    pub const fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span, ty: 0 }
    }
    pub fn with_type(self, ty: TypeId) -> Self {
        Self {
            kind: self.kind,
            span: self.span,
            ty,
        }
    }
    pub fn with_kind(self, kind: ExpressionKind) -> Self {
        Self {
            kind,
            span: self.span,
            ty: self.ty,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeIdent(pub String);
