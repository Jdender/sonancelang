use super::{
    super::{ir, semantic},
    IrResult,
};

pub fn ir_pass(input: crate::semantic::File) -> IrResult<ir::WasmModule> {
    Ok(input.visit_sem(()))
}

pub trait SemanticVisitor {
    type Argument;
    type Return;

    fn visit_sem(&self, args: Self::Argument) -> Self::Return;
}

impl SemanticVisitor for semantic::File {
    type Argument = ();
    type Return = ir::WasmModule;

    fn visit_sem(&self, (): Self::Argument) -> Self::Return {
        ir::WasmModule {
            name: self.name.0.clone(),
            body: self.body.visit_sem(()),
        }
    }
}

impl SemanticVisitor for semantic::Block {
    type Argument = ();
    type Return = ir::Block;

    fn visit_sem(&self, (): Self::Argument) -> Self::Return {
        ir::Block {
            body: self.body.iter().map(|smt| smt.visit_sem(())).collect(),
            trailing: Box::new(self.trailing.visit_sem(())),
        }
    }
}

impl SemanticVisitor for semantic::Statement {
    type Argument = ();
    type Return = ir::Expression;

    fn visit_sem(&self, (): Self::Argument) -> Self::Return {
        match self {
            Self::LetBinding {
                symbol_id, operand, ..
            } => ir::Expression::LocalDeclare(*symbol_id, Box::new(operand.visit_sem(()))),
            Self::Expression(expr) => expr.visit_sem(()),
        }
    }
}

impl SemanticVisitor for semantic::Expression {
    type Argument = ();
    type Return = ir::Expression;

    fn visit_sem(&self, (): Self::Argument) -> <Self as SemanticVisitor>::Return {
        match self {
            Self::Literal(num) => ir::Expression::Const(*num),
            Self::Lookup { symbol_id, .. } => ir::Expression::LocalGet(*symbol_id),
            Self::Block(block) => ir::Expression::Block(block.visit_sem(())),

            Self::Assignment {
                symbol_id, operand, ..
            } => ir::Expression::LocalSet(*symbol_id, Box::new(operand.visit_sem(()))),

            Self::ReturnValue(expr) => ir::Expression::Return(Box::new(expr.visit_sem(()))),
            Self::PrefixCall { operator, operand } => operator.visit_sem(operand.visit_sem(())),

            Self::InfixCall {
                operator,
                x_operand,
                y_operand,
            } => operator.visit_sem((x_operand.visit_sem(()), y_operand.visit_sem(()))),

            Self::Conditional {
                predicate,
                when_true,
                when_false,
            } => ir::Expression::Conditional {
                predicate: Box::new(predicate.visit_sem(())),
                when_true: when_true.visit_sem(()),
                when_false: when_false.visit_sem(()),
            },
        }
    }
}

impl SemanticVisitor for semantic::PrefixOperator {
    type Argument = ir::Expression;
    type Return = ir::Expression;

    fn visit_sem(&self, expr: Self::Argument) -> Self::Return {
        match self {
            Self::Negate => ir::Expression::Negate(Box::new(expr)),
            Self::BooleanNot => ir::Expression::BooleanNot(Box::new(expr)),
        }
    }
}

impl SemanticVisitor for semantic::InfixOperator {
    type Argument = (ir::Expression, ir::Expression);
    type Return = ir::Expression;

    fn visit_sem(&self, (x, y): Self::Argument) -> Self::Return {
        let x = Box::new(x);
        let y = Box::new(y);
        let helper = |operator| ir::Expression::SimpleInfixCall {
            operator,
            x_operand: x.clone(),
            y_operand: y.clone(),
        };

        match self {
            Self::Add => helper(ir::SimpleInfix::Add),
            Self::Subtract => helper(ir::SimpleInfix::Subtract),
            Self::Multiply => helper(ir::SimpleInfix::Multiply),
            Self::Divide => helper(ir::SimpleInfix::Divide),
            Self::Equal => helper(ir::SimpleInfix::Equal),
            Self::NotEqual => helper(ir::SimpleInfix::NotEqual),
            Self::GreaterThan => helper(ir::SimpleInfix::GreaterThan),
            Self::LessThan => helper(ir::SimpleInfix::LessThan),
            Self::GreaterOrEqual => helper(ir::SimpleInfix::GreaterOrEqual),
            Self::LessOrEqual => helper(ir::SimpleInfix::LessOrEqual),

            Self::BooleanOr => ir::Expression::Conditional {
                predicate: x,
                when_true: ir::Block {
                    body: vec![],
                    trailing: Box::new(ir::Expression::Const(1)),
                },
                when_false: ir::Block {
                    body: vec![],
                    trailing: y,
                },
            },

            Self::BooleanAnd => ir::Expression::Conditional {
                predicate: x,
                when_true: ir::Block {
                    body: vec![],
                    trailing: y,
                },
                when_false: ir::Block {
                    body: vec![],
                    trailing: Box::new(ir::Expression::Const(0)),
                },
            },
        }
    }
}
