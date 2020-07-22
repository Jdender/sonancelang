use super::super::{ir, wasm};
use parity_wasm::builder::module;

pub fn wasm_pass(input: ir::WasmModule) -> wasm::Module {
    input.visit_ir(())
}

pub trait IrVisitor {
    type Argument;
    type Return;

    fn visit_ir(self, args: Self::Argument) -> Self::Return;
}

impl IrVisitor for ir::WasmModule {
    type Argument = ();
    type Return = wasm::Module;

    fn visit_ir(self, (): Self::Argument) -> Self::Return {
        let mut inst = Vec::new();
        let mut locals = Vec::new();

        for stmt in self.body {
            let mut stmt = stmt.visit_ir(locals);
            inst.append(&mut stmt.0);
            locals = stmt.1;
        }

        inst.push(wasm::Instruction::End);

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
            .with_locals(vec![wasm::Local::new(
                locals.len() as u32,
                wasm::ValueType::I32,
            )])
            .with_instructions(wasm::Instructions::new(inst))
            .build()
            .build()
            .build()
    }
}

impl IrVisitor for ir::Expression {
    type Argument = Vec<String>;
    type Return = (Vec<wasm::Instruction>, Vec<String>);

    fn visit_ir(self, mut locals: Self::Argument) -> <Self as IrVisitor>::Return {
        let mut inst = Vec::new();
        let locals = match self {
            Self::Const(num) => {
                inst.push(wasm::Instruction::I32Const(num));
                locals
            }
            Self::LocalGet(name) => {
                inst.push(wasm::Instruction::GetLocal(
                    locals
                        .iter()
                        .position(|n| *n == name)
                        .expect("Local not found") as u32,
                ));
                locals
            }
            Self::LocalDeclare(name, expr) => {
                locals.push(name);

                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.push(wasm::Instruction::SetLocal((locals.len() - 1) as u32));
                locals
            }
            Self::LocalSet(name, expr) => {
                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.push(wasm::Instruction::SetLocal(
                    locals
                        .iter()
                        .position(|n| *n == name)
                        .expect("Local not found") as u32,
                ));
                locals
            }
            Self::Block(block) => {
                let mut locals = locals;
                for stmt in block {
                    let (mut new_inst, new_locals) = stmt.visit_ir(locals);
                    inst.append(&mut new_inst);
                    locals = new_locals;
                }
                locals
            }
            Self::Return(expr) => {
                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.push(wasm::Instruction::Return);
                locals
            }
            Self::SimpleInfixCall(x, op, y) => {
                let (mut x, locals) = x.visit_ir(locals);
                inst.append(&mut x);

                let (mut y, locals) = y.visit_ir(locals);
                inst.append(&mut y);

                inst.push(op.visit_ir(()));
                locals
            }
            Self::Negate(expr) => {
                inst.push(wasm::Instruction::I32Const(0));

                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.push(wasm::Instruction::I32Sub);
                locals
            }
            Self::BooleanNot(expr) => {
                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.append(&mut vec![
                    wasm::Instruction::I32Eqz,
                    wasm::Instruction::If(wasm::BlockType::Value(wasm::ValueType::I32)),
                    wasm::Instruction::I32Const(1),
                    wasm::Instruction::Else,
                    wasm::Instruction::I32Const(0),
                    wasm::Instruction::End,
                ]);
                locals
            }
            Self::BooleanOr(x, y) => {
                let (mut x, locals) = x.visit_ir(locals);
                inst.append(&mut x);

                inst.append(&mut vec![
                    wasm::Instruction::I32Eqz,
                    wasm::Instruction::If(wasm::BlockType::Value(wasm::ValueType::I32)),
                ]);

                let (mut y, locals) = y.visit_ir(locals);
                inst.append(&mut y);

                inst.append(&mut vec![
                    wasm::Instruction::Else,
                    wasm::Instruction::I32Const(1),
                    wasm::Instruction::End,
                ]);
                locals
            }
            Self::BooleanAnd(x, y) => {
                let (mut x, locals) = x.visit_ir(locals);
                inst.append(&mut x);

                inst.append(&mut vec![
                    wasm::Instruction::I32Eqz,
                    wasm::Instruction::If(wasm::BlockType::Value(wasm::ValueType::I32)),
                    wasm::Instruction::I32Const(0),
                    wasm::Instruction::Else,
                ]);

                let (mut y, locals) = y.visit_ir(locals);
                inst.append(&mut y);

                inst.push(wasm::Instruction::End);
                locals
            }
        };
        (inst, locals)
    }
}

impl IrVisitor for ir::SimpleInfix {
    type Argument = ();
    type Return = wasm::Instruction;

    fn visit_ir(self, (): Self::Argument) -> Self::Return {
        match self {
            Self::Add => wasm::Instruction::I32Add,
            Self::Subtract => wasm::Instruction::I32Sub,
            Self::Multiply => wasm::Instruction::I32Mul,
            Self::Divide => wasm::Instruction::I32DivS,

            Self::Equal => wasm::Instruction::I32Eq,
            Self::NotEqual => wasm::Instruction::I32Ne,
            Self::GreaterThan => wasm::Instruction::I32GtS,
            Self::LessThan => wasm::Instruction::I32LtS,
            Self::GreaterOrEqual => wasm::Instruction::I32GeS,
            Self::LessOrEqual => wasm::Instruction::I32LeS,
        }
    }
}