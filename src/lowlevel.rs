use crate::parse::InfixOp;
use parity_wasm::{
    builder::module,
    elements::{BlockType, Instruction, Instructions, Module, ValueType},
};

pub trait LowLevelVisitor {
    type Argument;
    type Return;

    fn visit_lowlevel(&self, args: Self::Argument) -> Self::Return;
}

#[derive(Debug, Clone, PartialEq)]
pub struct WasmModule(pub String, pub WasmExpression);

impl LowLevelVisitor for WasmModule {
    type Argument = ();
    type Return = Module;

    fn visit_lowlevel(&self, (): Self::Argument) -> Self::Return {
        let mut inst = self.1.visit_lowlevel(());
        inst.push(Instruction::End);

        module()
            .export()
            .field(&self.0)
            .internal()
            .func(0)
            .build()
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
    SimpleInfixCall(Box<WasmExpression>, InfixOp, Box<WasmExpression>),
}

impl LowLevelVisitor for WasmExpression {
    type Argument = ();
    type Return = Vec<Instruction>;

    fn visit_lowlevel(&self, (): Self::Argument) -> Self::Return {
        let mut inst = Vec::new();
        match self {
            WasmExpression::Const(num) => {
                inst.push(Instruction::I32Const(*num));
            }
            WasmExpression::Negate(expr) => {
                inst.push(Instruction::I32Const(0));
                inst.append(&mut expr.visit_lowlevel(()));
                inst.push(Instruction::I32Sub);
            }
            WasmExpression::BooleanNot(expr) => {
                inst.append(&mut expr.visit_lowlevel(()));
                inst.append(&mut vec![
                    Instruction::I32Const(0),
                    Instruction::I32Eq,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                    Instruction::I32Const(1),
                    Instruction::Else,
                    Instruction::I32Const(0),
                    Instruction::End,
                ]);
            }
            WasmExpression::SimpleInfixCall(x, op, y) => {
                inst.append(&mut x.visit_lowlevel(()));
                inst.append(&mut y.visit_lowlevel(()));
                inst.push(op.visit_lowlevel(()));
            }
        }
        inst
    }
}

impl LowLevelVisitor for InfixOp {
    type Argument = ();
    type Return = Instruction;

    fn visit_lowlevel(&self, (): Self::Argument) -> Self::Return {
        match self {
            InfixOp::Add => Instruction::I32Add,
            InfixOp::Subtract => Instruction::I32Sub,
            InfixOp::Multiply => Instruction::I32Mul,
            InfixOp::Divide => Instruction::I32DivS,
            InfixOp::Equal => Instruction::I32Eq,
            InfixOp::NotEqual => Instruction::I32Ne,
            InfixOp::GreaterThan => Instruction::I32GtS,
            InfixOp::LessThan => Instruction::I32LtS,
            InfixOp::GreaterOrEqual => Instruction::I32GeS,
            InfixOp::LessOrEqual => Instruction::I32LeS,
        }
    }
}
