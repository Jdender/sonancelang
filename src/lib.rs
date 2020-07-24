#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

#[cfg(test)]
mod test;

pub mod ast;
pub mod backend;
pub mod ir;
pub mod semantic;
pub mod wasm;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError<'input> {
    ParseError(ast::ParseError<'input>),
    SemanticError(semantic::SemanticError),
    IrError(ir::IrError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerOutput {
    pub wasm: wasm::Module,
    pub formatted: String,
}

pub fn compile(input: &str) -> Result<CompilerOutput, CompilerError> {
    use CompilerError::*;

    let ast = ast::ast_pass(input).map_err(ParseError)?;
    let formatted = ast.to_string();

    let wasm = wasm::wasm_pass(
        ir::ir_pass(semantic::semantic_pass(ast).map_err(SemanticError)?).map_err(IrError)?,
    );

    Ok(CompilerOutput { wasm, formatted })
}
