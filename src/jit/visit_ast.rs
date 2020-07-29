use super::ast;
use cranelift::prelude::*;

pub trait VisitAst {
    type Output;
    fn visit_ast(&self, builder: &mut FunctionBuilder) -> Self::Output;
}

impl VisitAst for ast::File {
    type Output = ();
    fn visit_ast(&self, builder: &mut FunctionBuilder) -> Self::Output {
        let block = builder.create_block();

        builder.switch_to_block(block);
        builder.append_block_params_for_function_params(block);

        let result = self.number.visit_ast(builder);

        builder.ins().return_(&[result]);

        builder.seal_all_blocks();
        builder.finalize();
    }
}

impl VisitAst for ast::Expression {
    type Output = Value;
    fn visit_ast(&self, builder: &mut FunctionBuilder) -> Self::Output {
        match self {
            Self::Literal(num) => builder.ins().iconst(types::I32, i64::from(*num)),
            Self::InfixCall {
                left,
                operator: ast::InfixOperator::Add,
                right,
            } => {
                let left = left.visit_ast(builder);
                let right = right.visit_ast(builder);
                builder.ins().iadd(left, right)
            }
        }
    }
}
