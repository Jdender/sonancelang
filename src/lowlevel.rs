use parity_wasm::{
    builder::ModuleBuilder,
    elements::{BlockType, Instruction, Instructions, Module, ValueType},
};

pub trait LowLevelVisitor {
    type Argument;
    type Return;

    fn visit_lowlevel(&self, args: Self::Argument) -> Self::Return;
}

#[derive(Debug, Clone, PartialEq)]
pub struct WasmModule(pub WasmExpression);

impl LowLevelVisitor for WasmModule {
    type Argument = ();
    type Return = Module;

    fn visit_lowlevel(&self, (): Self::Argument) -> Self::Return {
        let mut inst = self.0.visit_lowlevel(());
        inst.push(Instruction::End);

        ModuleBuilder::new()
            .function()
            .signature()
            .return_type()
            .i32()
            .build()
            .body()
            .with_instructions(Instructions::new(inst))
            .build()
            .build()
            .build()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WasmExpression {
    Const(i32),
    Negate(Box<WasmExpression>),
    BooleanNot(Box<WasmExpression>),
}

impl LowLevelVisitor for WasmExpression {
    type Argument = ();
    type Return = Vec<Instruction>;

    fn visit_lowlevel(&self, (): Self::Argument) -> Self::Return {
        match self {
            WasmExpression::Const(num) => vec![Instruction::I32Const(*num)],
            WasmExpression::Negate(expr) => {
                let mut inst = Vec::new();
                inst.push(Instruction::I32Const(0));
                inst.append(&mut expr.visit_lowlevel(()));
                inst.push(Instruction::I32Sub);
                inst
            }
            WasmExpression::BooleanNot(expr) => {
                let mut inst = Vec::new();
                inst.append(&mut expr.visit_lowlevel(()));
                inst.append(&mut vec![
                    Instruction::I32Const(0),
                    Instruction::I32Eq,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                    Instruction::I32Const(1),
                    Instruction::Else,
                    Instruction::I32Const(0),
                ]);
                inst
            }
        }
    }
}
