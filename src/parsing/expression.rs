use crate::common::span::Span;

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
    pub ty: String,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span, ty: "Unit".to_string() }
    }
    pub fn with_type(&self, ty: &str) -> Self {
        Self {
            kind: self.kind.clone(),
            span: self.span.clone(),
            ty: ty.to_string(),
        }
    }
    pub fn with_kind(&self, kind: ExpressionKind) -> Self {
        Self {
            kind,
            span: self.span.clone(),
            ty: self.ty.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeIdent(pub String);
