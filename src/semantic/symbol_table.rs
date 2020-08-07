use {
    super::{FunctionHead, Identifier, Ty},
    std::collections::HashMap,
};

#[derive(Debug, Clone, Default)]
pub struct SymbolTable<'a> {
    symbols: HashMap<Identifier, Symbol>,
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

    pub fn set(&mut self, key: Identifier, symbol: Symbol) -> SymbolId {
        let id = symbol.id();
        self.symbols.insert(key, symbol);
        id
    }

    pub fn get(&self, key: &Identifier) -> Option<&Symbol> {
        match (self.symbols.get(key), self.parent) {
            (Some(value), _) => Some(value),
            (None, Some(parent)) => parent.get(key),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    id: SymbolId,
    kind: SymbolKind,
}

impl Symbol {
    pub fn new_local(ty: Ty) -> Self {
        Self {
            id: SymbolId::new(),
            kind: SymbolKind::Local(LocalInfo { ty }),
        }
    }

    pub fn new_func(head: FunctionHead) -> Self {
        Self {
            id: SymbolId::new(),
            kind: SymbolKind::Func(head),
        }
    }

    pub fn id(&self) -> SymbolId {
        self.id
    }

    pub fn as_local(&self) -> Option<&LocalInfo> {
        match &self.kind {
            SymbolKind::Local(info) => Some(info),
            _ => None,
        }
    }

    pub fn as_func(&self) -> Option<&FunctionHead> {
        match &self.kind {
            SymbolKind::Func(head) => Some(head),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum SymbolKind {
    Local(LocalInfo),
    Func(FunctionHead),
}

#[derive(Debug, Clone, Copy)]
pub struct LocalInfo {
    pub ty: Ty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(u32);

impl Default for SymbolId {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolId {
    fn new() -> SymbolId {
        use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

        static COUNTER: AtomicU32 = AtomicU32::new(0);

        COUNTER.fetch_add(1, Relaxed);
        SymbolId(COUNTER.load(Relaxed))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}
