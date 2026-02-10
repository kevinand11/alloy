use crate::parsing::expression::Expression;

#[derive(Debug)]
pub struct CheckedAst(pub Vec<Expression>);
