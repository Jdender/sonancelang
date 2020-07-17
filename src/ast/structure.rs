use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub name: Identifier,
    pub body: Block,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "func {}() -> I32 {}", self.name, self.body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, r"{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Statement>);

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = self
            .0
            .iter()
            .map(|smt| format!("{:indent$}{}\n", "   ", smt, indent = 4))
            .collect::<String>();

        write!(f, "{{\n{}}}\n", body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetBinding(Identifier, Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Statement::LetBinding(name, expr) => write!(f, "let {} = {};", name, expr),
            Statement::Expression(expr) => write!(f, "{};", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(i32),
    Lookup(Identifier),
    Assignment(Identifier, Box<Expression>),
    Return(Box<Expression>),
    PrefixCall(PrefixOp, Box<Expression>),
    InfixCall(Box<Expression>, InfixOp, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Expression::Literal(num) => write!(f, "{}", num),
            Expression::Lookup(ident) => write!(f, "{}", ident),
            Expression::Assignment(name, expr) => write!(f, "{} = {}", name, expr),
            Expression::Return(expr) => write!(f, "return {}", expr),
            Expression::PrefixCall(op, expr) => write!(f, "{}{}", op, expr),
            Expression::InfixCall(x, op, y) => write!(f, "{} {} {}", x, op, y),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOp {
    Negate,
    BooleanNot,
}

impl Display for PrefixOp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                PrefixOp::Negate => "-",
                PrefixOp::BooleanNot => "!",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOp {
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

    BooleanOr,
    BooleanAnd,
}

impl Display for InfixOp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                InfixOp::Add => "+",
                InfixOp::Subtract => "-",
                InfixOp::Multiply => "*",
                InfixOp::Divide => "/",

                InfixOp::Equal => "==",
                InfixOp::NotEqual => "!=",
                InfixOp::GreaterThan => ">",
                InfixOp::LessThan => "<",
                InfixOp::GreaterOrEqual => ">=",
                InfixOp::LessOrEqual => "<=",

                InfixOp::BooleanOr => "||",
                InfixOp::BooleanAnd => "&&",
            }
        )
    }
}
