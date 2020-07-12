use super::structure::*;
use parity_wasm::{
    builder::module,
    elements::{BlockType, Instruction, Instructions, Local, Module, ValueType},
};

pub trait LowLevelVisitor {
    type Argument;
    type Return;

    fn visit_lowlevel(self, args: Self::Argument) -> Self::Return;
}

impl LowLevelVisitor for WasmModule {
    type Argument = ();
    type Return = Module;

    fn visit_lowlevel(self, (): Self::Argument) -> Self::Return {
        let mut inst = Vec::new();
        let mut locals = Vec::new();

        for stmt in self.body {
            let mut stmt = stmt.visit_lowlevel(locals);
            inst.append(&mut stmt.0);
            locals = stmt.1;
        }

        inst.push(Instruction::End);

        module()
            .export()
            .field(&self.name)
            .internal()
            .func(0)
            .build()
            .function()
            .signature()
            .return_type()
            .i32()
            .build()
            .body()
            .with_locals(vec![Local::new(locals.len() as u32, ValueType::I32)])
            .with_instructions(Instructions::new(inst))
            .build()
            .build()
            .build()
    }
}

impl LowLevelVisitor for WasmExpression {
    type Argument = Vec<String>;
    type Return = (Vec<Instruction>, Vec<String>);

    fn visit_lowlevel(self, mut locals: Self::Argument) -> <Self as LowLevelVisitor>::Return {
        let mut inst = Vec::new();
        let locals = match self {
            WasmExpression::Const(num) => {
                inst.push(Instruction::I32Const(num));
                locals
            }
            WasmExpression::LocalGet(name) => {
                inst.push(Instruction::GetLocal(
                    locals
                        .iter()
                        .position(|n| *n == name)
                        .expect("Local not found") as u32,
                ));
                locals
            }
            WasmExpression::LocalDeclare(name, expr) => {
                locals.push(name);

                let (mut expr, locals) = expr.visit_lowlevel(locals);
                inst.append(&mut expr);

                inst.push(Instruction::SetLocal((locals.len() - 1) as u32));
                locals
            }
            WasmExpression::LocalSet(name, expr) => {
                let (mut expr, locals) = expr.visit_lowlevel(locals);
                inst.append(&mut expr);

                inst.push(Instruction::SetLocal(
                    locals
                        .iter()
                        .position(|n| *n == name)
                        .expect("Local not found") as u32,
                ));
                locals
            }
            WasmExpression::Return(expr) => {
                let (mut expr, locals) = expr.visit_lowlevel(locals);
                inst.append(&mut expr);

                inst.push(Instruction::Return);
                locals
            }
            WasmExpression::SimpleInfixCall(x, op, y) => {
                let (mut x, locals) = x.visit_lowlevel(locals);
                inst.append(&mut x);

                let (mut y, locals) = y.visit_lowlevel(locals);
                inst.append(&mut y);

                inst.push(op.visit_lowlevel(()));
                locals
            }
            WasmExpression::Negate(expr) => {
                inst.push(Instruction::I32Const(0));

                let (mut expr, locals) = expr.visit_lowlevel(locals);
                inst.append(&mut expr);

                inst.push(Instruction::I32Sub);
                locals
            }
            WasmExpression::BooleanNot(expr) => {
                let (mut expr, locals) = expr.visit_lowlevel(locals);
                inst.append(&mut expr);

                inst.append(&mut vec![
                    Instruction::I32Eqz,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                    Instruction::I32Const(1),
                    Instruction::Else,
                    Instruction::I32Const(0),
                    Instruction::End,
                ]);
                locals
            }
            WasmExpression::BooleanOr(x, y) => {
                let (mut x, locals) = x.visit_lowlevel(locals);
                inst.append(&mut x);

                inst.append(&mut vec![
                    Instruction::I32Eqz,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                ]);

                let (mut y, locals) = y.visit_lowlevel(locals);
                inst.append(&mut y);

                inst.append(&mut vec![
                    Instruction::Else,
                    Instruction::I32Const(1),
                    Instruction::End,
                ]);
                locals
            }
            WasmExpression::BooleanAnd(x, y) => {
                let (mut x, locals) = x.visit_lowlevel(locals);
                inst.append(&mut x);

                inst.append(&mut vec![
                    Instruction::I32Eqz,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                    Instruction::I32Const(0),
                    Instruction::Else,
                ]);

                let (mut y, locals) = y.visit_lowlevel(locals);
                inst.append(&mut y);

                inst.push(Instruction::End);
                locals
            }
        };
        (inst, locals)
    }
}

impl LowLevelVisitor for WasmSimpleInfix {
    type Argument = ();
    type Return = Instruction;

    fn visit_lowlevel(self, (): Self::Argument) -> Self::Return {
        match self {
            WasmSimpleInfix::Add => Instruction::I32Add,
            WasmSimpleInfix::Subtract => Instruction::I32Sub,
            WasmSimpleInfix::Multiply => Instruction::I32Mul,
            WasmSimpleInfix::Divide => Instruction::I32DivS,

            WasmSimpleInfix::Equal => Instruction::I32Eq,
            WasmSimpleInfix::NotEqual => Instruction::I32Ne,
            WasmSimpleInfix::GreaterThan => Instruction::I32GtS,
            WasmSimpleInfix::LessThan => Instruction::I32LtS,
            WasmSimpleInfix::GreaterOrEqual => Instruction::I32GeS,
            WasmSimpleInfix::LessOrEqual => Instruction::I32LeS,
        }
    }
}
