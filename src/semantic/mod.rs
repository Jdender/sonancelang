pub mod from_parse;
pub mod structure;
pub mod symbol_table;

pub use from_parse::semantic_pass;
pub use structure::*;
pub use symbol_table::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    VariableNotDeclared(String),
}

pub type SemResult<T> = Result<T, SemanticError>;
