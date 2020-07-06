#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

pub mod parse;

#[cfg(test)]
mod test;
