use super::{
    super::{ast, semantic},
    symbol_table::SymbolTable,
    SemResult, SemanticError,
};

pub fn semantic_pass(input: ast::File) -> SemResult<semantic::File> {
    input.visit_ast(&SymbolTable::new())
}

pub trait AstVisitor {
    type Return;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Return;
}

impl AstVisitor for ast::File {
    type Return = SemResult<semantic::File>;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Return {
        Ok(semantic::File {
            name: self.name.visit_ast(symbol_table),
            body: self.body.visit_ast(symbol_table)?,
        })
    }
}

impl AstVisitor for ast::Identifier {
    type Return = semantic::Identifier;

    fn visit_ast(&self, _symbol_table: &SymbolTable) -> Self::Return {
        semantic::Identifier(self.to_string())
    }
}

impl AstVisitor for ast::Block {
    type Return = SemResult<semantic::Block>;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Return {
        let mut symbol_table = symbol_table.fork();
        let mut statements = Vec::new();

        for stmt in self.body.iter() {
            statements.push(match stmt {
                ast::Statement::LetBinding { place, operand } => {
                    let symbol_id = symbol_table.set(place.to_string());

                    semantic::Statement::LetBinding {
                        place: place.visit_ast(&symbol_table),
                        symbol_id,
                        operand: operand.visit_ast(&symbol_table)?,
                    }
                }
                ast::Statement::Expression(expr) => {
                    semantic::Statement::Expression(expr.visit_ast(&symbol_table)?)
                }
            });
        }

        Ok(semantic::Block {
            body: statements,
            trailing: Box::new(self.trailing.visit_ast(&symbol_table)?),
        })
    }
}

impl AstVisitor for ast::Expression {
    type Return = SemResult<semantic::Expression>;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Self::Return {
        Ok(match self {
            Self::Literal(num) => semantic::Expression::Literal(*num),
            Self::Lookup(place) => semantic::Expression::Lookup {
                place: place.visit_ast(symbol_table),
                symbol_id: symbol_table
                    .get(&place.0)
                    .ok_or_else(|| SemanticError::VariableNotDeclared(place.to_string()))?,
            },
            Self::Block(block) => semantic::Expression::Block(block.visit_ast(symbol_table)?),
            Self::Assignment { place, operand } => semantic::Expression::Assignment {
                place: place.visit_ast(symbol_table),
                symbol_id: symbol_table
                    .get(&place.0)
                    .ok_or_else(|| SemanticError::VariableNotDeclared(place.to_string()))?,
                operand: Box::new(operand.visit_ast(symbol_table)?),
            },
            Self::ReturnValue(expr) => {
                semantic::Expression::ReturnValue(Box::new(expr.visit_ast(symbol_table)?))
            }
            Self::PrefixCall { operator, operand } => semantic::Expression::PrefixCall {
                operator: operator.visit_ast(symbol_table),
                operand: Box::new(operand.visit_ast(symbol_table)?),
            },
            Self::InfixCall {
                operator,
                x_operand,
                y_operand,
            } => semantic::Expression::InfixCall {
                operator: operator.visit_ast(symbol_table),
                x_operand: Box::new(x_operand.visit_ast(symbol_table)?),
                y_operand: Box::new(y_operand.visit_ast(symbol_table)?),
            },
            Self::Conditional {
                predicate,
                when_true,
                when_false,
            } => semantic::Expression::Conditional {
                predicate: Box::new(predicate.visit_ast(symbol_table)?),
                when_true: when_true.visit_ast(symbol_table)?,
                when_false: when_false.visit_ast(symbol_table)?,
            },
        })
    }
}

impl AstVisitor for ast::PrefixOperator {
    type Return = semantic::PrefixOperator;

    fn visit_ast(&self, _symbol_table: &SymbolTable) -> Self::Return {
        match self {
            Self::Negate => semantic::PrefixOperator::Negate,
            Self::BooleanNot => semantic::PrefixOperator::BooleanNot,
        }
    }
}

impl AstVisitor for ast::InfixOperator {
    type Return = semantic::InfixOperator;

    fn visit_ast(&self, _symbol_table: &SymbolTable) -> Self::Return {
        match self {
            Self::Add => semantic::InfixOperator::Add,
            Self::Subtract => semantic::InfixOperator::Subtract,
            Self::Multiply => semantic::InfixOperator::Multiply,
            Self::Divide => semantic::InfixOperator::Divide,

            Self::Equal => semantic::InfixOperator::Equal,
            Self::NotEqual => semantic::InfixOperator::NotEqual,
            Self::GreaterThan => semantic::InfixOperator::GreaterThan,
            Self::LessThan => semantic::InfixOperator::LessThan,
            Self::GreaterOrEqual => semantic::InfixOperator::GreaterOrEqual,
            Self::LessOrEqual => semantic::InfixOperator::LessOrEqual,

            Self::BooleanOr => semantic::InfixOperator::BooleanOr,
            Self::BooleanAnd => semantic::InfixOperator::BooleanAnd,
        }
    }
}
