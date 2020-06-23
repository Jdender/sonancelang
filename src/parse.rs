use crate::parser::ModuleParser;
use lalrpop_util::lexer::Token;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, &'a str>;

pub fn parse(input: &str) -> Result<Module, ParseError> {
    ModuleParser::new().parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(FunctionItem),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionItem {
    pub name: Identifier,
    pub arguments: Arguments,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    pub normal: Vec<Argument>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: Option<Identifier>,
    pub pattern: Pattern,
    pub declared_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: Identifier,
    pub arguments: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {}
