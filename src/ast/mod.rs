pub mod display;
pub mod structure;

pub use structure::*;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'a>, &'a str>;
