use super::{FunctionHead, Identifier, Ty};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable<'a> {
    symbols: HashMap<Identifier, SymbolKind>,
    parent: Option<&'a SymbolTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn fork(&'a self) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn set(&mut self, key: Identifier, symbol: SymbolKind) {
        self.symbols.insert(key, symbol);
    }

    pub fn get(&self, key: &Identifier) -> Option<&SymbolKind> {
        match (self.symbols.get(key), self.parent) {
            (Some(value), _) => Some(value),
            (None, Some(parent)) => parent.get(key),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Local(LocalInfo),
    Func(FunctionHead),
}

impl SymbolKind {
    pub fn as_local(&self) -> Option<&LocalInfo> {
        match self {
            Self::Local(info) => Some(info),
            _ => None,
        }
    }

    pub fn as_func(&self) -> Option<&FunctionHead> {
        match self {
            Self::Func(head) => Some(head),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LocalInfo {
    id: LocalId,
    ty: Ty,
}

impl LocalInfo {
    pub fn new(ty: Ty) -> Self {
        Self {
            ty,
            id: LocalId::new(),
        }
    }

    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn id(&self) -> LocalId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalId(u32);

impl Default for LocalId {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalId {
    pub fn new() -> LocalId {
        use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

        static COUNTER: AtomicU32 = AtomicU32::new(0);

        COUNTER.fetch_add(1, Relaxed);
        LocalId(COUNTER.load(Relaxed))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}
