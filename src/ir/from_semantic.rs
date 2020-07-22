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
    type Return = Vec<ir::Expression>;

    fn visit_sem(&self, (): Self::Argument) -> Self::Return {
        self.0.iter().map(|smt| smt.visit_sem(())).collect()
    }
}

impl SemanticVisitor for semantic::Statement {
    type Argument = ();
    type Return = ir::Expression;

    fn visit_sem(&self, (): Self::Argument) -> Self::Return {
        match self {
            Self::LetBinding(_name, symbol_id, expr) => {
                ir::Expression::LocalDeclare(*symbol_id, Box::new(expr.visit_sem(())))
            }
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
            Self::Lookup(_name, symbol_id) => ir::Expression::LocalGet(*symbol_id),
            Self::Block(block) => ir::Expression::Block(block.visit_sem(())),
            Self::Assignment(_name, symbol_id, expr) => {
                ir::Expression::LocalSet(*symbol_id, Box::new(expr.visit_sem(())))
            }
            Self::ReturnValue(expr) => ir::Expression::Return(Box::new(expr.visit_sem(()))),
            Self::PrefixCall(op, expr) => op.visit_sem(expr.visit_sem(())),
            Self::InfixCall(x, op, y) => op.visit_sem((x.visit_sem(()), y.visit_sem(()))),
        }
    }
}

impl SemanticVisitor for semantic::PrefixOp {
    type Argument = ir::Expression;
    type Return = ir::Expression;

    fn visit_sem(&self, expr: Self::Argument) -> Self::Return {
        match self {
            Self::Negate => ir::Expression::Negate(Box::new(expr)),
            Self::BooleanNot => ir::Expression::BooleanNot(Box::new(expr)),
        }
    }
}

impl SemanticVisitor for semantic::InfixOp {
    type Argument = (ir::Expression, ir::Expression);
    type Return = ir::Expression;

    fn visit_sem(&self, (x, y): Self::Argument) -> Self::Return {
        let x = Box::new(x);
        let y = Box::new(y);
        let helper = |op| ir::Expression::SimpleInfixCall(x.clone(), op, y.clone());

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
            Self::BooleanOr => ir::Expression::BooleanOr(x, y),
            Self::BooleanAnd => ir::Expression::BooleanAnd(x, y),
        }
    }
}
