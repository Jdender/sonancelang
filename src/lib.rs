#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test;

pub mod generate;
pub mod lowlevel;
pub mod parse;

use generate::AstVisitor;
use lowlevel::LowLevelVisitor;
use parse::ParseError;

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

pub fn compile(input: &str) -> Result<CompilerOutput, CompilerError> {
    let parsed = grammar::FileParser::new()
        .parse(input)
        .map_err(CompilerError::ParseError)?;

    Ok(CompilerOutput {
        wasm: parsed.visit_ast(()).visit_lowlevel(()),
        formatted: parsed.to_string(),
    })
}
