use crate::common::span::Span;

#[derive(Debug)]
pub struct CheckedAstError {
    pub span: Span,
    pub kind: CheckedAstErrorKind,
}

#[derive(Debug)]
pub enum CheckedAstErrorKind {
    TypeMismatch(String, String),
    VariableNotFound(String),
    TypeNotFound(String),
}

impl CheckedAstError {
    pub fn type_mismatch(expected: &str, got: &str, span: Span) -> CheckedAstError {
        CheckedAstError {
            span,
            kind: CheckedAstErrorKind::TypeMismatch(expected.to_string(), got.to_string()),
        }
    }
    pub fn variable_not_found(name: &str, span: Span) -> CheckedAstError {
        CheckedAstError {
            span,
            kind: CheckedAstErrorKind::VariableNotFound(name.to_string()),
        }
    }
    pub fn type_not_found(ty: &str, span: Span) -> CheckedAstError {
        CheckedAstError {
            span,
            kind: CheckedAstErrorKind::TypeNotFound(ty.to_string()),
        }
    }
}
