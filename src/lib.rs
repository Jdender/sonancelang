#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test;

pub mod ast;
pub mod ir;
pub mod semantic;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError<'input> {
    ParseError(ast::ParseError<'input>),
    SemanticError(semantic::SemanticError),
    IrError(ir::IrError),
}

pub fn compile(input: &str) -> Result<ir::WasmModule, CompilerError> {
    use CompilerError::*;
    ir::ir_pass(
        semantic::semantic_pass(ast::ast_pass(input).map_err(ParseError)?)
            .map_err(SemanticError)?,
    )
    .map_err(IrError)
}
