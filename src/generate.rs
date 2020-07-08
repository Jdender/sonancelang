use crate::lowlevel::*;
use crate::parse::*;

pub trait AstVisitor {
    type Argument;
    type Return;

    fn visit_ast(&self, args: Self::Argument) -> Self::Return;
}

impl AstVisitor for File {
    type Argument = ();
    type Return = WasmModule;

    fn visit_ast(&self, (): Self::Argument) -> Self::Return {
        WasmModule {
            name: self.name.to_string(),
            body: self.body.iter().map(|smt| smt.visit_ast(())).collect(),
        }
    }
}

impl AstVisitor for Statement {
    type Argument = ();
    type Return = WasmExpression;

    fn visit_ast(&self, (): Self::Argument) -> Self::Return {
        match self {
            Statement::LetBinding(..) => unimplemented!(),
            Statement::Expression(expr) => expr.visit_ast(()),
        }
    }
}

impl AstVisitor for Expression {
    type Argument = ();
    type Return = WasmExpression;

    fn visit_ast(&self, (): Self::Argument) -> <Self as AstVisitor>::Return {
        match self {
            Expression::Literal(num) => WasmExpression::Const(*num),
            Expression::Lookup(..) => unimplemented!(),
            Expression::Return(expr) => WasmExpression::Return(Box::new(expr.visit_ast(()))),
            Expression::PrefixCall(op, expr) => op.visit_ast(expr.visit_ast(())),
            Expression::InfixCall(x, op, y) => op.visit_ast((x.visit_ast(()), y.visit_ast(()))),
        }
    }
}

impl AstVisitor for PrefixOp {
    type Argument = WasmExpression;
    type Return = WasmExpression;

    fn visit_ast(&self, expr: Self::Argument) -> Self::Return {
        match self {
            PrefixOp::Negate => WasmExpression::Negate(Box::new(expr)),
            PrefixOp::BooleanNot => WasmExpression::BooleanNot(Box::new(expr)),
        }
    }
}

impl AstVisitor for InfixOp {
    type Argument = (WasmExpression, WasmExpression);
    type Return = WasmExpression;

    fn visit_ast(&self, (x, y): Self::Argument) -> Self::Return {
        let x = Box::new(x);
        let y = Box::new(y);
        let helper = |op| WasmExpression::SimpleInfixCall(x.clone(), op, y.clone());

        match self {
            InfixOp::Add => helper(WasmSimpleInfix::Add),
            InfixOp::Subtract => helper(WasmSimpleInfix::Subtract),
            InfixOp::Multiply => helper(WasmSimpleInfix::Multiply),
            InfixOp::Divide => helper(WasmSimpleInfix::Divide),

            InfixOp::Equal => helper(WasmSimpleInfix::Equal),
            InfixOp::NotEqual => helper(WasmSimpleInfix::NotEqual),
            InfixOp::GreaterThan => helper(WasmSimpleInfix::GreaterThan),
            InfixOp::LessThan => helper(WasmSimpleInfix::LessThan),
            InfixOp::GreaterOrEqual => helper(WasmSimpleInfix::GreaterOrEqual),
            InfixOp::LessOrEqual => helper(WasmSimpleInfix::LessOrEqual),

            InfixOp::BooleanOr => WasmExpression::BooleanOr(x, y),
            InfixOp::BooleanAnd => WasmExpression::BooleanAnd(x, y),
        }
    }
}
