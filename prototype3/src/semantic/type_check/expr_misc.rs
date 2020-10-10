use super::*;

impl ast::Literal {
    pub fn visit_header(self, _: &mut SymbolTable) -> Result<Literal, SemanticError> {
        use Literal::*;

        Ok(match self {
            Self::I8(num) => I8(num),
            Self::I16(num) => I16(num),
            Self::I32(num) => I32(num),
            Self::I64(num) => I64(num),
            Self::ISize(num) => ISize(num),
            Self::U8(num) => U8(num),
            Self::U16(num) => U16(num),
            Self::U32(num) => U32(num),
            Self::U64(num) => U64(num),
            Self::USize(num) => USize(num),
            Self::F32(num) => F32(num),
            Self::F64(num) => F64(num),
        })
    }
}

impl ast::PrefixOperator {
    pub fn visit_header(self, _: &mut SymbolTable) -> Result<PrefixOperator, SemanticError> {
        use PrefixOperator::*;

        Ok(match self {
            Self::Negate => Negate,
        })
    }
}

impl ast::InfixOperator {
    pub fn visit_header(self, _: &mut SymbolTable) -> Result<InfixOperator, SemanticError> {
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
