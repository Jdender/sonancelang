use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct SymbolId(usize);

pub fn new_symbol_id() -> SymbolId {
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    COUNTER.fetch_add(1, Relaxed);
    SymbolId(COUNTER.load(Relaxed))
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable<'a> {
    members: HashMap<String, SymbolId>,
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

    pub fn set(&mut self, key: String) {
        self.members.insert(key, new_symbol_id());
    }

    pub fn get(&self, key: &str) -> Option<SymbolId> {
        match (self.members.get(key), self.parent) {
            (Some(value), _) => Some(*value),
            (None, Some(parent)) => parent.get(key),
            _ => None,
        }
    }
}
