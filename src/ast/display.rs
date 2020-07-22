use super::structure::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "func {}() -> I32 {}", self.name, self.body)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, r"{}", self.0)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = self
            .body
            .iter()
            .map(|smt| format!("{:indent$}{}\n", "   ", smt, indent = 4))
            .collect::<String>();

        write!(f, "{{\n{} {}\n}}\n", body, self.trailing)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Statement::LetBinding { place, operand } => write!(f, "let {} = {};", place, operand),
            Statement::Expression(expr) => write!(f, "{};", expr),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Expression::Literal(num) => write!(f, "{}", num),
            Expression::Lookup(ident) => write!(f, "{}", ident),
            Expression::Block(block) => write!(f, "{}", block),
            Expression::Assignment { place, operand } => write!(f, "{} = {}", place, operand),
            Expression::ReturnValue(expr) => write!(f, "return {}", expr),
            Expression::PrefixCall { op, operand } => write!(f, "{}{}", op, operand),
            Expression::InfixCall {
                op,
                x_operand,
                y_operand,
            } => write!(f, "{} {} {}", x_operand, op, y_operand),
            Expression::Conditional {
                predicate,
                when_true,
                when_false,
            } => write!(f, "if {} {} else {}", predicate, when_true, when_false),
        }
    }
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
