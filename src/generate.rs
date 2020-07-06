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
        WasmModule(self.0.to_string(), self.1.visit_ast(()))
    }
}

impl AstVisitor for Expression {
    type Argument = ();
    type Return = WasmExpression;

    fn visit_ast(&self, (): Self::Argument) -> Self::Return {
        match self {
            Expression::Literal(num) => WasmExpression::Const(*num),
            Expression::PrefixOp(op, expr) => op.visit_ast(expr.visit_ast(())),
            Expression::InfixOp(x, op, y) => op.visit_ast((x.visit_ast(()), y.visit_ast(()))),
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
        match self {
            InfixOp::Add => WasmExpression::Add(Box::new(x), Box::new(y)),
            InfixOp::Subtract => WasmExpression::Subtract(Box::new(x), Box::new(y)),
            InfixOp::Multiply => WasmExpression::Multiply(Box::new(x), Box::new(y)),
            InfixOp::Divide => WasmExpression::Divide(Box::new(x), Box::new(y)),
        }
    }
}
