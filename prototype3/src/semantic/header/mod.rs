pub mod structure;

pub use {super::*, structure::*};

impl ast::File {
    pub fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<File, SemanticError> {
        Ok(File {
            items: self
                .items
                .into_iter()
                .map(|i| i.visit_ast(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl ast::Item {
    pub fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Item, SemanticError> {
        use Item::*;
        Ok(match self {
            Self::Declare(declare) => Declare(declare.visit_ast(symbol_table)?),
            Self::Function(func) => Function(func.visit_ast(symbol_table)?),
        })
    }
}

impl ast::DeclareBlock {
    pub fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<DeclareBlock, SemanticError> {
        Ok(DeclareBlock {
            functions: self
                .functions
                .into_iter()
                .map(|f| f.visit_ast(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl ast::DeclareFunction {
    pub fn visit_ast(
        self,
        symbol_table: &mut SymbolTable,
    ) -> Result<DeclareFunction, SemanticError> {
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

        Ok(DeclareFunction {
            ty,
            params,
            name,
            symbol_id,
        })
    }
}

impl ast::Function {
    pub fn visit_ast(self, symbol_table: &mut SymbolTable) -> Result<Function, SemanticError> {
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

impl ast::Parameter {
    pub fn visit_ast(self, _: &mut SymbolTable) -> Result<Parameter, SemanticError> {
        Ok(Parameter {
            name: self.name.visit_common(),
            ty: self.ty.visit_common(),
        })
    }
}
