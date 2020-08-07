pub mod from_ast;
pub mod structure;
pub mod symbol_table;

pub use {structure::*, symbol_table::*};

use {super::ast, from_ast::AstVisitor};

pub fn semantic_pass(input: ast::File) -> Result<File, SemanticError> {
    let mut symbol_table = SymbolTable::new();
    input.visit_ast(&mut symbol_table)
}

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SemanticError {
    #[error(
        "Type Mismatch: Function return declared as {expected:?} but found an return of {found:?}"
    )]
    TyMismatchReturn { expected: Ty, found: Ty },
    #[error(
        "Type Mismatch: Let binding declared as {expected:?} but found an initializer of {found:?}"
    )]
    TyMismatchDeclare { expected: Ty, found: Ty },
    #[error("Type Mismatch: Variable declared previously as {expected:?} but trying to assign with {found:?}")]
    TyMismatchAssign { expected: Ty, found: Ty },
    #[error("Type Mismatch: Left {left:?} and right {right:?} can't use {operator:?} together")]
    TyMismatchOperator {
        left: Ty,
        right: Ty,
        operator: InfixOperator,
    },
    #[error("Type Mismatch: If expression has two incompatible results ({when_true:?} and {when_false:?})")]
    TyMismatchIfElse { when_true: Ty, when_false: Ty },
    #[error("Type Mismatch: Parameter declared as {expected:?} but passed argument of {found:?} in position {position}")]
    TyMismatchArg {
        expected: Ty,
        found: Ty,
        position: usize,
    },
    #[error("Local {symbol:?} not found in the current scope")]
    LocalNotFound { symbol: Identifier },
    #[error("Expected symbol {symbol:?} to be a local, it wasn't")]
    ExpectedLocalSymbol { symbol: Identifier },
    #[error("Function {symbol:?} not found in the current scope")]
    FuncNotFound { symbol: Identifier },
    #[error("Expected symbol {symbol:?} to be a function, it wasn't")]
    ExpectedFuncSymbol { symbol: Identifier },
    #[error("Not Enough Arguments (expected {expected}, found {found})")]
    NotEnoughArgs { expected: usize, found: usize },
    #[error("Too Many Arguments (expected {expected}, found {found})")]
    TooManyArgs { expected: usize, found: usize },
}
