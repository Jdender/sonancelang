use crate::semantic::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub struct WasmModule {
    pub name: String,
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Const(i32),
    LocalGet(SymbolId),
    LocalDeclare(SymbolId, Box<Expression>),
    LocalSet(SymbolId, Box<Expression>),
    Block(Vec<Expression>),
    Return(Box<Expression>),
    SimpleInfixCall(Box<Expression>, SimpleInfix, Box<Expression>),
    Negate(Box<Expression>),
    BooleanNot(Box<Expression>),
    BooleanOr(Box<Expression>, Box<Expression>),
    BooleanAnd(Box<Expression>, Box<Expression>),
    Conditional(Box<Expression>, Vec<Expression>, Vec<Expression>),
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
