use lalrpop_util::lexer::Token;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, &'a str>;

pub struct File(pub u32);
