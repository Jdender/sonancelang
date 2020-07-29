use super::ast;
use cranelift::prelude::*;

pub trait VisitAst {
    type Param;
    type Output;
    fn visit_ast(&self, builder: &mut FunctionBuilder, param: Self::Param) -> Self::Output;
}

impl VisitAst for ast::File {
    type Param = ();
    type Output = ();
    fn visit_ast(&self, builder: &mut FunctionBuilder, (): Self::Param) -> Self::Output {
        let block = builder.create_block();

        builder.switch_to_block(block);
        builder.append_block_params_for_function_params(block);

        let result = self.body.visit_ast(builder, ());

        builder.ins().return_(&[result]);

        builder.seal_all_blocks();
        builder.finalize();
    }
}

impl VisitAst for ast::Expression {
    type Param = ();
    type Output = Value;
    fn visit_ast(&self, builder: &mut FunctionBuilder, (): Self::Param) -> Self::Output {
        match self {
            Self::Literal(num) => builder.ins().iconst(types::I32, i64::from(*num)),
            Self::PrefixCall { operator, value } => {
                let value = value.visit_ast(builder, ());
                operator.visit_ast(builder, value)
            }
            Self::InfixCall {
                left,
                operator,
                right,
            } => {
                let left = left.visit_ast(builder, ());
                let right = right.visit_ast(builder, ());
                operator.visit_ast(builder, (left, right))
            }
        }
    }
}

impl VisitAst for ast::PrefixOperator {
    type Param = Value;
    type Output = Value;
    fn visit_ast(&self, builder: &mut FunctionBuilder, value: Self::Param) -> Self::Output {
        match self {
            Self::Negate => builder.ins().ineg(value),
        }
    }
}

impl VisitAst for ast::InfixOperator {
    type Param = (Value, Value);
    type Output = Value;
    fn visit_ast(&self, builder: &mut FunctionBuilder, (left, right): Self::Param) -> Self::Output {
        match self {
            Self::Add => builder.ins().iadd(left, right),
            Self::Subtract => builder.ins().isub(left, right),
            Self::Multiply => builder.ins().imul(left, right),
            Self::Divide => builder.ins().sdiv(left, right),
        }
    }
}
