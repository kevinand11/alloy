use crate::{
    checking::{
        errors::CheckError,
        scope::{
            BOOL_TYPE_ID, FLOAT_TYPE_ID, INT_TYPE_ID, ScopeManager, ScopeTypeId, ScopedVar,
            UNIT_TYPE_ID,
        },
    },
    common::{
        ast::Ast,
        expression::{Expression, ExpressionKind, InfixOp, PrefixOp},
    },
};

pub mod errors;
pub mod scope;

pub struct Checker {
    scope_manager: ScopeManager,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            scope_manager: ScopeManager::new(),
        }
    }

    pub fn check(&mut self, ast: Ast) -> Result<Ast, Vec<CheckError>> {
        let mut checked_exprs = vec![];
        let mut errors = vec![];
        for expr in ast.0 {
            match self.check_expression(&expr, None) {
                Ok(checked_expr) => checked_exprs.push(checked_expr),
                Err(err) => errors.push(err),
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(Ast(checked_exprs))
    }

    fn check_expression(
        &mut self,
        expr: &Expression,
        type_hint: Option<ScopeTypeId>,
    ) -> Result<Expression, CheckError> {
        match &expr.kind {
            ExpressionKind::LiteralBool(_) => self.expect(expr, BOOL_TYPE_ID, type_hint),
            ExpressionKind::LiteralInt(_) => self.expect(expr, INT_TYPE_ID, type_hint),
            ExpressionKind::LiteralFloat(_) => self.expect(expr, FLOAT_TYPE_ID, type_hint),
            ExpressionKind::Ident(name) => {
                let var = self.scope_manager.lookup_var(&name, self.scope_manager.cur);
                match var {
                    Some(ScopedVar {
                        id: _,
                        name: _,
                        type_id,
                        mutable: _,
                    }) => self.expect(expr, *type_id, type_hint),
                    None => {
                        return Err(CheckError::variable_not_found(&name, &expr.span));
                    }
                }
            }
            ExpressionKind::Prefix { op, rh } => match op {
                PrefixOp::Not => {
                    let rh = self.check_expression(&rh, None)?;
                    self.expect(
                        &expr.with_kind(ExpressionKind::Prefix {
                            op: *op,
                            rh: Box::new(rh),
                        }),
                        BOOL_TYPE_ID,
                        type_hint,
                    )
                }
            },
            ExpressionKind::Infix { op, lh, rh } => match op {
                InfixOp::Add
                | InfixOp::Subtract
                | InfixOp::Multiply
                | InfixOp::Divide
                | InfixOp::Power => {
                    let (lh, lh_type) =
                        self.expect_with_types(&lh, vec![INT_TYPE_ID, FLOAT_TYPE_ID])?;
                    let (rh, rh_type) =
                        self.expect_with_types(&rh, vec![INT_TYPE_ID, FLOAT_TYPE_ID])?;
                    let res_type = match op {
                        InfixOp::Divide => FLOAT_TYPE_ID,
                        _ => Checker::choose_btw_types(lh_type, rh_type, FLOAT_TYPE_ID),
                    };
                    self.expect(
                        &expr.with_kind(ExpressionKind::Infix {
                            op: *op,
                            lh: Box::new(lh),
                            rh: Box::new(rh),
                        }),
                        res_type,
                        type_hint,
                    )
                }
                InfixOp::LessThan
                | InfixOp::GreaterThan
                | InfixOp::LessThanOrEqual
                | InfixOp::GreaterThanOrEqual => {
                    let (lh, _) = self.expect_with_types(&lh, vec![INT_TYPE_ID, FLOAT_TYPE_ID])?;
                    let (rh, _) = self.expect_with_types(&rh, vec![INT_TYPE_ID, FLOAT_TYPE_ID])?;
                    self.expect(
                        &expr.with_kind(ExpressionKind::Infix {
                            op: *op,
                            lh: Box::new(lh),
                            rh: Box::new(rh),
                        }),
                        BOOL_TYPE_ID,
                        type_hint,
                    )
                }
                InfixOp::Equals | InfixOp::NotEquals => {
                    let lh = self.check_expression(&lh, None)?;
                    let rh = self.check_expression(&rh, None)?;
                    if lh.ty().1 != rh.ty().1 {
                        Err(CheckError::type_mismatch(
                            vec![lh.ty().1],
                            rh.ty().1,
                            &expr.span,
                        ))
                    } else {
                        self.expect(expr, BOOL_TYPE_ID, type_hint)
                    }
                }
            },
            ExpressionKind::Block(exprs) => {
                let new_scope = self.scope_manager.create_scope(self.scope_manager.cur);
                let original_scope = self.scope_manager.cur;
                self.scope_manager.cur = new_scope;
                let len = exprs.len();
                let mut checked_exprs = vec![];

                for (index, expr) in exprs.iter().enumerate() {
                    let expr = self.check_expression(
                        expr,
                        match index == len - 1 {
                            true => type_hint,
                            false => None,
                        },
                    )?;
                    checked_exprs.push(expr);
                }

                let block_type = match checked_exprs.last() {
                    Some(expr) => expr.ty().1,
                    None => UNIT_TYPE_ID,
                };

                self.scope_manager.cur = original_scope;

                self.expect(
                    &expr.with_kind(ExpressionKind::Block(checked_exprs)),
                    block_type,
                    type_hint,
                )
            }
            ExpressionKind::VariableDecl {
                name,
                value,
                mutable,
                ty,
            } => {
                let value = if let Some(ty) = ty {
                    let scope_type = self
                        .scope_manager
                        .lookup_type(&ty.0, self.scope_manager.cur)
                        .ok_or(CheckError::type_name_not_found(&ty.0, &expr.span))?;
                    self.check_expression(&value, Some(scope_type.id))?
                } else {
                    self.check_expression(&value, None)?
                };

                self.scope_manager
                    .add_var(name.as_str(), value.ty().1, *mutable);
                self.expect(
                    &expr.with_kind(ExpressionKind::VariableDecl {
                        name: name.clone(),
                        value: Box::new(value),
                        mutable: *mutable,
                        ty: ty.clone(),
                    }),
                    UNIT_TYPE_ID,
                    type_hint,
                )
            }
            ExpressionKind::VariableAssignment { name, value } => {
                let value = match self.scope_manager.lookup_var(&name, self.scope_manager.cur) {
                    Some(var) => {
                        if !var.mutable {
                            return Err(CheckError::assign_to_const_variable(&name, &expr.span));
                        }
                        self.check_expression(&value, Some(var.type_id))?
                    }
                    None => {
                        return Err(CheckError::variable_not_found(&name, &expr.span));
                    }
                };
                self.expect(
                    &expr.with_kind(ExpressionKind::VariableAssignment {
                        name: name.clone(),
                        value: Box::new(value),
                    }),
                    UNIT_TYPE_ID,
                    type_hint,
                )
            }
            ExpressionKind::TypeDecl { name, value } => {
                let parent_type = self
                    .scope_manager
                    .lookup_type(&value.0, self.scope_manager.cur)
                    .ok_or(CheckError::type_name_not_found(&value.0, &expr.span))?;
                self.scope_manager.add_type(name.as_str(), parent_type.id);
                self.expect(expr, UNIT_TYPE_ID, type_hint)
            }
            ExpressionKind::FunctionCall { name, args } => {
                // Hardcoded for now, will implement proper function definitions and lookups later
                // also verify argument types and length matches fn type and length
                if name != "to_unit" {
                    return Err(CheckError::function_not_found(&name, &expr.span));
                }
                let args = args
                    .into_iter()
                    .map(|arg| self.check_expression(&arg, None))
                    .collect::<Result<Vec<_>, _>>()?;
                self.expect(
                    &expr.with_kind(ExpressionKind::FunctionCall {
                        name: name.clone(),
                        args,
                    }),
                    UNIT_TYPE_ID,
                    type_hint,
                )
            }
            ExpressionKind::MethodCall { name, args, caller } => {
                // Hardcoded for now, will implement proper function definitions and lookups later
                // also verify argument types and length matches method type and length and caller type
                if name != "to_unit" {
                    return Err(CheckError::method_not_found(&name, &expr.span));
                }
                let caller = self.check_expression(&caller, None)?;
                let args = args
                    .into_iter()
                    .map(|arg| self.check_expression(&arg, None))
                    .collect::<Result<Vec<_>, _>>()?;
                self.expect(
                    &expr.with_kind(ExpressionKind::MethodCall {
                        name: name.clone(),
                        args,
                        caller: Box::new(caller),
                    }),
                    UNIT_TYPE_ID,
                    type_hint,
                )
            }
        }
    }

    fn expect_with_types(
        &mut self,
        expr: &Expression,
        type_hints: Vec<ScopeTypeId>,
    ) -> Result<(Expression, ScopeTypeId), CheckError> {
        let expr_type = expr.ty();
        for type_hint in &type_hints {
            match self.check_expression(expr, Some(*type_hint)) {
                Ok(checked_expr) => return Ok((checked_expr, *type_hint)),
                Err(_) => (),
            }
        }
        Err(CheckError::type_mismatch(
            type_hints,
            expr_type.1,
            &expr.span,
        ))
    }

    fn choose_btw_types(
        type1: ScopeTypeId,
        type2: ScopeTypeId,
        or_else: ScopeTypeId,
    ) -> ScopeTypeId {
        if type1 == type2 { type1 } else { or_else }
    }

    fn expect<'a>(
        &self,
        expr: &Expression,
        exp_type: ScopeTypeId,
        type_hint: Option<ScopeTypeId>,
    ) -> Result<Expression, CheckError> {
        let scope_type = self
            .scope_manager
            .get_type(exp_type, self.scope_manager.cur)
            .ok_or(CheckError::type_not_found(exp_type, &expr.span))?;

        if let Some(type_hint) = type_hint {
            if type_hint == exp_type {
                Ok(expr.mark_checked((&scope_type.name, scope_type.id)))
            } else {
                let span = &expr.span;
                Err(CheckError::type_mismatch(vec![type_hint], exp_type, span))
            }
        } else {
            Ok(expr.mark_checked((&scope_type.name, scope_type.id)))
        }
    }
}
