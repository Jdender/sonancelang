#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test;

pub mod generate;
pub mod parse;

use generate::AstVisitor;
use parse::ParseError;

use parity_wasm::elements::Module;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError<'input> {
    ParseError(ParseError<'input>),
}

pub fn compile(input: &str) -> Result<Module, CompilerError> {
    Ok(grammar::FileParser::new()
        .parse(input)
        .map_err(CompilerError::ParseError)?
        .visit_ast(()))
}
