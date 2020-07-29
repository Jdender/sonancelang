#[derive(Debug, Clone)]
pub struct File {
    pub name: Identifier,
    pub body: Expression,
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
pub enum Expression {
    Literal(i32),
    InfixCall {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum InfixOperator {
    Add,
}
