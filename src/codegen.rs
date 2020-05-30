use super::parser::*;
use parity_wasm::{
    builder::module,
    elements::{Instruction, Instructions, Local, Module, ValueType},
};
use std::collections::HashMap;

pub fn generate(block: Block) -> Result<Module, GeneratorError> {
    let mut result = dbg!(block.visit(())?);

    result.instructions.push(Instruction::End);

    Ok(module()
        .export()
        .field("main")
        .internal()
        .func(0)
        .build()
        .function()
        .signature()
        .with_return_type(result.result_type.map(|t| type_to_raw(&t)))
        .build()
        .body()
        .with_locals(dbg!(result.locals))
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

    fn visit(&self, args: Self::Argument) -> Self::Return;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
}

#[derive(Debug, Clone)]
struct Symbol {
    index: u32,
    ident: Identifier,
    result_type: Type,
}

#[derive(Debug, Clone)]
struct BlockResult {
    instructions: Vec<Instruction>,
    locals: Vec<Local>,
    result_type: Option<Type>,
}

fn type_to_raw(input: &Type) -> ValueType {
    match input {
        Type::Int => ValueType::I32,
        Type::Float => ValueType::F32,
    }
}

impl AstVisitor for Block {
    type Argument = ();
    type Return = Result<BlockResult, GeneratorError>;

    fn visit(&self, _: Self::Argument) -> Self::Return {
        let mut instructions = vec![];
        let mut local_index = 0;
        let mut symbols = HashMap::new();

        for stmt in &self.body[..] {
            let mut stmt = stmt.visit(())?;
            instructions.append(&mut stmt.instructions);

            if let Some(dec) = stmt.new_declaration {
                symbols.insert(
                    dec.ident.0.clone(),
                    Symbol {
                        index: local_index,
                        ident: dec.ident,
                        result_type: dec.result_type,
                    },
                );
                instructions.push(Instruction::SetLocal(local_index));
                local_index += 1;
            }
        }

        let result_type = match &self.trailing {
            Some(expr) => {
                let mut expr = expr.visit(())?;
                instructions.append(&mut expr.instructions);
                Some(expr.result_type)
            }
            None => None,
        };

        let locals = {
            let mut symbols: Vec<_> = symbols.values().collect();

            symbols.sort_by_key(|sym| sym.index);

            symbols
                .iter()
                .map(|sym| Local::new(1, type_to_raw(&sym.result_type)))
                .collect()
        };

        Ok(BlockResult {
            instructions,
            result_type,
            locals,
        })
    }
}

#[derive(Debug, Clone)]
struct Declaration {
    ident: Identifier,
    result_type: Type,
}

#[derive(Debug, Clone)]
struct StatementResult {
    instructions: Vec<Instruction>,
    new_declaration: Option<Declaration>,
}

impl AstVisitor for Statement {
    type Argument = ();
    type Return = Result<StatementResult, GeneratorError>;

    fn visit(&self, _: Self::Argument) -> Self::Return {
        use Statement::*;
        Ok(match self {
            SideEffect(expr) => StatementResult {
                instructions: expr.visit(())?.instructions,
                new_declaration: None,
            },
            Assignment(ident, expr) => {
                let expr = expr.visit(())?;
                StatementResult {
                    instructions: expr.instructions,
                    new_declaration: Some(Declaration {
                        ident: ident.clone(),
                        result_type: expr.result_type,
                    }),
                }
            }
        })
    }
}

#[derive(Debug, Clone)]
struct ExpressionResult {
    instructions: Vec<Instruction>,
    result_type: Type,
}

impl AstVisitor for Expression {
    type Argument = ();
    type Return = Result<ExpressionResult, GeneratorError>;

    fn visit(&self, _: Self::Argument) -> Self::Return {
        use Expression::*;

        Ok(match self {
            Literal(num) => num.visit(()),
            Operation(x, op, y) => {
                let x = x.visit(())?;
                let y = y.visit(())?;
                let op = op.visit((x.result_type, y.result_type))?;

                ExpressionResult {
                    instructions: x
                        .instructions
                        .into_iter()
                        .chain(y.instructions.into_iter())
                        .chain(op.instructions.into_iter())
                        .collect(),
                    result_type: op.result_type,
                }
            }
        })
    }
}

impl AstVisitor for Literal {
    type Argument = ();
    type Return = ExpressionResult;

    fn visit(&self, _: Self::Argument) -> Self::Return {
        use Literal::*;

        fn float_to_int_literally(num: f32) -> u32 {
            unsafe { std::mem::transmute(num) }
        }

        match self {
            Int(num) => ExpressionResult {
                instructions: vec![Instruction::I32Const(*num)],
                result_type: Type::Int,
            },
            Float(num) => ExpressionResult {
                instructions: vec![Instruction::F32Const(float_to_int_literally(*num))],
                result_type: Type::Float,
            },
        }
    }
}

impl AstVisitor for Opcode {
    type Argument = (Type, Type);
    type Return = Result<ExpressionResult, GeneratorError>;

    fn visit(&self, (x, y): Self::Argument) -> Self::Return {
        use Instruction::*;
        use Opcode::*;
        use Type::*;

        if x != y {
            Err(TypeMismatch(x.clone(), self.clone(), y.clone()))?;
        }

        Ok(ExpressionResult {
            result_type: x,
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
