#[derive(Debug, Clone, PartialEq)]
pub struct WasmModule {
    pub name: String,
    pub body: Vec<WasmExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WasmExpression {
    Const(i32),
    LocalGet(String),
    LocalDeclare(String, Box<WasmExpression>),
    LocalSet(String, Box<WasmExpression>),
    Return(Box<WasmExpression>),
    SimpleInfixCall(Box<WasmExpression>, WasmSimpleInfix, Box<WasmExpression>),
    Negate(Box<WasmExpression>),
    BooleanNot(Box<WasmExpression>),
    BooleanOr(Box<WasmExpression>, Box<WasmExpression>),
    BooleanAnd(Box<WasmExpression>, Box<WasmExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WasmSimpleInfix {
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
