use crate::{
    checking::{
        errors::CheckedAstError,
        scope::{ScopeManager, ScopedVar},
    },
    parsing::{
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

    pub fn check(&mut self, ast: Ast) -> Result<Ast, Vec<CheckedAstError>> {
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
    ) -> Result<Expression, CheckedAstError> {
        match expr.kind.clone() {
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
                        return Err(CheckedAstError::variable_not_found(
                            &name,
                            &expr.span,
                        ));
                    }
                }
            }
            ExpressionKind::Prefix { op, rh } => match op {
                PrefixOp::Not => {
                    let rh = self.check_expression(&rh, None)?;
                    self.expect(
                        &expr.with_kind(ExpressionKind::Prefix {
                            op,
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
                    let (lh, lh_type) =
                        self.check_expression_with_type_hints(&lh, vec!["Int", "Float"])?;
                    let (rh, rh_type) =
                        self.check_expression_with_type_hints(&rh, vec!["Int", "Float"])?;
                    let res_type = match op {
                        InfixOp::Divide => "Float",
                        _ => Checker::choose_btw_types(lh_type, rh_type, "Float"),
                    };
                    self.expect(
                        &expr.with_kind(ExpressionKind::Infix {
                            op,
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
                    let (lh, _) =
                        self.check_expression_with_type_hints(&*lh, vec!["Int", "Float"])?;
                    let (rh, _) =
                        self.check_expression_with_type_hints(&*rh, vec!["Int", "Float"])?;
                    self.expect(
                        &expr.with_kind(ExpressionKind::Infix {
                            op,
                            lh: Box::new(lh),
                            rh: Box::new(rh),
                        }),
                        "Bool",
                        type_hint,
                    )
                }
                InfixOp::Equals | InfixOp::NotEquals => {
                    let lh = self.check_expression(&*lh, None)?;
                    let rh = self.check_expression(&*rh, None)?;
                    if lh.ty != rh.ty {
                        Err(CheckedAstError::type_mismatch(
                            &lh.ty,
                            &rh.ty,
                            &expr.span,
                        ))
                    } else {
                        self.expect(
                            &expr.with_kind(ExpressionKind::Infix {
                                op,
                                lh: Box::new(lh),
                                rh: Box::new(rh),
                            }),
                            "Bool",
                            type_hint,
                        )
                    }
                }
            },
            ExpressionKind::Block(exprs) => {
                let new_scope = self.scope_manager.create_scope(self.scope_manager.cur);
                let original_scope = self.scope_manager.cur;
                self.scope_manager.cur = new_scope;

                let mut checked_exprs = vec![];
                for (index, expr) in exprs.iter().enumerate() {
                    let checked_expr = self.check_expression(
                        expr,
                        match index == exprs.len() - 1 {
                            true => type_hint,
                            false => None,
                        },
                    )?;
                    checked_exprs.push(checked_expr);
                }

                let block_type = match checked_exprs.last() {
                    Some(expr) => &expr.ty.clone(),
                    None => "Unit",
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
                let value = if let Some(ty) = &ty {
                    match self
                        .scope_manager
                        .lookup_type(&ty.0, self.scope_manager.cur)
                    {
                        Some(ty) => {
                            let ty_name = ty.name.clone();
                            self.check_expression(&*value, Some(&ty_name))?
                        }
                        None => {
                            return Err(CheckedAstError::type_not_found(&ty.0, &expr.span));
                        }
                    }
                } else {
                    self.check_expression(&*value, None)?
                };

                self.scope_manager.add_var(name.as_str(), &value.ty);
                self.expect(
                    &expr.with_kind(ExpressionKind::VariableDecl {
                        name,
                        value: Box::new(value),
                        mutable,
                        ty,
                    }),
                    "Unit",
                    type_hint,
                )
            }
            ExpressionKind::FunctionCall { name, args } => {
                // Hardcoded for now, will implement proper function definitions and lookups later
                // also verify argument types and length matches fn type and length
                if name != "to_unit" {
                    return Err(CheckedAstError::function_not_found(
                        &name,
                        &expr.span,
                    ));
                }
                let args = args
                    .into_iter()
                    .map(|arg| self.check_expression(&arg, None))
                    .collect::<Result<Vec<_>, _>>()?;
                self.expect(
                    &expr.with_kind(ExpressionKind::FunctionCall { name, args }),
                    "Unit",
                    type_hint,
                )
            }
            ExpressionKind::MethodCall { name, args, caller } => {
                // Hardcoded for now, will implement proper function definitions and lookups later
                // also verify argument types and length matches method type and length and caller type
                if name != "to_unit" {
                    return Err(CheckedAstError::method_not_found(
                        &name,
                        &expr.span,
                    ));
                }
                let caller = self.check_expression(&*caller, None)?;
                let args = args
                    .into_iter()
                    .map(|arg| self.check_expression(&arg, None))
                    .collect::<Result<Vec<_>, _>>()?;
                self.expect(
                    &expr.with_kind(ExpressionKind::MethodCall { name, args, caller: Box::new(caller) }),
                    "Unit",
                    type_hint,
                )
            }
        }
    }

    fn check_expression_with_type_hints<'a>(
        &mut self,
        expr: &Expression,
        type_hints: Vec<&'a str>,
    ) -> Result<(Expression, &'a str), CheckedAstError> {
        let mut last_type_hint = None;
        for type_hint in type_hints {
            match self.check_expression(expr, Some(type_hint)) {
                Ok(checked_expr) => return Ok((checked_expr, type_hint)),
                Err(_) => last_type_hint = Some(type_hint),
            }
        }
        Err(CheckedAstError::type_mismatch(
            last_type_hint.unwrap_or("Unknown"),
            "Unknown",
            &expr.span,
        ))
    }

    fn choose_btw_types<'a>(type1: &'a str, type2: &'a str, or_else: &'a str) -> &'a str {
        if type1 == type2 { type1 } else { or_else }
    }

    fn expect(
        &self,
        expr: &Expression,
        res_type: &str,
        type_hint: Option<&str>,
    ) -> Result<Expression, CheckedAstError> {
        if let Some(type_hint) = type_hint {
            if type_hint == res_type {
                Ok(expr.with_type(res_type))
            } else {
                Err(CheckedAstError::type_mismatch(
                    type_hint,
                    res_type,
                    &expr.span,
                ))
            }
        } else {
            Ok(expr.with_type(res_type))
        }
    }
}
