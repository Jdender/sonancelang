lalrpop_mod!(pub expr);

use lalrpop_util::{lexer::Token, ParseError};

pub fn parse(input: &str) -> Result<Box<Expr>, ParseError<usize, Token, &str>> {
    expr::ExprParser::new().parse(input)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Operation(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}
