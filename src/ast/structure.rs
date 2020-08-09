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
}

#[derive(Debug, Clone)]
pub struct Function {
    pub scope: Scope,
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub ty: Ty,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Scope {
    Export,
    Local,
}

#[derive(Debug, Clone)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(ident: String) -> Self {
        Identifier(ident)
    }

    pub fn take(self) -> String {
        self.0
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Ty {
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    pub trailing: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetBinding {
        place: Identifier,
        value: Expression,
        ty: Option<Ty>,
    },
    SideEffect(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Lookup(Identifier),
    Block(Block),
    Assignment {
        place: Identifier,
        value: Box<Expression>,
    },
    FuncCall {
        name: Identifier,
        args: Vec<Expression>,
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

#[derive(Debug, Clone)]
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
