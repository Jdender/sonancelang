mod block;
mod expr_misc;
mod expression;
pub mod structure;

pub use {super::*, structure::*};

pub trait HeaderVisitor {
    type Output;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError>;
}

impl HeaderVisitor for from_ast::File {
    type Output = File;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(File {
            items: self
                .items
                .into_iter()
                .map(|i| i.visit_header(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl HeaderVisitor for from_ast::Function {
    type Output = Function;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let symbol_table = &mut symbol_table.fork();
        Ok(Function {
            scope: self.scope,
            name: self.name,
            ty: self.ty,
            symbol_id: self.symbol_id,
            params: self
                .params
                .into_iter()
                .map(|i| i.visit_header(symbol_table))
                .collect::<Result<_, _>>()?,
            body: self.body.visit_header(symbol_table)?,
        })
    }
}

impl HeaderVisitor for from_ast::Parameter {
    type Output = Parameter;

    fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(Parameter {
            symbol_id: symbol_table.set(self.name.clone(), Symbol::new_local(self.ty)),
            name: self.name,
            ty: self.ty,
        })
    }
}