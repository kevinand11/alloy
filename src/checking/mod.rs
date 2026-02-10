use crate::{
    checking::{
        errors::CheckedAstError,
        globals::{TYPE_BOOL, TYPE_FLOAT, TYPE_INT},
        scope::{ScopeManager, ScopedType, ScopedVar, TypeId},
        types::CheckedAst,
    },
    parsing::{
        ast::Ast,
        expression::{Expression, ExpressionKind, InfixOp, PrefixOp},
    },
};

pub mod errors;
pub mod globals;
pub mod scope;
pub mod types;

pub struct Checker<'a> {
    scope_manager: ScopeManager<'a>,
}

impl<'a> Checker<'a> {
    pub fn new() -> Self {
        Self {
            scope_manager: ScopeManager::new(),
        }
    }

    pub fn check(&mut self, ast: Ast) -> Result<CheckedAst, Vec<CheckedAstError>> {
        let mut checked_exprs = vec![];
        let mut errors = vec![];
        for expr in ast.0 {
            match self.check_expression(expr, None) {
                Ok(checked_expr) => checked_exprs.push(checked_expr),
                Err(err) => errors.push(err),
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(CheckedAst(checked_exprs))
    }

    fn check_expression(
        &mut self,
        expr: Expression,
        type_hint: Option<&ScopedType>,
    ) -> Result<Expression, CheckedAstError> {
        match expr.kind.clone() {
            ExpressionKind::LiteralBool(_) => self.expect(expr, TYPE_BOOL.id, type_hint),
            ExpressionKind::LiteralInt(_) => self.expect(expr, TYPE_INT.id, type_hint),
            ExpressionKind::LiteralFloat(_) => self.expect(expr, TYPE_FLOAT.id, type_hint),
            ExpressionKind::Ident(name) => {
                match self.scope_manager.lookup_var(&name, self.scope_manager.cur) {
                    Some(&ScopedVar {
                        id: _,
                        name: _,
                        type_id,
                    }) => self.expect(expr, type_id, type_hint),
                    None => {
                        return Err(CheckedAstError::variable_not_found(
                            name.to_owned(),
                            expr.span,
                        ));
                    }
                }
            }
            ExpressionKind::Prefix { op, rh } => match op {
                PrefixOp::Not => {
                    let rh = self.check_expression(*rh, Some(&TYPE_BOOL))?;
                    self.expect(rh, TYPE_BOOL.id, type_hint)
                }
            },
            ExpressionKind::Infix { op, lh, rh } => match op {
                InfixOp::Add | InfixOp::Subtract | InfixOp::Multiply | InfixOp::Divide | InfixOp::Power => {
                    let lh = self.check_expression(*lh, Some(&TYPE_INT))?;
                    let rh = self.check_expression(*rh, Some(&TYPE_INT))?;
                    self.expect(expr.with_kind(ExpressionKind::Infix { op, lh: Box::new(lh), rh: Box::new(rh) }), TYPE_INT.id, type_hint)
                }
                InfixOp::LessThan | InfixOp::GreaterThan | InfixOp::LessThanOrEqual | InfixOp::GreaterThanOrEqual => {
                    let lh = self.check_expression(*lh, Some(&TYPE_INT))?;
                    let rh = self.check_expression(*rh, Some(&TYPE_INT))?;
                    self.expect(
                        expr.with_kind(ExpressionKind::Infix { op, lh: Box::new(lh), rh: Box::new(rh) }),
                        TYPE_BOOL.id,
                        type_hint,
                    )
                }
                InfixOp::Equals | InfixOp::NotEquals => {
                    let lh = self.check_expression(*lh, None)?;
                    let rh = self.check_expression(*rh, None)?;
                    if lh.ty != rh.ty {
                        Err(CheckedAstError::type_mismatch(lh.ty, rh.ty, expr.span))
                    } else {
                        self.expect(expr.with_kind(ExpressionKind::Infix { op, lh: Box::new(lh), rh: Box::new(rh) }), TYPE_BOOL.id, type_hint)
                    }
                }
            },
            ExpressionKind::Block(_) => todo!(),
            ExpressionKind::VariableDecl {
                name: _,
                value: _,
                mutable: _,
                ty: _,
            } => todo!(),
        }
    }

    fn expect(
        &self,
        expr: Expression,
        res_type: TypeId,
        type_hint: Option<&ScopedType>,
    ) -> Result<Expression, CheckedAstError> {
        if let Some(type_hint) = type_hint {
            if type_hint.id == res_type {
                Ok(expr.with_type(res_type))
            } else {
                Err(CheckedAstError::type_mismatch(
                    type_hint.id,
                    res_type,
                    expr.span,
                ))
            }
        } else {
            Ok(expr.with_type(res_type))
        }
    }
}
