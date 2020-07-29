#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

pub mod ast;
pub mod backend;
pub mod semantic;

pub fn compile(input: &'_ str) -> Result<Vec<u8>, String> {
    let ast = grammar::FileParser::new()
        .parse(input)
        .map_err(|e| e.to_string())?;

    let semantic = semantic::semantic_pass(ast);

    let backend = backend::Backend::new()?;

    let input = backend.compile_func(semantic)?;

    Ok(input)
}
