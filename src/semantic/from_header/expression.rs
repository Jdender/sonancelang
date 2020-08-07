use {super::*, std::cmp::Ordering};

impl HeaderVisitor for ast::Expression {
    type Output = Expression;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use ExpressionKind::*;

        Ok(match self {
            Self::Literal(literal) => {
                let literal = literal.visit_header(symbol_table)?;
                Expression {
                    ty: literal.into(),
                    kind: Literal(literal),
                }
            }

            Self::Lookup(place) => {
                let place = place.visit_common();

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

                Expression {
                    ty,
                    kind: Lookup {
                        place,
                        symbol_id: symbol.id(),
                    },
                }
            }

            Self::Block(block) => {
                let block = block.visit_header(symbol_table)?;
                Expression {
                    ty: block.ty,
                    kind: Block(block),
                }
            }

            Self::Assignment { place, value } => {
                let place = place.visit_common();

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
                let value = value.visit_header(symbol_table)?;

                // Assert types match
                if ty != value.ty {
                    return Err(SemanticError::TyMismatchAssign {
                        expected: ty,
                        found: value.ty,
                    });
                }

                Expression {
                    ty,
                    kind: Assignment {
                        place,
                        value: Box::new(value),
                        symbol_id,
                    },
                }
            }

            Self::FuncCall { name, args } => {
                let name = name.visit_common();

                // Lookup symbol
                let symbol =
                    symbol_table
                        .get(&name)
                        .ok_or_else(|| SemanticError::FuncNotFound {
                            symbol: name.clone(),
                        })?;

                let func = symbol
                    .as_func()
                    .ok_or_else(|| SemanticError::ExpectedFuncSymbol {
                        symbol: name.clone(),
                    })?;

                let symbol_id = symbol.id();
                let ty = func.ty;
                let params = func.params.clone();

                let args = args
                    .into_iter()
                    .map(|a| a.visit_header(symbol_table))
                    .collect::<Result<Vec<_>, _>>()?;

                // Make sure arg and param size match
                match args.len().cmp(&params.len()) {
                    Ordering::Less => {
                        return Err(SemanticError::NotEnoughArgs {
                            expected: params.len(),
                            found: args.len(),
                        })
                    }
                    Ordering::Greater => {
                        return Err(SemanticError::TooManyArgs {
                            expected: params.len(),
                            found: args.len(),
                        })
                    }
                    Ordering::Equal => { /* Check's out, do nothing */ }
                }

                // Make sure arg and param types match
                for (position, (found, expected)) in args
                    .iter()
                    .map(|a| a.ty)
                    .zip(params.into_iter())
                    .enumerate()
                {
                    if found != expected {
                        return Err(SemanticError::TyMismatchArg {
                            expected,
                            found,
                            position,
                        });
                    }
                }

                Expression {
                    ty,
                    kind: FuncCall {
                        name,
                        args,
                        symbol_id,
                    },
                }
            }

            Self::PrefixCall { operator, value } => {
                let value = value.visit_header(symbol_table)?;
                Expression {
                    ty: value.ty,
                    kind: PrefixCall {
                        operator: operator.visit_header(symbol_table)?,
                        value: Box::new(value),
                    },
                }
            }

            Self::InfixCall {
                left,
                right,
                operator,
            } => {
                let operator = operator.visit_header(symbol_table)?;
                let left = left.visit_header(symbol_table)?;
                let right = right.visit_header(symbol_table)?;

                if left.ty != right.ty {
                    return Err(SemanticError::TyMismatchOperator {
                        operator,
                        left: left.ty,
                        right: right.ty,
                    });
                }

                Expression {
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
                let when_true = when_true.visit_header(symbol_table)?;
                let when_false = when_false.visit_header(symbol_table)?;

                if when_true.ty != when_false.ty {
                    return Err(SemanticError::TyMismatchIfElse {
                        when_true: when_true.ty,
                        when_false: when_false.ty,
                    });
                }

                Expression {
                    ty: when_true.ty,
                    kind: IfElse {
                        predicate: Box::new(predicate.visit_header(symbol_table)?),
                        when_true,
                        when_false,
                    },
                }
            }
        })
    }
}
