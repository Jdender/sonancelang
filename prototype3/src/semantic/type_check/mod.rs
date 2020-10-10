mod block;
mod expr_misc;
mod expression;
pub mod structure;

pub use {super::*, structure::*};

impl header::File {
    pub fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<File, SemanticError> {
        Ok(File {
            items: self
                .items
                .into_iter()
                .map(|i| i.visit_header(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl header::Item {
    pub fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Item, SemanticError> {
        use Item::*;
        Ok(match self {
            Self::Declare(declare) => Declare(declare.visit_header(symbol_table)?),
            Self::Function(func) => Function(func.visit_header(symbol_table)?),
        })
    }
}

impl header::DeclareBlock {
    pub fn visit_header(
        self,
        symbol_table: &mut SymbolTable,
    ) -> Result<DeclareBlock, SemanticError> {
        Ok(DeclareBlock {
            functions: self
                .functions
                .into_iter()
                .map(|f| f.visit_header(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl header::DeclareFunction {
    pub fn visit_header(
        self,
        symbol_table: &mut SymbolTable,
    ) -> Result<DeclareFunction, SemanticError> {
        let symbol_table = &mut symbol_table.fork();
        Ok(DeclareFunction {
            name: self.name,
            ty: self.ty,
            symbol_id: self.symbol_id,
            params: self
                .params
                .into_iter()
                .map(|i| i.visit_header(symbol_table))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl header::Function {
    pub fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Function, SemanticError> {
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

impl header::Parameter {
    pub fn visit_header(self, symbol_table: &mut SymbolTable) -> Result<Parameter, SemanticError> {
        Ok(Parameter {
            symbol_id: symbol_table.set(self.name.clone(), Symbol::new_local(self.ty)),
            name: self.name,
            ty: self.ty,
        })
    }
}
