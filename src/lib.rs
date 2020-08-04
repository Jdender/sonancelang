#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod backend;
pub mod semantic;

pub fn compile(input: &'_ str) -> Result<Vec<u8>, CompileError> {
    let backend = backend::Backend::new()?;

    let ast = ast::ast_pass(input).map_err(CompileError::Parse)?;
    let semantic = semantic::semantic_pass(ast)?;
    let binary = backend.compile_func(semantic)?;

    Ok(binary)
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Error while parsing file")]
    Parse(String),
    #[error("Error while checking code")]
    Semantic(#[from] semantic::SemanticError),
    #[error("Error while generating binary code")]
    Backend(#[from] backend::BackendError),
}
