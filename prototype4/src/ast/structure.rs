#[derive(Debug, Clone)]
pub struct File {
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Call(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
}
