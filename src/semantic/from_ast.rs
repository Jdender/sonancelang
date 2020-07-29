use super::{
    super::{ast, semantic},
    SymbolTable,
};

pub trait AstVisitor {
    type Output;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Output;
}

impl AstVisitor for ast::File {
    type Output = semantic::File;
    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Output {
        semantic::File {
            name: self.name.visit_ast(symbol_table),
            body: self.body.visit_ast(symbol_table),
        }
    }
}

impl AstVisitor for ast::Identifier {
    type Output = semantic::Identifier;
    fn visit_ast(&self, _: &SymbolTable) -> Self::Output {
        semantic::Identifier::new(self.as_string().clone())
    }
}

impl AstVisitor for ast::Expression {
    type Output = semantic::Expression;
    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Output {
        use semantic::Expression::*;
        match self {
            Self::Literal(num) => Literal(*num),
            Self::PrefixCall { operator, value } => PrefixCall {
                operator: operator.visit_ast(symbol_table),
                value: Box::new(value.visit_ast(symbol_table)),
            },
            Self::InfixCall {
                left,
                operator,
                right,
            } => InfixCall {
                left: Box::new(left.visit_ast(symbol_table)),
                operator: operator.visit_ast(symbol_table),
                right: Box::new(right.visit_ast(symbol_table)),
            },
        }
    }
}

impl AstVisitor for ast::PrefixOperator {
    type Output = semantic::PrefixOperator;
    fn visit_ast(&self, _: &SymbolTable) -> Self::Output {
        use semantic::PrefixOperator::*;
        match self {
            Self::Negate => Negate,
        }
    }
}

impl AstVisitor for ast::InfixOperator {
    type Output = semantic::InfixOperator;
    fn visit_ast(&self, _: &SymbolTable) -> Self::Output {
        use semantic::InfixOperator::*;
        match self {
            Self::Add => Add,
            Self::Subtract => Subtract,
            Self::Multiply => Multiply,
            Self::Divide => Divide,
        }
    }
}