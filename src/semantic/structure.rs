use super::SymbolId;

#[derive(Debug, Clone)]
pub struct File {
    pub name: Identifier,
    pub return_type: Type,
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
pub enum Type {
    I32,
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
    Literal(i32),
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
