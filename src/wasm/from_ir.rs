use crate::{ir, semantic::SymbolId, wasm};
use parity_wasm::builder::module;

pub fn wasm_pass(input: ir::WasmModule) -> wasm::Module {
    input.visit_ir(())
}

pub trait IrVisitor {
    type Argument;
    type Return;

    fn visit_ir(self, args: Self::Argument) -> Self::Return;
}

type Locals = Vec<SymbolId>;

impl IrVisitor for ir::WasmModule {
    type Argument = ();
    type Return = wasm::Module;

    fn visit_ir(self, (): Self::Argument) -> Self::Return {
        let (mut inst, locals) = self.body.visit_ir(Vec::new());

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

impl IrVisitor for ir::Block {
    type Argument = Locals;
    type Return = (Vec<wasm::Instruction>, Locals);

    fn visit_ir(self, locals: Self::Argument) -> Self::Return {
        let mut inst = Vec::new();
        let mut locals = locals;

        for stmt in self.body {
            let (mut new_inst, new_locals) = stmt.visit_ir(locals);
            inst.append(&mut new_inst);
            inst.push(wasm::Instruction::Drop);
            locals = new_locals;
        }

        let (mut expr, locals) = self.trailing.visit_ir(locals);
        inst.append(&mut expr);

        (inst, locals)
    }
}

impl IrVisitor for ir::Expression {
    type Argument = Locals;
    type Return = (Vec<wasm::Instruction>, Locals);

    fn visit_ir(self, mut locals: Self::Argument) -> <Self as IrVisitor>::Return {
        let mut inst = Vec::new();
        let locals = match self {
            Self::Const(num) => {
                inst.push(wasm::Instruction::I32Const(num));
                locals
            }

            Self::LocalGet(symbol_id) => {
                inst.push(wasm::Instruction::GetLocal(
                    locals
                        .iter()
                        .position(|s| *s == symbol_id)
                        .expect("Local not found") as u32,
                ));
                locals
            }

            Self::LocalDeclare(symbol_id, expr) => {
                locals.push(symbol_id);

                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.append(&mut vec![
                    wasm::Instruction::SetLocal((locals.len() - 1) as u32),
                    wasm::Instruction::I32Const(0),
                ]);
                locals
            }

            Self::LocalSet(symbol_id, expr) => {
                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.append(&mut vec![
                    wasm::Instruction::SetLocal(
                        locals
                            .iter()
                            .position(|s| *s == symbol_id)
                            .expect("Local not found") as u32,
                    ),
                    wasm::Instruction::I32Const(0),
                ]);
                locals
            }

            Self::Block(block) => {
                let (mut block, locals) = block.visit_ir(locals);
                inst.append(&mut block);
                locals
            }

            Self::Return(expr) => {
                let (mut expr, locals) = expr.visit_ir(locals);
                inst.append(&mut expr);

                inst.append(&mut vec![
                    wasm::Instruction::Return,
                    wasm::Instruction::I32Const(0),
                ]);
                locals
            }

            Self::SimpleInfixCall {
                operator,
                x_operand,
                y_operand,
            } => {
                let (mut x_operand, locals) = x_operand.visit_ir(locals);
                inst.append(&mut x_operand);

                let (mut y_operand, locals) = y_operand.visit_ir(locals);
                inst.append(&mut y_operand);

                inst.push(operator.visit_ir(()));
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

            Self::Conditional {
                predicate,
                when_true,
                when_false,
            } => {
                let (mut predicate, locals) = predicate.visit_ir(locals);

                inst.append(&mut predicate);

                inst.append(&mut vec![
                    wasm::Instruction::I32Eqz,
                    wasm::Instruction::If(wasm::BlockType::Value(wasm::ValueType::I32)),
                ]);

                let (mut when_false, locals) = when_false.visit_ir(locals);
                inst.append(&mut when_false);

                inst.push(wasm::Instruction::Else);

                let (mut when_true, locals) = when_true.visit_ir(locals);
                inst.append(&mut when_true);

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
