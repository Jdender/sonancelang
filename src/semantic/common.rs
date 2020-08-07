use super::ast;

pub trait CommonVisitor {
    type Output;

    fn visit_common(self) -> Self::Output;
}
#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Export,
    Local,
}

impl CommonVisitor for ast::Scope {
    type Output = Scope;

    fn visit_common(self) -> Self::Output {
        use Scope::*;
        match self {
            Self::Local => Local,
            Self::Export => Export,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ty {
    I32,
    F32,
}

impl CommonVisitor for ast::Ty {
    type Output = Ty;

    fn visit_common(self) -> Self::Output {
        use Ty::*;

        match self {
            Self::I32 => I32,
            Self::F32 => F32,
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
