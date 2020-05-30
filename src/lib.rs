#[macro_use]
extern crate lalrpop_util;

mod codegen;
mod parser;

use codegen::{generate, GeneratorError};
use parser::{parse, ParseError};

#[derive(Debug, Clone)]
pub enum CompileError<'a> {
    SyntaxError(ParseError<'a>),
    WasmError,
    GeneratorError(GeneratorError),
}

use CompileError::*;

pub fn compile(input: &str) -> Result<Vec<u8>, CompileError> {
    let parsed = parse(input).map_err(SyntaxError)?;
    let compiled = generate(parsed).map_err(GeneratorError)?;
    compiled.to_bytes().map_err(|_| WasmError)
}
