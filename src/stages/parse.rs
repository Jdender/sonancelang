use lalrpop_util::lexer::Token;
use crate::parser::BlockParser;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, &'a str>;

pub fn parse(input: &str) -> Result<Block, ParseError> {
    BlockParser::new().parse(input)
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    pub trailing: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum Statement {
    SideEffect(Expression),
    Assignment(Identifier, Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Operation(Box<Expression>, Opcode, Box<Expression>),
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
