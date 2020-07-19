pub mod from_semantic;
pub mod structure;

pub use from_semantic::ir_pass;
pub use structure::*;

#[derive(Debug, Clone, PartialEq)]
pub enum IrError {
    VariableNotDeclared(String),
}

pub type IrResult<T> = Result<T, IrError>;
