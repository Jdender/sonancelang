use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Number(f32),
    String(String),
}

impl Value {
    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Nil => "Nil",
            Self::Number(_) => "Number",
            Self::String(_) => "String",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Number(num) => write!(f, "{}", num),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}

pub trait UnwrapValue {
    fn unwrap_nil(self) -> Option<()>;
    fn unwrap_number(self) -> Option<f32>;
    fn unwrap_string(self) -> Option<String>;
}

impl UnwrapValue for Value {
    fn unwrap_nil(self) -> Option<()> {
        match self {
            Self::Nil => Some(()),
            _ => None,
        }
    }

    fn unwrap_number(self) -> Option<f32> {
        match self {
            Self::Number(value) => Some(value),
            _ => None,
        }
    }

    fn unwrap_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl UnwrapValue for Option<Value> {
    fn unwrap_nil(self) -> Option<()> {
        self?.unwrap_nil()
    }

    fn unwrap_number(self) -> Option<f32> {
        self?.unwrap_number()
    }

    fn unwrap_string(self) -> Option<String> {
        self?.unwrap_string()
    }
}
