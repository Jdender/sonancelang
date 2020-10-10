#[macro_use]
extern crate lalrpop_util;

mod ast;
mod eval;

use std::error::Error;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub fn run(input: &str) -> Result<()> {
    eval::eval(ast::parse(input)?)
}
