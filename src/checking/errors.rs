use crate::{checking::scope::ScopeTypeId, common::span::Span};

#[derive(Debug)]
pub struct CheckError {
    pub span: Span,
    pub kind: CheckErrorKind,
}

#[derive(Debug)]
pub enum CheckErrorKind {
    TypeMismatch(Vec<ScopeTypeId>, ScopeTypeId),
    VariableNotFound(String),
    AssignToConstVariable(String),
    TypeNameNotFound(String),
    TypeNotFound(ScopeTypeId),
    FunctionNotFound(String),
    MethodNotFound(String),
}

impl CheckError {
    pub fn type_mismatch(expected: Vec<ScopeTypeId>, got: ScopeTypeId, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::TypeMismatch(expected, got),
        }
    }
    pub fn variable_not_found(name: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::VariableNotFound(name.to_string()),
        }
    }
    pub fn assign_to_const_variable(name: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::AssignToConstVariable(name.to_string()),
        }
    }
    pub fn type_name_not_found(name: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::TypeNameNotFound(name.to_string()),
        }
    }
    pub fn type_not_found(ty: ScopeTypeId, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::TypeNotFound(ty),
        }
    }
    pub fn function_not_found(name: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::FunctionNotFound(name.to_string()),
        }
    }
    pub fn method_not_found(name: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::MethodNotFound(name.to_string()),
        }
    }
}
