use lalrpop_util::lexer::Token;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, &'a str>;

#[derive(Debug, Clone, PartialEq)]
pub struct File(pub Expression);

use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            r"
                func main() -> I32 {{
                    return {};
                }}
            ",
            self.0
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(i32),
    UnaryOp(UnaryOp, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Expression::Literal(num) => write!(f, "{}", num),
            Expression::UnaryOp(op, num) => write!(f, "{}{}", op, num),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    BooleanNot,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                UnaryOp::Negate => "-",
                UnaryOp::BooleanNot => "!",
            }
        )
    }
}
