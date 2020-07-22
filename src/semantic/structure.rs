use super::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub name: Identifier,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub body: Vec<Statement>,
    pub trailing: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetBinding(Identifier, SymbolId, Expression),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(i32),
    Lookup(Identifier, SymbolId),
    Block(Block),
    Assignment(Identifier, SymbolId, Box<Expression>),
    ReturnValue(Box<Expression>),
    PrefixCall(PrefixOp, Box<Expression>),
    InfixCall(Box<Expression>, InfixOp, Box<Expression>),
    Conditional {
        predicate: Box<Expression>,
        when_true: Block,
        when_false: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOp {
    Negate,
    BooleanNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOp {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,

    BooleanOr,
    BooleanAnd,
}
