use super::span::Span;

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

    VariableAssignment {
        name: String,
        value: Box<Expression>,
    },

    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },

    MethodCall {
        caller: Box<Expression>,
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Not,
}

#[derive(Clone, Copy, Debug)]
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
pub enum ExpressionState {
    Checked(String),
    Unchecked,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
    pub state: ExpressionState,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self {
            kind,
            span,
            state: ExpressionState::Unchecked,
        }
    }
    pub fn mark_checked(&self, ty: &str) -> Self {
        Self {
            kind: self.kind.clone(),
            span: self.span.clone(),
            state: ExpressionState::Checked(ty.to_string()),
        }
    }
    pub fn ty(&self) -> Option<&str> {
        match &self.state {
            ExpressionState::Checked(ty) => Some(ty),
            ExpressionState::Unchecked => None,
        }
    }

    pub fn with_kind(&self, kind: ExpressionKind) -> Self {
        Self {
            span: self.span.clone(),
            state: self.state.clone(),
            kind,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeIdent(pub String);
