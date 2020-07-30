use super::SymbolId;

#[derive(Debug, Clone)]
pub struct File {
    pub name: Identifier,
    pub return_type: Tye,
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

#[derive(Debug, Clone)]
pub enum Tye {
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
        symbol_id: SymbolId,
    },
    SideEffect(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
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
}

#[derive(Debug, Clone)]
pub enum Literal {
    I32(i32),
    F32(f32),
}

impl From<Literal> for Tye {
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
