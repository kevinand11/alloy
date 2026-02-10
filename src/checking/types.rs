use crate::{checking::scope::TypeId, parsing::expression::Expression};

pub struct CheckedAst(pub Vec<CheckedExpression>);

pub struct CheckedExpression {
    pub type_id: TypeId,
    pub data: Expression,
}

impl CheckedExpression {
    pub fn new(data: Expression, type_id: TypeId) -> Self {
        Self { type_id, data }
    }
}
