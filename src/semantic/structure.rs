use super::SymbolId;

#[derive(Debug, Clone)]
pub struct File {
    pub name: Identifier,
    pub ty: Ty,
    pub body: Block,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ty {
    I32,
    F32,
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
    I32(i32),
    F32(f32),
}

impl From<Literal> for Ty {
    fn from(literal: Literal) -> Self {
        use Literal::*;

        match literal {
            I32(_) => Self::I32,
            F32(_) => Self::F32,
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
}
