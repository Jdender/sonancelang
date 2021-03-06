use super::*;

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Declare(DeclareBlock),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct DeclareBlock {
    pub functions: Vec<DeclareFunction>,
}

#[derive(Debug, Clone)]
pub struct DeclareFunction {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub ty: Ty,
    pub symbol_id: SymbolId,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub scope: Scope,
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub ty: Ty,
    pub body: Block,
    pub symbol_id: SymbolId,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub ty: Ty,
    pub symbol_id: SymbolId,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    pub trailing: Box<Expression>,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetBinding {
        place: Identifier,
        ty: Ty,
        value: Expression,
        symbol_id: SymbolId,
    },
    SideEffect(Expression),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Literal(Literal),
    Lookup {
        place: Identifier,
        symbol_id: SymbolId,
    },
    Block(Block),
    Assignment {
        place: Identifier,
        value: Box<Expression>,
        symbol_id: SymbolId,
    },
    FuncCall {
        name: Identifier,
        args: Vec<Expression>,
        symbol_id: SymbolId,
    },
    PrefixCall {
        operator: PrefixOperator,
        value: Box<Expression>,
    },
    InfixCall {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    IfElse {
        predicate: Box<Expression>,
        when_true: Block,
        when_false: Block,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    F32(f32),
    F64(f64),
}

impl From<Literal> for Ty {
    fn from(literal: Literal) -> Self {
        use Literal::*;

        match literal {
            I8(_) => Self::I8,
            I16(_) => Self::I16,
            I32(_) => Self::I32,
            I64(_) => Self::I64,
            ISize(_) => Self::ISize,
            U8(_) => Self::U8,
            U16(_) => Self::U16,
            U32(_) => Self::U32,
            U64(_) => Self::U64,
            USize(_) => Self::USize,
            F32(_) => Self::F32,
            F64(_) => Self::F64,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrefixOperator {
    Negate,
}

#[derive(Debug, Clone)]
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
}
