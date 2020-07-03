use crate::parser::ModuleParser;
use lalrpop_util::lexer::Token;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, &'a str>;

pub type OptionBox<T> = Option<Box<T>>;

pub fn parse(input: &str) -> Result<Module, ParseError> {
    ModuleParser::new().parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Import(ImportItem),
    Function(FunctionItem),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportItem(pub Path);

#[derive(Debug, Clone, PartialEq)]
pub enum Path {
    End(Identifier, Option<Identifier>),
    Normal(Identifier, PathKind),
    Arrow(Identifier, PathKind),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathKind {
    Single(Box<Path>),
    Multiple(Vec<Path>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionItem {
    pub name: Identifier,
    pub arguments: Vec<Argument>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
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
pub struct Block {
    pub body: Vec<Statement>,
    pub trailing: OptionBox<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    SideEffect(Expression),
    Assignment(Pattern, Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Lookup(Identifier),
    Block(Block),
    Call(Box<Expression>, Vec<Expression>),
    Return(OptionBox<Expression>),
    Match(Box<Expression>, Vec<MatchCase>),
    If {
        cases: Vec<PartialIf>,
        otherwise: Option<Block>,
    },
    Loop(Block),
    For {
        pattern: Pattern,
        iterator: Box<Expression>,
        body: Block,
        otherwise: Option<Block>,
    },
    While {
        condition: Box<ConditionOrPattern>,
        body: Block,
        otherwise: Option<Block>,
    },
    Break(OptionBox<Expression>),
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub result: Expression,
    pub guard: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartialIf(pub ConditionOrPattern, pub Block);

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionOrPattern {
    Condition(Expression),
    Pattern(Pattern, Expression),
}
