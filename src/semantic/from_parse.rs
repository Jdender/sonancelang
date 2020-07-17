use super::super::semantic;
use crate::ast;

pub trait AstVisitor {
    type Argument;
    type Return;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return;
}

impl AstVisitor for ast::File {
    type Argument = ();
    type Return = semantic::File;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        semantic::File {
            name: self.name.visit_ast(()),
            body: self.body.visit_ast(()),
        }
    }
}

impl AstVisitor for ast::Identifier {
    type Argument = ();
    type Return = semantic::Identifier;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        semantic::Identifier(self.0)
    }
}

impl AstVisitor for ast::Block {
    type Argument = ();
    type Return = semantic::Block;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        semantic::Block(self.0.iter().map(|stmt| stmt.visit_ast(())).collect())
    }
}

impl AstVisitor for ast::Statement {
    type Argument = ();
    type Return = semantic::Statement;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        match self {
            Self::LetBinding(ident, expr) => {
                semantic::Statement::LetBinding(ident.visit_ast(()), expr.visit_ast(()))
            }
            Self::Expression(expr) => semantic::Statement::Expression(expr.visit_ast(())),
        }
    }
}

impl AstVisitor for ast::Expression {
    type Argument = ();
    type Return = semantic::Expression;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        match self {
            Self::Literal(num) => semantic::Expression::Literal(*num),
            Self::Lookup(ident) => semantic::Expression::Lookup(ident.visit_ast(())),
            Self::Assignment(ident, expr) => {
                semantic::Expression::Assignment(ident.visit_ast(()), Box::new(expr.visit_ast(())))
            }
            Self::ReturnValue(expr) => {
                semantic::Expression::ReturnValue(Box::new(expr.visit_ast(())))
            }
            Self::PrefixCall(op, expr) => {
                semantic::Expression::PrefixCall(op.visit_ast(()), Box::new(expr.visit_ast(())))
            }
            Self::InfixCall(x, op, y) => semantic::Expression::InfixCall(
                Box::new(x.visit_ast(())),
                op.visit_ast(()),
                Box::new(y.visit_ast(())),
            ),
        }
    }
}

impl AstVisitor for ast::PrefixOp {
    type Argument = ();
    type Return = semantic::PrefixOp;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        match self {
            Self::Negate => semantic::PrefixOp::Negate,
            Self::BooleanNot => semantic::PrefixOp::BooleanNot,
        }
    }
}

impl AstVisitor for ast::InfixOp {
    type Argument = ();
    type Return = semantic::InfixOp;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return {
        match self {
            Self::Add => semantic::InfixOp::Add,
            Self::Subtract => semantic::InfixOp::Subtract,
            Self::Multiply => semantic::InfixOp::Multiply,
            Self::Divide => semantic::InfixOp::Divide,

            Self::Equal => semantic::InfixOp::Equal,
            Self::NotEqual => semantic::InfixOp::NotEqual,
            Self::GreaterThan => semantic::InfixOp::GreaterThan,
            Self::LessThan => semantic::InfixOp::LessThan,
            Self::GreaterOrEqual => semantic::InfixOp::GreaterOrEqual,
            Self::LessOrEqual => semantic::InfixOp::LessOrEqual,

            Self::BooleanOr => semantic::InfixOp::BooleanOr,
            Self::BooleanAnd => semantic::InfixOp::BooleanAnd,
        }
    }
}
