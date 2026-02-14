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
    Checked(String, usize),
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
    pub fn mark_checked(&self, ty: (&str, usize)) -> Self {
        Self {
            kind: self.kind.clone(),
            span: self.span.clone(),
            state: ExpressionState::Checked(ty.0.to_string(), ty.1),
        }
    }
    pub fn ty(&self) -> (&str, usize) {
        match &self.state {
            ExpressionState::Checked(ty_name, ty_id) => (ty_name.as_str(), *ty_id),
            ExpressionState::Unchecked => ("Unknown", 0),
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
