use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolInfo;

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

    pub fn set(&mut self, key: String, value: SymbolInfo) {
        self.members.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&SymbolInfo> {
        match (self.members.get(key), self.parent) {
            (Some(value), _) => Some(value),
            (None, Some(parent)) => parent.get(key),
            _ => None,
        }
    }
}
