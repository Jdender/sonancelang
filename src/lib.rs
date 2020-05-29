#[macro_use]
extern crate lalrpop_util;

mod codegen;
mod parser;
mod visitor;

use codegen::{generate, GeneratorError};
use parser::parse;

#[derive(Debug, Clone)]
pub enum CompileError {
    SyntaxError,
    WasmError,
    GeneratorError(GeneratorError),
}

use CompileError::*;

pub fn compile(input: &str) -> Result<Vec<u8>, CompileError> {
    let parsed = parse(input).map_err(|_| SyntaxError)?;
    let compiled = generate(*parsed).map_err(|err| GeneratorError(err))?;
    compiled.to_bytes().map_err(|_| WasmError)
}
