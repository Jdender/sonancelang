use super::{ast, semantic, AstVisitor, SemanticError, Symbol, SymbolTable};

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
                let symbol_id = symbol_table.set(place.clone(), Symbol::new_local(ty));

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