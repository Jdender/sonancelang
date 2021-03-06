use super::*;

impl ast::Block {
    pub fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Block, SemanticError> {
        let symbol_table = &mut symbol_table.fork();

        let body = self
            .body
            .into_iter()
            .map(|s| s.visit_header(symbol_table))
            .collect::<Result<_, _>>()?;

        let trailing = Box::new(
            self.trailing
                .map(|t| *t)
                .unwrap_or(ast::Expression::Literal(ast::Literal::I32(0)))
                .visit_header(symbol_table)?,
        );

        // Blocks return their trailing expr, same goes for types
        Ok(Block {
            body,
            ty: trailing.ty,
            trailing,
        })
    }
}

impl ast::Statement {
    pub fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Statement, SemanticError> {
        Ok(match self {
            ast::Statement::LetBinding { place, value, ty } => {
                let place = place.visit_common();
                let value = value.visit_header(symbol_table)?;

                // Infer type if not declared
                let ty = if let Some(ty) = ty {
                    ty.visit_common()
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
                let symbol_id = symbol_table.set(place.clone(), Symbol::new_local(ty));

                Statement::LetBinding {
                    place,
                    symbol_id,
                    ty,
                    value,
                }
            }
            ast::Statement::SideEffect(expr) => {
                Statement::SideEffect(expr.visit_header(symbol_table)?)
            }
        })
    }
}
