use crate::common::span::Span;

#[derive(Debug)]
pub struct CheckError {
    pub span: Span,
    pub kind: CheckErrorKind,
}

#[derive(Debug)]
pub enum CheckErrorKind {
    TypeMismatch(Vec<String>, String),
    VariableNotFound(String),
    TypeNotFound(String),
    FunctionNotFound(String),
    MethodNotFound(String),
}

impl CheckError {
    pub fn type_mismatch(expected: Vec<&str>, got: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::TypeMismatch(expected.iter().map(|s| s.to_string()).collect(), got.to_string()),
        }
    }
    pub fn variable_not_found(name: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::VariableNotFound(name.to_string()),
        }
    }
    pub fn type_not_found(ty: &str, span: &Span) -> CheckError {
        CheckError {
            span: span.clone(),
            kind: CheckErrorKind::TypeNotFound(ty.to_string()),
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
