use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct ScopeIndex(usize);

#[derive(Debug, Clone, Default)]
struct Scope {
    parent: Option<ScopeIndex>,
    children: Vec<ScopeIndex>,
    symbols: HashMap<String, SymbolInfo>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![Scope::default()],
        }
    }

    pub fn root() -> ScopeIndex {
        ScopeIndex(0)
    }

    fn get_scope(&self, scope: ScopeIndex) -> &Scope {
        self.scopes
            .get(scope.0)
            .expect("Shouldn't ask for a invalid scope")
    }

    fn get_scope_mut(&mut self, scope: ScopeIndex) -> &mut Scope {
        self.scopes
            .get_mut(scope.0)
            .expect("Shouldn't ask for a invalid scope")
    }

    pub fn set(&mut self, scope: ScopeIndex, key: String, value: SymbolInfo) {
        self.get_scope_mut(scope).symbols.insert(key, value);
    }

    pub fn get(&self, scope: ScopeIndex, key: &str) -> Option<&SymbolInfo> {
        let scope = self.get_scope(scope);
        match (scope.symbols.get(key), scope.parent) {
            (Some(value), _) => Some(value),
            (None, Some(parent)) => self.get(parent, key),
            _ => None,
        }
    }

    pub fn fork(&mut self, parent: ScopeIndex) -> ScopeIndex {
        let id = ScopeIndex(self.scopes.len());

        self.scopes.push(Scope {
            parent: Some(parent),
            symbols: HashMap::new(),
            children: Vec::new(),
        });
        self.get_scope_mut(parent).children.push(id);

        id
    }
}
