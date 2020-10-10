pub mod structure;

lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/ast/grammar.rs");

use crate::Result;
pub use structure::*;

pub fn parse(input: &str) -> Result<File> {
    Ok(grammar::FileParser::new()
        .parse(input)
        .map_err(|err| err.to_string())?)
}
