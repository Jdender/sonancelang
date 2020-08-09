use super::super::*;

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Declare(DeclareBlock),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct DeclareBlock {
    pub functions: Vec<DeclareFunction>,
}

#[derive(Debug, Clone)]
pub struct DeclareFunction {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub ty: Ty,
    pub symbol_id: SymbolId,
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
