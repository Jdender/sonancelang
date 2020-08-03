use super::{
    super::{ast, semantic},
    SymbolInfo, SymbolTable,
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
            ty: self.ty.visit_ast(symbol_table)?,
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

impl AstVisitor for ast::Ty {
    type Output = semantic::Ty;

    fn visit_ast(&self, _: &SymbolTable) -> Result<Self::Output, String> {
        use semantic::Ty::*;

        Ok(match self {
            Self::I32 => I32,
            Self::F32 => F32,
        })
    }
}

impl AstVisitor for ast::Block {
    type Output = semantic::Block;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Result<Self::Output, String> {
        let mut symbol_table = symbol_table.fork();
        let mut body = Vec::with_capacity(self.body.len());

        for stmt in self.body.iter() {
            body.push(match stmt {
                ast::Statement::LetBinding { place, value, ty } => {
                    let value = value.visit_ast(&symbol_table)?;

                    // Infer type if not declared
                    let ty = if let Some(ty) = ty {
                        ty.visit_ast(&symbol_table)?
                    } else {
                        value.ty
                    };

                    // Assert types match
                    if ty != value.ty {
                        return Err(format!(
                            "Type mismatch: Let declared as {:?} but got an expr of {:?}",
                            ty, value.ty,
                        ));
                    }

                    // Create a new symbol in the current scope
                    let symbol = symbol_table.set(place.as_string().clone(), SymbolInfo::new(ty));

                    semantic::Statement::LetBinding {
                        place: place.visit_ast(&symbol_table)?,
                        symbol_id: symbol.id(),
                        ty,
                        value,
                    }
                }
                ast::Statement::SideEffect(expr) => {
                    semantic::Statement::SideEffect(expr.visit_ast(&symbol_table)?)
                }
            });
        }

        let trailing = Box::new(self.trailing.visit_ast(&symbol_table)?);

        // Blocks return their trailing expr, same goes for types
        Ok(semantic::Block {
            body,
            ty: trailing.ty,
            trailing,
        })
    }
}

impl AstVisitor for ast::Expression {
    type Output = semantic::Expression;

    fn visit_ast(&self, symbol_table: &SymbolTable) -> Result<Self::Output, String> {
        use semantic::ExpressionKind::*;

        Ok(match self {
            Self::Literal(literal) => {
                let literal = literal.visit_ast(symbol_table)?;
                semantic::Expression {
                    ty: literal.into(),
                    kind: Literal(literal),
                }
            }

            Self::Lookup(place) => {
                // Lookup symbol
                let symbol = symbol_table
                    .get(place.as_string())
                    .ok_or(format!("Could not find place: {}", place.as_string()))?;

                semantic::Expression {
                    ty: symbol.ty(),
                    kind: Lookup {
                        place: place.visit_ast(symbol_table)?,
                        symbol_id: symbol.id(),
                    },
                }
            }

            Self::Block(block) => {
                let block = block.visit_ast(symbol_table)?;
                semantic::Expression {
                    ty: block.ty,
                    kind: Block(block),
                }
            }

            Self::Assignment { place, value } => {
                // Lookup symbol
                let symbol = symbol_table
                    .get(place.as_string())
                    .ok_or(format!("Could not find place: {}", place.as_string()))?;

                let value = value.visit_ast(symbol_table)?;

                // Assert types match
                if symbol.ty() != value.ty {
                    return Err(format!(
                        "Type mismatch: Let declared as {:?} but got an expr of {:?}",
                        symbol.ty(),
                        value.ty,
                    ));
                }

                semantic::Expression {
                    ty: symbol.ty(),
                    kind: Assignment {
                        place: place.visit_ast(symbol_table)?,
                        value: Box::new(value),
                        symbol_id: symbol.id(),
                    },
                }
            }

            Self::PrefixCall { operator, value } => {
                let value = value.visit_ast(symbol_table)?;
                semantic::Expression {
                    ty: value.ty,
                    kind: PrefixCall {
                        operator: operator.visit_ast(symbol_table)?,
                        value: Box::new(value),
                    },
                }
            }

            Self::InfixCall {
                left,
                operator,
                right,
            } => {
                let left = left.visit_ast(symbol_table)?;
                let right = right.visit_ast(symbol_table)?;

                if left.ty != right.ty {
                    return Err(format!(
                        "Type mismatch: Left {:?} and right {:?} can't use {:?} together",
                        left.ty, right.ty, operator
                    ));
                }

                semantic::Expression {
                    ty: left.ty,
                    kind: InfixCall {
                        left: Box::new(left),
                        operator: operator.visit_ast(symbol_table)?,
                        right: Box::new(right),
                    },
                }
            }
        })
    }
}

impl AstVisitor for ast::Literal {
    type Output = semantic::Literal;

    fn visit_ast(&self, _: &SymbolTable) -> Result<Self::Output, String> {
        use semantic::Literal::*;

        Ok(match self {
            Self::I32(num) => I32(*num),
            Self::F32(num) => F32(*num),
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
