use super::{
    super::{ast, semantic},
    SymbolTable,
};

pub trait AstVisitor {
    type Output;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Result<Self::Output, String>;
}

impl AstVisitor for ast::File {
    type Output = semantic::File;
    fn visit_ast(&self, symbol_table: &SymbolTable) -> Result<Self::Output, String> {
        Ok(semantic::File {
            name: self.name.visit_ast(symbol_table)?,
            body: self.body.visit_ast(symbol_table)?,
        })
    }
}

impl AstVisitor for ast::Identifier {
    type Output = semantic::Identifier;
    fn visit_ast(&self, _: &SymbolTable) -> Result<Self::Output, String> {
        Ok(semantic::Identifier::new(self.as_string().clone()))
    }
}

impl AstVisitor for ast::Block {
    type Output = semantic::Block;
    fn visit_ast(&self, symbol_table: &SymbolTable) -> Result<Self::Output, String> {
        let mut symbol_table = symbol_table.fork();
        let mut body = Vec::with_capacity(self.body.len());

        for stmt in self.body.iter() {
            body.push(match stmt {
                ast::Statement::LetBinding { place, value } => {
                    let symbol_id = symbol_table.set(place.as_string().clone());

                    semantic::Statement::LetBinding {
                        place: place.visit_ast(&symbol_table)?,
                        value: value.visit_ast(&symbol_table)?,
                        symbol_id,
                    }
                }
                ast::Statement::SideEffect(expr) => {
                    semantic::Statement::SideEffect(expr.visit_ast(&symbol_table)?)
                }
            });
        }

        let trailing = self.trailing.visit_ast(&symbol_table)?;

        Ok(semantic::Block { body, trailing })
    }
}

impl AstVisitor for ast::Expression {
    type Output = semantic::Expression;
    fn visit_ast(&self, symbol_table: &SymbolTable) -> Result<Self::Output, String> {
        use semantic::Expression::*;
        Ok(match self {
            Self::Literal(num) => Literal(*num),
            Self::Lookup(place) => semantic::Expression::Lookup {
                place: place.visit_ast(symbol_table)?,
                symbol_id: symbol_table
                    .get(place.as_string())
                    .ok_or(format!("Could not find place: {}", place.as_string()))?,
            },
            Self::Assignment { place, value } => semantic::Expression::Assignment {
                place: place.visit_ast(symbol_table)?,
                value: Box::new(value.visit_ast(symbol_table)?),
                symbol_id: symbol_table
                    .get(place.as_string())
                    .ok_or(format!("Could not find place: {}", place.as_string()))?,
            },
            Self::PrefixCall { operator, value } => PrefixCall {
                operator: operator.visit_ast(symbol_table)?,
                value: Box::new(value.visit_ast(symbol_table)?),
            },
            Self::InfixCall {
                left,
                operator,
                right,
            } => InfixCall {
                left: Box::new(left.visit_ast(symbol_table)?),
                operator: operator.visit_ast(symbol_table)?,
                right: Box::new(right.visit_ast(symbol_table)?),
            },
        })
    }
}

impl AstVisitor for ast::PrefixOperator {
    type Output = semantic::PrefixOperator;
    fn visit_ast(&self, _: &SymbolTable) -> Result<Self::Output, String> {
        use semantic::PrefixOperator::*;
        Ok(match self {
            Self::Negate => Negate,
        })
    }
}

impl AstVisitor for ast::InfixOperator {
    type Output = semantic::InfixOperator;
    fn visit_ast(&self, _: &SymbolTable) -> Result<Self::Output, String> {
        use semantic::InfixOperator::*;
        Ok(match self {
            Self::Add => Add,
            Self::Subtract => Subtract,
            Self::Multiply => Multiply,
            Self::Divide => Divide,
        })
    }
}
