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
    LetBinding {
        place: Identifier,
        symbol_id: SymbolId,
        operand: Expression,
    },
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(i32),
    Lookup {
        place: Identifier,
        symbol_id: SymbolId,
    },
    Block(Block),
    Assignment {
        place: Identifier,
        symbol_id: SymbolId,
        operand: Box<Expression>,
    },
    ReturnValue(Box<Expression>),
    PrefixCall {
        operator: PrefixOperator,
        operand: Box<Expression>,
    },
    InfixCall {
        operator: InfixOperator,
        x_operand: Box<Expression>,
        y_operand: Box<Expression>,
    },
    Conditional {
        predicate: Box<Expression>,
        when_true: Block,
        when_false: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOperator {
    Negate,
    BooleanNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOperator {
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
