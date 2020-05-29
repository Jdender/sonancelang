use super::parser::*;
use parity_wasm::{
    builder::module,
    elements::{Instruction, Instructions, Module},
};

pub fn generate(expr: Expr) -> Result<Module, GeneratorError> {
    let mut result = expr.accept(())?;

    result.instructions.push(Instruction::End);

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
        .with_instructions(Instructions::new(result.instructions))
        .build()
        .build()
        .build())
}

#[derive(Debug, Clone)]
pub enum GeneratorError {
    TypeMismatch(Type, Opcode, Type),
}
use GeneratorError::*;

trait AstVisitor {
    type Argument;
    type Return;

    fn accept(&self, args: Self::Argument) -> Self::Return;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
}

#[derive(Debug, Clone, PartialEq)]
struct ExprReturn {
    instructions: Vec<Instruction>,
    value_type: Type,
}

impl AstVisitor for Expr {
    type Argument = ();
    type Return = Result<ExprReturn, GeneratorError>;

    fn accept(&self, _: Self::Argument) -> Self::Return {
        use Expr::*;

        Ok(match self {
            Literal(num) => num.accept(()),
            Operation(x, op, y) => {
                let x = x.accept(())?;
                let y = y.accept(())?;
                let op = op.accept((x.value_type, y.value_type))?;

                ExprReturn {
                    instructions: x
                        .instructions
                        .into_iter()
                        .chain(y.instructions.into_iter())
                        .chain(op.instructions.into_iter())
                        .collect(),
                    value_type: op.value_type,
                }
            }
        })
    }
}

impl AstVisitor for Literal {
    type Argument = ();
    type Return = ExprReturn;

    fn accept(&self, _: Self::Argument) -> Self::Return {
        use Literal::*;

        fn float_to_int_literally(num: f32) -> u32 {
            unsafe { std::mem::transmute(num) }
        }

        match self {
            Int(num) => ExprReturn {
                instructions: vec![Instruction::I32Const(*num)],
                value_type: Type::Int,
            },
            Float(num) => ExprReturn {
                instructions: vec![Instruction::F32Const(float_to_int_literally(*num))],
                value_type: Type::Float,
            },
        }
    }
}

impl AstVisitor for Opcode {
    type Argument = (Type, Type);
    type Return = Result<ExprReturn, GeneratorError>;

    fn accept(&self, (x, y): Self::Argument) -> Self::Return {
        use Instruction::*;
        use Opcode::*;
        use Type::*;

        if x != y {
            Err(TypeMismatch(x.clone(), self.clone(), y.clone()))?;
        }

        Ok(ExprReturn {
            value_type: x,
            instructions: vec![match (self, y) {
                (Add, Int) => I32Add,
                (Sub, Int) => I32Sub,
                (Mul, Int) => I32Mul,
                (Div, Int) => I32DivS,
                (Add, Float) => F32Add,
                (Sub, Float) => F32Sub,
                (Mul, Float) => F32Mul,
                (Div, Float) => F32Div,
            }],
        })
    }
}
