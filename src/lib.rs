#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub(crate) parser);

pub mod parse;

#[cfg(test)]
mod test;
