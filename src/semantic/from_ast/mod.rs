pub mod structure;

pub use {super::*, structure::*};

pub trait AstVisitor {
    type Output;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError>;
}

impl AstVisitor for ast::File {
    type Output = File;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(File {
            items: self
                .items
                .into_iter()
                .map(|i| i.visit_ast(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl AstVisitor for ast::Item {
    type Output = Item;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        use Item::*;
        Ok(match self {
            Self::Function(func) => Function(func.visit_ast(symbol_table)?),
        })
    }
}

impl AstVisitor for ast::Function {
    type Output = Function;

    fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        let name = self.name.visit_common();
        let ty = self.ty.visit_common();

        let params = self
            .params
            .into_iter()
            .map(|a| a.visit_ast(symbol_table))
            .collect::<Result<Vec<_>, _>>()?;

        let symbol_id = symbol_table.set(
            name.clone(),
            Symbol::new_func(ty, params.iter().map(|p| p.ty).collect()),
        );

        Ok(Function {
            ty,
            params,
            name,
            scope: self.scope.visit_common(),
            body: self.body,
            symbol_id,
        })
    }
}

impl AstVisitor for ast::Parameter {
    type Output = Parameter;

    fn visit_ast(self, _: &mut SymbolTable) -> Result<Self::Output, SemanticError> {
        Ok(Parameter {
            name: self.name.visit_common(),
            ty: self.ty.visit_common(),
        })
    }
}
