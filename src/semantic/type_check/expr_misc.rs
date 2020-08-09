use super::*;

impl HeaderVisitor for ast::Literal {
    type Output = Literal;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use Literal::*;

        Ok(match self {
            Self::I8(num) => I8(num),
            Self::I16(num) => I16(num),
            Self::I32(num) => I32(num),
            Self::I64(num) => I64(num),
            Self::U8(num) => U8(num),
            Self::U16(num) => U16(num),
            Self::U32(num) => U32(num),
            Self::U64(num) => U64(num),
            Self::F32(num) => F32(num),
            Self::F64(num) => F64(num),
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
