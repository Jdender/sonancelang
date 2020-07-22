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
    LetBinding(Identifier, Expression),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(i32),
    Lookup(Identifier),
    Block(Block),
    Assignment(Identifier, Box<Expression>),
    ReturnValue(Box<Expression>),
    PrefixCall(PrefixOp, Box<Expression>),
    InfixCall(Box<Expression>, InfixOp, Box<Expression>),
    Conditional(Box<Expression>, Block, Block),
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
