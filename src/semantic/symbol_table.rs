use super::{Identifier, Ty};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable<'a> {
    locals: HashMap<Identifier, SymbolInfo>,
    parent: Option<&'a SymbolTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            locals: HashMap::new(),
            parent: None,
        }
    }

    pub fn fork(&'a self) -> Self {
        SymbolTable {
            locals: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn set(&mut self, key: Identifier, symbol: SymbolInfo) -> SymbolInfo {
        self.locals.insert(key, symbol);
        symbol
    }

    pub fn get(&self, key: &Identifier) -> Option<SymbolInfo> {
        match (self.locals.get(key), self.parent) {
            (Some(value), _) => Some(*value),
            (None, Some(parent)) => parent.get(key),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolInfo {
    id: SymbolId,
    ty: Ty,
}

impl SymbolInfo {
    pub fn new(ty: Ty) -> Self {
        Self {
            ty,
            id: SymbolId::new(),
        }
    }

    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn id(&self) -> SymbolId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolId(u32);

impl Default for SymbolId {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolId {
    pub fn new() -> SymbolId {
        use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

        static COUNTER: AtomicU32 = AtomicU32::new(0);

        COUNTER.fetch_add(1, Relaxed);
        SymbolId(COUNTER.load(Relaxed))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}
