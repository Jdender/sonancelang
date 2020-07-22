use crate::semantic::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub struct WasmModule {
    pub name: String,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub body: Vec<Expression>,
    pub trailing: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Const(i32),
    LocalGet(SymbolId),
    LocalDeclare(SymbolId, Box<Expression>),
    LocalSet(SymbolId, Box<Expression>),
    Block(Block),
    Return(Box<Expression>),
    SimpleInfixCall {
        operator: SimpleInfix,
        x_operand: Box<Expression>,
        y_operand: Box<Expression>,
    },
    Negate(Box<Expression>),
    BooleanNot(Box<Expression>),
    BooleanOr(Box<Expression>, Box<Expression>),
    BooleanAnd(Box<Expression>, Box<Expression>),
    Conditional {
        predicate: Box<Expression>,
        when_true: Block,
        when_false: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleInfix {
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
}
