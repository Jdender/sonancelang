use super::{parser::*, visitor::*};
use parity_wasm::{
    builder::module,
    elements::{Instruction, Instructions, Module},
};

pub fn generate(expr: Expr) -> Result<Module, GeneratorError> {
    let mut this = Generator {
        instructions: vec![],
        states: vec![],
    };
    expr.accept(&mut this)?;
    this.instructions.push(Instruction::End);

    Ok(module()
        .export()
        .field("main")
        .internal()
        .func(0)
        .build()
        .function()
        .main()
        .signature()
        .return_type()
        .i32()
        .build()
        .body()
        .with_instructions(Instructions::new(this.instructions))
        .build()
        .build()
        .build())
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberType {
    Float,
    Int,
}

#[derive(Debug, Clone)]
pub enum GeneratorError {
    NotEnoughState,
    InvalidState,
    TypeMismatch,
}
use GeneratorError::*;

#[derive(Debug, Clone)]
pub enum GeneratorState {
    Argument(NumberType),
}
use GeneratorState::*;

#[derive(Debug, Clone)]
pub struct Generator {
    instructions: Vec<Instruction>,
    states: Vec<GeneratorState>,
}

impl AstVisitor for Generator {
    type Result = Result<(), GeneratorError>;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        use Expr::*;
        match expr {
            Literal(num) => num.accept(self)?,
            Operation(x, op, y) => {
                x.accept(self)?;
                y.accept(self)?;
                op.accept(self)?;
            }
        };
        Ok(())
    }

    fn visit_opcode(&mut self, opcode: &Opcode) -> Self::Result {
        use Instruction::*;
        use Opcode::*;
        use NumberType::*;

        let number_type = match self.states.iter().rev().take(2).collect::<Vec<_>>()[..] {
            [Argument(x), Argument(y)] => {
                if x != y {
                    Err(TypeMismatch)?
                }
                x
            }
            _ => Err(InvalidState)?,
        };

        self.instructions.push(match (opcode, number_type) {
            (Add, Int) => I32Add,
            (Sub, Int) => I32Sub,
            (Mul, Int) => I32Mul,
            (Div, Int) => I32DivS,
            (Add, Float) => F32Add,
            (Sub, Float) => F32Sub,
            (Mul, Float) => F32Mul,
            (Div, Float) => F32Div,
        });
        Ok(())
    }

    fn visit_number_literal(&mut self, number: &NumberLiteral) -> Self::Result {
        use Instruction::*;
        let (inst, state) = match number {
            NumberLiteral::Int(num) => (I32Const(*num), NumberType::Int),
            NumberLiteral::Float(num) => {
                (F32Const(float_to_int_literally(*num)), NumberType::Float)
            }
        };
        self.instructions.push(inst);
        self.states.push(GeneratorState::Argument(state));
        Ok(())
    }
}

fn float_to_int_literally(num: f32) -> u32 {
    unsafe { std::mem::transmute(num) }
}
