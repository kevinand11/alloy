use crate::{checking::scope::TypeId, common::span::Span, parsing::expression::TypeIdent};

#[derive(Debug)]
pub struct CheckedAstError {
    pub span: Span,
    pub kind: CheckedAstErrorKind,
}

#[derive(Debug)]
pub enum CheckedAstErrorKind {
    TypeMismatch(TypeId, TypeId),
    VariableNotFound(String),
    TypeNotFound(TypeIdent),
}

impl CheckedAstError {
    pub fn type_mismatch(expected: TypeId, got: TypeId, span: Span) -> CheckedAstError {
        CheckedAstError {
            span,
            kind: CheckedAstErrorKind::TypeMismatch(expected, got),
        }
    }
    pub fn variable_not_found(name: String, span: Span) -> CheckedAstError {
        CheckedAstError {
            span,
            kind: CheckedAstErrorKind::VariableNotFound(name),
        }
    }
    pub fn type_not_found(ty: TypeIdent, span: Span) -> CheckedAstError {
        CheckedAstError {
            span,
            kind: CheckedAstErrorKind::TypeNotFound(ty),
        }
    }
}
