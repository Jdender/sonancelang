use super::ast;

#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Public,
    Local,
}

impl ast::Scope {
    pub fn visit_common(self) -> Scope {
        use Scope::*;
        match self {
            Self::Local => Local,
            Self::Public => Public,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ty {
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
}

impl ast::Ty {
    pub fn visit_common(self) -> Ty {
        use Ty::*;

        match self {
            Self::I8 => I8,
            Self::I16 => I16,
            Self::I32 => I32,
            Self::I64 => I64,
            Self::ISize => ISize,
            Self::U8 => U8,
            Self::U16 => U16,
            Self::U32 => U32,
            Self::U64 => U64,
            Self::USize => USize,
            Self::F32 => F32,
            Self::F64 => F64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(ident: String) -> Self {
        Identifier(ident)
    }

    pub fn take(self) -> String {
        self.0
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }
}

impl ast::Identifier {
    pub fn visit_common(self) -> Identifier {
        Identifier::new(self.take())
    }
}
