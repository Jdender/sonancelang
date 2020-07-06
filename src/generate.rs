use crate::parse::*;
use parity_wasm::{
    builder::ModuleBuilder,
    elements::{Instruction, Instructions, Module},
};

pub trait AstVisitor {
    type Argument;
    type Return;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return;
}

impl AstVisitor for File {
    type Argument = ();
    type Return = Module;

    fn visit_ast(&self, (): Self::Argument) -> Self::Return {
        ModuleBuilder::new()
            .function()
            .signature()
            .return_type()
            .i32()
            .build()
            .body()
            .with_instructions(Instructions::new(vec![
                Instruction::I32Const(self.0),
                Instruction::End,
            ]))
            .build()
            .build()
            .build()
    }
}
