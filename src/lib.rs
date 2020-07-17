#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test;

pub mod ast;
pub mod ir;
pub mod semantic;

use ast::ParseError;
use parity_wasm::elements::Module;

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerOutput {
    pub wasm: Module,
    pub formatted: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError<'input> {
    ParseError(ParseError<'input>),
}

pub fn compile(_input: &str) -> Result<CompilerOutput, CompilerError> {
    unimplemented!()
}
