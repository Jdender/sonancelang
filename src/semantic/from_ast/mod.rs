mod block;
mod expr_misc;
mod expression;

pub use super::{
    super::{ast, semantic},
    SemanticError, Symbol, SymbolId, SymbolTable,
};

pub trait AstVisitor {
    type Output;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError>;
}

impl AstVisitor for ast::File {
    type Output = semantic::File;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let symbol_table = &mut symbol_table.fork();

        Ok(semantic::File {
            items: self
                .items
                .into_iter()
                .map(|item| {
                    // Convert the func head first
                    let partial = item.visit_ast(symbol_table)?;

                    // Add func head to symbol table
                    let symbol_id = symbol_table.set(
                        partial.head.name.clone(),
                        Symbol::new_func(partial.head.clone()),
                    );

                    Ok((partial, symbol_id))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|i| i.visit_ast(symbol_table))
                .collect::<Result<_, _>>()?,
        })
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

impl AstVisitor for (PartialFunction, SymbolId) {
    type Output = semantic::Function;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let (func, symbol_id) = self;

        let body = func.body.visit_ast(symbol_table)?;

        // Assert types match
        if func.head.ty != body.ty {
            return Err(SemanticError::TyMismatchReturn {
                expected: func.head.ty,
                found: body.ty,
            });
        }

        Ok(semantic::Function {
            head: func.head,
            body,
            symbol_id,
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
