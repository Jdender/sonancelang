pub mod from_ast;
pub mod structure;
pub mod symbol_table;

pub use structure::*;
pub use symbol_table::*;

use super::ast;
use from_ast::AstVisitor;

pub fn semantic_pass(input: ast::File) -> File {
    let symbol_table = SymbolTable::new();
    input.visit_ast(&symbol_table)
}
