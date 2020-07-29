#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

pub mod ast;
pub mod backend;

pub fn compile(input: &'_ str) -> Result<Vec<u8>, String> {
    let input = grammar::FileParser::new()
        .parse(input)
        .map_err(|e| e.to_string())?;

    let backend = backend::Backend::new()?;

    let input = backend.compile_func(input)?;

    Ok(input)
}
