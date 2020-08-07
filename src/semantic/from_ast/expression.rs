use super::*;

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
                        .ok_or_else(|| SemanticError::LocalNotFound {
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
                        .ok_or_else(|| SemanticError::LocalNotFound {
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

            Self::FuncCall { name } => {
                let name = name.visit_ast(symbol_table)?;

                // Lookup symbol
                let symbol =
                    symbol_table
                        .get(&name)
                        .ok_or_else(|| SemanticError::FuncNotFound {
                            symbol: name.clone(),
                        })?;

                let ty = symbol
                    .as_func()
                    .ok_or_else(|| SemanticError::ExpectedFuncSymbol {
                        symbol: name.clone(),
                    })?
                    .ty;

                semantic::Expression {
                    ty,
                    kind: FuncCall {
                        name,
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
