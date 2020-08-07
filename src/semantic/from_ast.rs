use super::{
    super::{ast, semantic},
    SemanticError, Symbol, SymbolTable,
};

pub trait AstVisitor {
    type Output;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError>;
}

impl AstVisitor for ast::File {
    type Output = semantic::File;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let mut items = Vec::with_capacity(self.items.len());
        let mut symbol_table = symbol_table.fork();

        for item in self.items {
            // Convert the func head first
            let partial = item.visit_ast(&mut symbol_table)?;
            // Add func head to symbol table
            symbol_table.set(
                partial.head.name.clone(),
                Symbol::new_func(partial.head.clone()),
            );
            // Convert the rest of the func
            items.push(partial.visit_ast(&mut symbol_table)?);
        }

        Ok(semantic::File { items })
    }
}

#[derive(Debug, Clone)]
pub struct PartialFunction {
    pub head: semantic::FunctionHead,
    pub body: ast::Block,
}

impl AstVisitor for ast::Function {
    type Output = PartialFunction;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(PartialFunction {
            head: semantic::FunctionHead {
                scope: self.scope.visit_ast(symbol_table)?,
                name: self.name.visit_ast(symbol_table)?,
                ty: self.ty.visit_ast(symbol_table)?,
            },
            body: self.body,
        })
    }
}

impl AstVisitor for PartialFunction {
    type Output = semantic::Function;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let body = self.body.visit_ast(symbol_table)?;

        // Assert types match
        if self.head.ty != body.ty {
            return Err(SemanticError::TyMismatchReturn {
                expected: self.head.ty,
                found: body.ty,
            });
        }

        Ok(semantic::Function {
            head: self.head,
            body,
        })
    }
}

impl AstVisitor for ast::Scope {
    type Output = semantic::Scope;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::Scope::*;
        Ok(match self {
            Self::Local => Local,
            Self::Export => Export,
        })
    }
}

impl AstVisitor for ast::Identifier {
    type Output = semantic::Identifier;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(semantic::Identifier::new(self.as_string().clone()))
    }
}

impl AstVisitor for ast::Ty {
    type Output = semantic::Ty;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::Ty::*;

        Ok(match self {
            Self::I32 => I32,
            Self::F32 => F32,
        })
    }
}

impl AstVisitor for ast::Block {
    type Output = semantic::Block;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let symbol_table = &mut symbol_table.fork();

        let body = self
            .body
            .into_iter()
            .map(|s| s.visit_ast(symbol_table))
            .collect::<Result<_, _>>()?;

        let trailing = Box::new(self.trailing.visit_ast(symbol_table)?);

        // Blocks return their trailing expr, same goes for types
        Ok(semantic::Block {
            body,
            ty: trailing.ty,
            trailing,
        })
    }
}

impl AstVisitor for ast::Statement {
    type Output = semantic::Statement;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(match self {
            ast::Statement::LetBinding { place, value, ty } => {
                let place = place.visit_ast(symbol_table)?;
                let value = value.visit_ast(symbol_table)?;

                // Infer type if not declared
                let ty = if let Some(ty) = ty {
                    ty.visit_ast(symbol_table)?
                } else {
                    value.ty
                };

                // Assert types match
                if ty != value.ty {
                    return Err(SemanticError::TyMismatchDeclare {
                        expected: ty,
                        found: value.ty,
                    });
                }

                // Create a new symbol in the current scope
                symbol_table.set(place.clone(), Symbol::new_local(ty));
                let symbol_id = symbol_table
                    .get(&place)
                    .expect("Should get back what we set")
                    .id();

                semantic::Statement::LetBinding {
                    place,
                    symbol_id,
                    ty,
                    value,
                }
            }
            ast::Statement::SideEffect(expr) => {
                semantic::Statement::SideEffect(expr.visit_ast(symbol_table)?)
            }
        })
    }
}

impl AstVisitor for ast::Expression {
    type Output = semantic::Expression;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
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
                let place = place.visit_ast(symbol_table)?;

                // Lookup symbol
                let symbol =
                    symbol_table
                        .get(&place)
                        .ok_or_else(|| SemanticError::SymbolNotFound {
                            symbol: place.clone(),
                        })?;

                let ty = symbol
                    .as_local()
                    .ok_or_else(|| SemanticError::ExpectedLocalSymbol {
                        symbol: place.clone(),
                    })?
                    .ty;

                semantic::Expression {
                    ty,
                    kind: Lookup {
                        place,
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
                let place = place.visit_ast(symbol_table)?;

                // Lookup symbol
                let symbol =
                    symbol_table
                        .get(&place)
                        .ok_or_else(|| SemanticError::SymbolNotFound {
                            symbol: place.clone(),
                        })?;

                let ty = symbol
                    .as_local()
                    .ok_or_else(|| SemanticError::ExpectedLocalSymbol {
                        symbol: place.clone(),
                    })?
                    .ty;

                let symbol_id = symbol.id();
                let value = value.visit_ast(symbol_table)?;

                // Assert types match
                if ty != value.ty {
                    return Err(SemanticError::TyMismatchAssign {
                        expected: ty,
                        found: value.ty,
                    });
                }

                semantic::Expression {
                    ty,
                    kind: Assignment {
                        place,
                        value: Box::new(value),
                        symbol_id,
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
                right,
                operator,
            } => {
                let operator = operator.visit_ast(symbol_table)?;
                let left = left.visit_ast(symbol_table)?;
                let right = right.visit_ast(symbol_table)?;

                if left.ty != right.ty {
                    return Err(SemanticError::TyMismatchOperator {
                        operator,
                        left: left.ty,
                        right: right.ty,
                    });
                }

                semantic::Expression {
                    ty: left.ty,
                    kind: InfixCall {
                        operator,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }

            Self::IfElse {
                predicate,
                when_true,
                when_false,
            } => {
                let when_true = when_true.visit_ast(symbol_table)?;
                let when_false = when_false.visit_ast(symbol_table)?;

                if when_true.ty != when_false.ty {
                    return Err(SemanticError::TyMismatchIfElse {
                        when_true: when_true.ty,
                        when_false: when_false.ty,
                    });
                }

                semantic::Expression {
                    ty: when_true.ty,
                    kind: IfElse {
                        predicate: Box::new(predicate.visit_ast(symbol_table)?),
                        when_true,
                        when_false,
                    },
                }
            }
        })
    }
}

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
