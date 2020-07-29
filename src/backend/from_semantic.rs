use super::super::semantic;
use cranelift::prelude::*;

pub trait SemanticVisitor {
    type Param;
    type Output;
    fn visit_semantic(&self, builder: &mut FunctionBuilder, param: Self::Param) -> Self::Output;
}

impl SemanticVisitor for semantic::File {
    type Param = ();
    type Output = ();
    fn visit_semantic(&self, builder: &mut FunctionBuilder, _: Self::Param) -> Self::Output {
        let block = builder.create_block();

        builder.switch_to_block(block);
        builder.append_block_params_for_function_params(block);

        let result = self.body.visit_semantic(builder, ());

        builder.ins().return_(&[result]);

        builder.seal_all_blocks();
        builder.finalize();
    }
}

impl SemanticVisitor for semantic::Block {
    type Param = ();
    type Output = Value;
    fn visit_semantic(&self, builder: &mut FunctionBuilder, _: Self::Param) -> Self::Output {
        for stmt in self.body.iter() {
            stmt.visit_semantic(builder, ());
        }

        self.trailing.visit_semantic(builder, ())
    }
}

impl SemanticVisitor for semantic::Statement {
    type Param = ();
    type Output = ();
    fn visit_semantic(&self, builder: &mut FunctionBuilder, _: Self::Param) -> Self::Output {
        match self {
            Self::LetBinding {
                value, symbol_id, ..
            } => {
                let symbol_id = symbol_id.into();
                let value = value.visit_semantic(builder, ());

                builder.declare_var(symbol_id, types::I32);
                builder.def_var(symbol_id, value);
            }
            Self::SideEffect(expr) => {
                expr.visit_semantic(builder, ());
            }
        }
    }
}

impl From<&semantic::SymbolId> for Variable {
    fn from(id: &semantic::SymbolId) -> Self {
        Variable::with_u32(id.as_u32())
    }
}

impl SemanticVisitor for semantic::Expression {
    type Param = ();
    type Output = Value;
    fn visit_semantic(&self, builder: &mut FunctionBuilder, _: Self::Param) -> Self::Output {
        match self {
            Self::Literal(num) => builder.ins().iconst(types::I32, i64::from(*num)),
            Self::Lookup { symbol_id, .. } => builder.use_var(symbol_id.into()),
            Self::Block(block) => block.visit_semantic(builder, ()),
            Self::Assignment {
                symbol_id, value, ..
            } => {
                let value = value.visit_semantic(builder, ());
                builder.def_var(symbol_id.into(), value);
                builder.ins().iconst(types::I32, 0)
            }
            Self::PrefixCall { operator, value } => {
                let value = value.visit_semantic(builder, ());
                operator.visit_semantic(builder, value)
            }
            Self::InfixCall {
                left,
                operator,
                right,
            } => {
                let left = left.visit_semantic(builder, ());
                let right = right.visit_semantic(builder, ());
                operator.visit_semantic(builder, (left, right))
            }
        }
    }
}

impl SemanticVisitor for semantic::PrefixOperator {
    type Param = Value;
    type Output = Value;
    fn visit_semantic(&self, builder: &mut FunctionBuilder, value: Self::Param) -> Self::Output {
        match self {
            Self::Negate => builder.ins().ineg(value),
        }
    }
}

impl SemanticVisitor for semantic::InfixOperator {
    type Param = (Value, Value);
    type Output = Value;
    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        (left, right): Self::Param,
    ) -> Self::Output {
        match self {
            Self::Add => builder.ins().iadd(left, right),
            Self::Subtract => builder.ins().isub(left, right),
            Self::Multiply => builder.ins().imul(left, right),
            Self::Divide => builder.ins().sdiv(left, right),
        }
    }
}
