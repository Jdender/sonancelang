pub mod structure;

lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/ast/grammar.rs");

pub use structure::*;

pub fn ast_pass(input: &'_ str) -> Result<File, String> {
    Ok(grammar::FileParser::new().parse(input).unwrap())
}
