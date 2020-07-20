use super::symbol_table::{ScopeIndex, SymbolTable};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct AstContext<T> {
    symbol_table: Rc<RefCell<SymbolTable>>,
    scope: ScopeIndex,
    pass_along: T,
}

impl<T> AstContext<T> {
    pub fn new(pass_along: T) -> Self {
        let symbol_table = Rc::new(RefCell::new(SymbolTable::new()));
        AstContext {
            scope: SymbolTable::root(),
            symbol_table,
            pass_along,
        }
    }

    pub fn with<R>(&self, pass_along: R) -> AstContext<R> {
        AstContext {
            pass_along,
            scope: self.scope,
            symbol_table: Rc::clone(&self.symbol_table),
        }
    }

    pub fn unit(&self) -> AstContext<()> {
        self.with(())
    }

    pub fn fork(&mut self) -> AstContext<()> {
        AstContext {
            scope: self.symbol_table.borrow_mut().fork(self.scope),
            symbol_table: Rc::clone(&self.symbol_table),
            pass_along: (),
        }
    }

    pub fn symbols(&self) -> &Rc<RefCell<SymbolTable>> {
        &self.symbol_table
    }

    pub fn scope(&self) -> ScopeIndex {
        self.scope
    }
}
