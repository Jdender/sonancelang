use super::ast;

pub trait CommonVisitor {
    type Output;

    fn visit_common(self) -> Self::Output;
}
#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Public,
    Local,
}

impl CommonVisitor for ast::Scope {
    type Output = Scope;

    fn visit_common(self) -> Self::Output {
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

impl CommonVisitor for ast::Ty {
    type Output = Ty;

    fn visit_common(self) -> Self::Output {
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

impl CommonVisitor for ast::Identifier {
    type Output = Identifier;

    fn visit_common(self) -> Self::Output {
        Identifier::new(self.take())
    }
}
