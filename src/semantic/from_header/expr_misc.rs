use super::*;

impl HeaderVisitor for ast::Literal {
    type Output = Literal;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use Literal::*;

        Ok(match self {
            Self::I32(num) => I32(num),
            Self::F32(num) => F32(num),
        })
    }
}

impl HeaderVisitor for ast::PrefixOperator {
    type Output = PrefixOperator;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use PrefixOperator::*;

        Ok(match self {
            Self::Negate => Negate,
        })
    }
}

impl HeaderVisitor for ast::InfixOperator {
    type Output = InfixOperator;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use InfixOperator::*;

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
