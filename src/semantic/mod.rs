pub mod common;
pub mod from_ast;
pub mod from_header;
pub mod symbol_table;

pub use {common::*, symbol_table::*};

use {super::ast, from_ast::AstVisitor, from_header::HeaderVisitor};

pub fn semantic_pass(input: ast::File) -> Result<from_header::File, SemanticError> {
    let mut symbol_table = SymbolTable::new();
    input
        .visit_ast(&mut symbol_table)?
        .visit_header(&mut symbol_table)
}

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SemanticError {
    #[error(
        "Type Mismatch: Function return declared as {expected:?} but found an return of {found:?}"
    )]
    TyMismatchReturn {
        expected: from_header::Ty,
        found: from_header::Ty,
    },
    #[error(
        "Type Mismatch: Let binding declared as {expected:?} but found an initializer of {found:?}"
    )]
    TyMismatchDeclare {
        expected: from_header::Ty,
        found: from_header::Ty,
    },
    #[error("Type Mismatch: Variable declared previously as {expected:?} but trying to assign with {found:?}")]
    TyMismatchAssign {
        expected: from_header::Ty,
        found: from_header::Ty,
    },
    #[error("Type Mismatch: Left {left:?} and right {right:?} can't use {operator:?} together")]
    TyMismatchOperator {
        left: from_header::Ty,
        right: from_header::Ty,
        operator: from_header::InfixOperator,
    },
    #[error("Type Mismatch: If expression has two incompatible results ({when_true:?} and {when_false:?})")]
    TyMismatchIfElse {
        when_true: from_header::Ty,
        when_false: from_header::Ty,
    },
    #[error("Type Mismatch: Parameter declared as {expected:?} but passed argument of {found:?} in position {position}")]
    TyMismatchArg {
        expected: from_header::Ty,
        found: from_header::Ty,
        position: usize,
    },
    #[error("Local {symbol:?} not found in the current scope")]
    LocalNotFound { symbol: from_header::Identifier },
    #[error("Expected symbol {symbol:?} to be a local, it wasn't")]
    ExpectedLocalSymbol { symbol: from_header::Identifier },
    #[error("Function {symbol:?} not found in the current scope")]
    FuncNotFound { symbol: from_header::Identifier },
    #[error("Expected symbol {symbol:?} to be a function, it wasn't")]
    ExpectedFuncSymbol { symbol: from_header::Identifier },
    #[error("Not Enough Arguments (expected {expected}, found {found})")]
    NotEnoughArgs { expected: usize, found: usize },
    #[error("Too Many Arguments (expected {expected}, found {found})")]
    TooManyArgs { expected: usize, found: usize },
}
