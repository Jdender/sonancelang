mod block;
mod expr_misc;
mod expression;
pub mod structure;

pub use super::{
    super::{ast, semantic},
    SemanticError, Symbol, SymbolId, SymbolTable,
};

pub trait HeaderVisitor {
    type Output;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError>;
}

impl HeaderVisitor for ast::File {
    type Output = semantic::File;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let mut symbol_table = symbol_table.fork();

        Ok(semantic::File {
            items: self
                .items
                .into_iter()
                .map(|item| {
                    // Convert the func head first
                    let partial = item.visit_header(&mut symbol_table)?;

                    // Add func head to symbol table
                    let symbol_id = symbol_table.set(
                        partial.head.name.clone(),
                        Symbol::new_func(partial.head.clone()),
                    );

                    Ok((partial, symbol_id))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|i| i.visit_header(&mut symbol_table.fork()))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PartialFunction {
    pub head: semantic::FunctionHead,
    pub body: ast::Block,
}

impl HeaderVisitor for ast::Function {
    type Output = PartialFunction;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(PartialFunction {
            head: semantic::FunctionHead {
                scope: self.scope.visit_header(symbol_table)?,
                name: self.name.visit_header(symbol_table)?,
                params: self
                    .params
                    .into_iter()
                    .map(|a| a.visit_header(symbol_table))
                    .collect::<Result<_, _>>()?,
                ty: self.ty.visit_header(symbol_table)?,
            },
            body: self.body,
        })
    }
}

impl HeaderVisitor for ast::Parameter {
    type Output = semantic::Parameter;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let name = self.name.visit_header(symbol_table)?;
        let ty = self.ty.visit_header(symbol_table)?;
        let symbol_id = symbol_table.set(name.clone(), Symbol::new_local(ty));
        Ok(semantic::Parameter {
            name,
            ty,
            symbol_id,
        })
    }
}

impl HeaderVisitor for (PartialFunction, SymbolId) {
    type Output = semantic::Function;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let (func, symbol_id) = self;
        let symbol_table = &mut symbol_table.fork();

        let body = func.body.visit_header(symbol_table)?;

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

impl HeaderVisitor for ast::Scope {
    type Output = semantic::Scope;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::Scope::*;
        Ok(match self {
            Self::Local => Local,
            Self::Export => Export,
        })
    }
}

impl HeaderVisitor for ast::Identifier {
    type Output = semantic::Identifier;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(semantic::Identifier::new(self.as_string().clone()))
    }
}

impl HeaderVisitor for ast::Ty {
    type Output = semantic::Ty;

    fn visit_header(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use semantic::Ty::*;

        Ok(match self {
            Self::I32 => I32,
            Self::F32 => F32,
        })
    }
}
