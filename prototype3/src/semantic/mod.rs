pub mod common;
pub mod header;
pub mod symbol_table;
pub mod type_check;

pub use {common::*, symbol_table::*};

use super::ast;

pub fn semantic_pass(input: ast::File) -> Result<type_check::File, SemanticError> {
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
        expected: type_check::Ty,
        found: type_check::Ty,
    },
    #[error(
        "Type Mismatch: Let binding declared as {expected:?} but found an initializer of {found:?}"
    )]
    TyMismatchDeclare {
        expected: type_check::Ty,
        found: type_check::Ty,
    },
    #[error("Type Mismatch: Variable declared previously as {expected:?} but trying to assign with {found:?}")]
    TyMismatchAssign {
        expected: type_check::Ty,
        found: type_check::Ty,
    },
    #[error("Type Mismatch: Left {left:?} and right {right:?} can't use {operator:?} together")]
    TyMismatchOperator {
        left: type_check::Ty,
        right: type_check::Ty,
        operator: type_check::InfixOperator,
    },
    #[error("Type Mismatch: If expression has two incompatible results ({when_true:?} and {when_false:?})")]
    TyMismatchIfElse {
        when_true: type_check::Ty,
        when_false: type_check::Ty,
    },
    #[error("Type Mismatch: Parameter declared as {expected:?} but passed argument of {found:?} in position {position}")]
    TyMismatchArg {
        expected: type_check::Ty,
        found: type_check::Ty,
        position: usize,
    },
    #[error("Local {symbol:?} not found in the current scope")]
    LocalNotFound { symbol: type_check::Identifier },
    #[error("Expected symbol {symbol:?} to be a local, it wasn't")]
    ExpectedLocalSymbol { symbol: type_check::Identifier },
    #[error("Function {symbol:?} not found in the current scope")]
    FuncNotFound { symbol: type_check::Identifier },
    #[error("Expected symbol {symbol:?} to be a function, it wasn't")]
    ExpectedFuncSymbol { symbol: type_check::Identifier },
    #[error("Not Enough Arguments (expected {expected}, found {found})")]
    NotEnoughArgs { expected: usize, found: usize },
    #[error("Too Many Arguments (expected {expected}, found {found})")]
    TooManyArgs { expected: usize, found: usize },
}
