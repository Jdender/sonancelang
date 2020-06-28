#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod parse;

#[cfg(test)]
mod test;
