#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod backend;
pub mod semantic;

pub fn compile(input: &'_ str) -> Result<Vec<u8>, String> {
    let backend = backend::Backend::new()?;

    let ast = ast::ast_pass(input)?;
    let semantic = semantic::semantic_pass(ast);
    let binary = backend.compile_func(semantic)?;

    Ok(binary)
}
