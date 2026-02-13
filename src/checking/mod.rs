use crate::{
    checking::{
        errors::CheckError,
        scope::{ScopeManager, ScopedVar},
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

type Type<'a> = &'a str;

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
        type_hint: Option<&str>,
    ) -> Result<Expression, CheckError> {
        match &expr.kind {
            ExpressionKind::LiteralBool(_) => self.expect(expr, "Bool", type_hint),
            ExpressionKind::LiteralInt(_) => self.expect(expr, "Int", type_hint),
            ExpressionKind::LiteralFloat(_) => self.expect(expr, "Float", type_hint),
            ExpressionKind::Ident(name) => {
                let var = self.scope_manager.lookup_var(&name, self.scope_manager.cur);
                match var {
                    Some(ScopedVar { name: _, type_name }) => {
                        self.expect(expr, &type_name, type_hint)
                    }
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
                        "Bool",
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
                    let (lh, lh_type) = self.expect_with_types(&lh, vec!["Int", "Float"])?;
                    let (rh, rh_type) = self.expect_with_types(&rh, vec!["Int", "Float"])?;
                    let res_type = match op {
                        InfixOp::Divide => "Float",
                        _ => Checker::choose_btw_types(lh_type, rh_type, "Float"),
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
                    let (lh, _) = self.expect_with_types(&lh, vec!["Int", "Float"])?;
                    let (rh, _) = self.expect_with_types(&rh, vec!["Int", "Float"])?;
                    self.expect(
                        &expr.with_kind(ExpressionKind::Infix {
                            op: *op,
                            lh: Box::new(lh),
                            rh: Box::new(rh),
                        }),
                        "Bool",
                        type_hint,
                    )
                }
                InfixOp::Equals | InfixOp::NotEquals => {
                    let lh = self.check_expression(&lh, None)?;
                    let rh = self.check_expression(&rh, None)?;
                    if lh.ty().unwrap() != rh.ty().unwrap() {
                        Err(CheckError::type_mismatch(
                            vec![lh.ty().unwrap()],
                            rh.ty().unwrap(),
                            &expr.span,
                        ))
                    } else {
                        self.expect(expr, "Bool", type_hint)
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
                    Some(expr) => expr.ty().unwrap().to_string(),
                    None => "Unit".to_string(),
                };

                self.scope_manager.cur = original_scope;

                self.expect(
                    &expr.with_kind(ExpressionKind::Block(checked_exprs)),
                    block_type.as_str(),
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
                        .ok_or(CheckError::type_not_found(&ty.0, &expr.span))?;
                    let type_name = &scope_type.name.clone();
                    self.check_expression(&value, Some(type_name.as_str()))?
                } else {
                    self.check_expression(&value, None)?
                };

                self.scope_manager.add_var(name.as_str(),value.ty().unwrap());
                self.expect(
                    &expr.with_kind(ExpressionKind::VariableDecl {
                        name: name.clone(),
                        value: Box::new(value),
                        mutable: *mutable,
                        ty: ty.clone(),
                    }),
                    "Unit",
                    type_hint,
                )
            }
            ExpressionKind::VariableAssignment { name, value } => {
                let value = match self.scope_manager.lookup_var(&name, self.scope_manager.cur) {
                    Some(var) => {
                        let type_name = &var.type_name.clone();
                        self.check_expression(&value, Some(type_name))?
                    }
                    None => {
                        return Err(CheckError::variable_not_found(&name, &expr.span));
                    }
                };

                self.scope_manager.add_var(name.as_str(), value.ty().unwrap());
                self.expect(
                    &expr.with_kind(ExpressionKind::VariableAssignment {
                        name: name.clone(),
                        value: Box::new(value),
                    }),
                    "Unit",
                    type_hint,
                )
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
                    &expr.with_kind(ExpressionKind::FunctionCall { name: name.clone(), args }),
                    "Unit",
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
                    "Unit",
                    type_hint,
                )
            }
        }
    }

    fn expect_with_types<'a>(
        &mut self,
        expr: &Expression,
        type_hints: Vec<&'a str>,
    ) -> Result<(Expression, &'a str), CheckError> {
        for type_hint in &type_hints {
            match self.check_expression(expr, Some(type_hint)) {
                Ok(checked_expr) => return Ok((checked_expr, type_hint)),
                Err(_) => (),
            }
        }
        Err(CheckError::type_mismatch(type_hints, "Unknown", &expr.span))
    }

    fn choose_btw_types<'a>(type1: &'a str, type2: &'a str, or_else: &'a str) -> &'a str {
        if type1 == type2 { type1 } else { or_else }
    }

    fn expect<'a>(
        &self,
        expr: &Expression,
        res_type: &'a str,
        type_hint: Option<&str>,
    ) -> Result<Expression, CheckError> {
        if let Some(type_hint) = type_hint {
            if type_hint == res_type {
                Ok(expr.mark_checked(res_type))
            } else {
                let span = &expr.span;
                Err(CheckError::type_mismatch(vec![type_hint], res_type, span))
            }
        } else {
            Ok(expr.mark_checked(res_type))
        }
    }
}
