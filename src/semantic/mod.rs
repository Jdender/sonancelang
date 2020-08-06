pub mod from_ast;
pub mod structure;
pub mod symbol_table;

pub use structure::*;
pub use symbol_table::*;

use super::ast;
use from_ast::AstVisitor;

pub fn semantic_pass(input: ast::File) -> Result<File, SemanticError> {
    let symbol_table = SymbolTable::new();
    input.visit_ast(&symbol_table)
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
    #[error("Symbol {symbol:?} not found in the current scope")]
    SymbolNotFound { symbol: Identifier },
    #[error("Expected symbol {symbol:?} to be local, it wasn't")]
    ExpectedLocalSymbol { symbol: Identifier },
}
