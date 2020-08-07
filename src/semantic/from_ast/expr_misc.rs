use super::*;

impl AstVisitor for ast::Literal {
    type Output = semantic::Literal;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::Literal::*;

        Ok(match self {
            Self::I32(num) => I32(num),
            Self::F32(num) => F32(num),
        })
    }
}

impl AstVisitor for ast::PrefixOperator {
    type Output = semantic::PrefixOperator;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::PrefixOperator::*;

        Ok(match self {
            Self::Negate => Negate,
        })
    }
}

impl AstVisitor for ast::InfixOperator {
    type Output = semantic::InfixOperator;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::InfixOperator::*;

        Ok(match self {
            Self::Add => Add,
            Self::Subtract => Subtract,
            Self::Multiply => Multiply,
            Self::Divide => Divide,

            Self::Equal => Equal,
            Self::NotEqual => NotEqual,
            Self::GreaterThan => GreaterThan,
            Self::LessThan => LessThan,
            Self::GreaterOrEqual => GreaterOrEqual,
            Self::LessOrEqual => LessOrEqual,
        })
    }
}
