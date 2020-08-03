use super::Ty;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable<'a> {
    members: HashMap<String, SymbolInfo>,
    parent: Option<&'a SymbolTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            members: HashMap::new(),
            parent: None,
        }
    }

    pub fn fork(&'a self) -> Self {
        SymbolTable {
            members: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn set(&mut self, key: String, symbol: SymbolInfo) -> SymbolInfo {
        self.members.insert(key, symbol);
        symbol
    }

    pub fn get(&self, key: &str) -> Option<SymbolInfo> {
        match (self.members.get(key), self.parent) {
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
