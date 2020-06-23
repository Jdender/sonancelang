pub mod codegen;
pub mod parse;

use {
    codegen::{generate, GeneratorError},
    parse::{parse, ParseError},
    parity_wasm::elements::Error as WasmError,
};

#[derive(Debug, Clone)]
pub enum CompileError<'a> {
    SyntaxError(ParseError<'a>),
    WasmError(WasmError),
    GeneratorError(GeneratorError),
}

use CompileError::*;

pub fn compile(input: &str) -> Result<Vec<u8>, CompileError> {
    let parsed = parse(input).map_err(SyntaxError)?;
    let compiled = generate(parsed).map_err(GeneratorError)?;
    compiled.to_bytes().map_err(WasmError)
}
