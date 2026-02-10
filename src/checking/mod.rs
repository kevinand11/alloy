use crate::{
    checking::{
        errors::CheckedAstError,
        globals::{TYPE_BOOL, TYPE_FLOAT, TYPE_INT},
        scope::{ScopeManager, ScopedType, ScopedVariable, TypeId},
        types::{CheckedAst, CheckedExpression},
    },
    parsing::{
        ast::Ast,
        expression::{Expression, ExpressionKind},
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
    ) -> Result<CheckedExpression, CheckedAstError> {
        match &expr.kind {
            ExpressionKind::LiteralBool(_) => self.expect(
                expr,
                TYPE_BOOL.id,
                type_hint,
            ),
            ExpressionKind::LiteralInt(_) => self.expect(
                expr,
                TYPE_INT.id,
                type_hint,
            ),
            ExpressionKind::LiteralFloat(_) => self.expect(
                expr,
                TYPE_FLOAT.id,
                type_hint,
            ),
            ExpressionKind::Ident(name) => {
                if let Some(&ScopedVariable {
                    id: _,
                    name: _,
                    type_id: var_type,
                }) = self.scope_manager.lookup_var(&name, self.scope_manager.cur)
                {
                    self.expect(
                        expr,
                        var_type,
                        type_hint,
                    )
                } else {
                    Err(CheckedAstError::variable_not_found(
                        name.to_owned(),
                        expr.span,
                    ))
                }
            }
            ExpressionKind::Prefix { op: _, rh: _ } => todo!(),
            ExpressionKind::Infix { op: _, lh: _, rh:  _ } => todo!(),
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
    ) -> Result<CheckedExpression, CheckedAstError> {
        if let Some(type_hint) = type_hint {
            if type_hint.id == res_type {
                Ok(CheckedExpression::new(expr, res_type))
            } else {
                Err(CheckedAstError::type_mismatch(
                    type_hint.id,
                    res_type,
                    expr.span,
                ))
            }
        } else {
            Ok(CheckedExpression::new(expr, res_type))
        }
    }
}
