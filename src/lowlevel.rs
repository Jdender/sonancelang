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
    Return(Box<WasmExpression>),
    SimpleInfixCall(Box<WasmExpression>, WasmSimpleInfix, Box<WasmExpression>),
    Negate(Box<WasmExpression>),
    BooleanNot(Box<WasmExpression>),
    BooleanOr(Box<WasmExpression>, Box<WasmExpression>),
    BooleanAnd(Box<WasmExpression>, Box<WasmExpression>),
}

impl LowLevelVisitor for WasmExpression {
    type Argument = ();
    type Return = Vec<Instruction>;

    fn visit_lowlevel(&self, (): Self::Argument) -> <Self as LowLevelVisitor>::Return {
        let mut inst = Vec::new();
        match self {
            WasmExpression::Const(num) => {
                inst.push(Instruction::I32Const(*num));
            }
            WasmExpression::Return(expr) => {
                inst.append(&mut expr.visit_lowlevel(()));
                inst.push(Instruction::Return);
            }
            WasmExpression::SimpleInfixCall(x, op, y) => {
                inst.append(&mut x.visit_lowlevel(()));
                inst.append(&mut y.visit_lowlevel(()));
                inst.push(op.visit_lowlevel(()));
            }
            WasmExpression::Negate(expr) => {
                inst.push(Instruction::I32Const(0));
                inst.append(&mut expr.visit_lowlevel(()));
                inst.push(Instruction::I32Sub);
            }
            WasmExpression::BooleanNot(expr) => {
                inst.append(&mut expr.visit_lowlevel(()));
                inst.append(&mut vec![
                    Instruction::I32Eqz,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                    Instruction::I32Const(1),
                    Instruction::Else,
                    Instruction::I32Const(0),
                    Instruction::End,
                ]);
            }
            WasmExpression::BooleanOr(x, y) => {
                inst.append(&mut x.visit_lowlevel(()));
                inst.append(&mut vec![
                    Instruction::I32Eqz,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                ]);
                inst.append(&mut y.visit_lowlevel(()));
                inst.append(&mut vec![
                    Instruction::Else,
                    Instruction::I32Const(1),
                    Instruction::End,
                ]);
            }
            WasmExpression::BooleanAnd(x, y) => {
                inst.append(&mut x.visit_lowlevel(()));
                inst.append(&mut vec![
                    Instruction::I32Eqz,
                    Instruction::If(BlockType::Value(ValueType::I32)),
                    Instruction::I32Const(0),
                    Instruction::Else,
                ]);
                inst.append(&mut y.visit_lowlevel(()));
                inst.push(Instruction::End);
            }
        }
        inst
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WasmSimpleInfix {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

impl LowLevelVisitor for WasmSimpleInfix {
    type Argument = ();
    type Return = Instruction;

    fn visit_lowlevel(&self, (): Self::Argument) -> Self::Return {
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
