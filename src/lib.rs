#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

pub mod ast;
pub mod jit;

pub fn compile(input: &'_ str) -> Result<Vec<u8>, String> {
    let input = grammar::FileParser::new()
        .parse(input)
        .map_err(|e| e.to_string())?;

    let jit = jit::JIT::new()?;

    let input = jit.compile_func(input)?;

    Ok(input)
}
