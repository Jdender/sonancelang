use super::super::*;

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub scope: Scope,
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub ty: Ty,
    pub body: ast::Block,
    pub symbol_id: SymbolId,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub ty: Ty,
}
