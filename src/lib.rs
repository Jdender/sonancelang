#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub(crate) parser);

pub mod stages;

pub use stages::compile;
