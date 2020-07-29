#[derive(Debug, Clone)]
pub struct File {
    pub name: Identifier,
    pub number: i32,
}

#[derive(Debug, Clone)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(ident: String) -> Self {
        Identifier(ident)
    }
    pub fn as_string(&self) -> &String {
        &self.0
    }
}
