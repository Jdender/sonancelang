#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Function>,
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
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Ty {
    I32,
    F32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    pub trailing: Box<Expression>,
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
    I32(i32),
    F32(f32),
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
