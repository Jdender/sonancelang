use super::super::semantic;
use cranelift::prelude::*;

pub trait SemanticVisitor {
    type Param;
    type Output;

    fn visit_semantic(&self, builder: &mut FunctionBuilder, param: Self::Param) -> Self::Output;
}

impl SemanticVisitor for semantic::Function {
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
                let ty = value.ty.into();

                let symbol_id = symbol_id.into();
                let value = value.visit_semantic(builder, ());

                builder.declare_var(symbol_id, ty);
                builder.def_var(symbol_id, value);
            }
            Self::SideEffect(expr) => {
                expr.visit_semantic(builder, ());
            }
        }
    }
}

impl From<semantic::Ty> for Type {
    fn from(ty: semantic::Ty) -> Self {
        match ty {
            semantic::Ty::I32 => types::I32,
            semantic::Ty::F32 => types::F32,
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
        use semantic::ExpressionKind::*;
        match &self.kind {
            Literal(literal) => literal.visit_semantic(builder, ()),

            Lookup { symbol_id, .. } => builder.use_var(symbol_id.into()),

            Block(block) => block.visit_semantic(builder, ()),

            Assignment {
                symbol_id, value, ..
            } => {
                let value = value.visit_semantic(builder, ());
                builder.def_var(symbol_id.into(), value);
                builder.ins().iconst(types::I32, 0)
            }

            PrefixCall { operator, value } => {
                let ty = value.ty;
                let value = value.visit_semantic(builder, ());
                operator.visit_semantic(builder, (ty, value))
            }

            InfixCall {
                left,
                operator,
                right,
            } => {
                let ty = left.ty;
                let left = left.visit_semantic(builder, ());
                let right = right.visit_semantic(builder, ());
                operator.visit_semantic(builder, (ty, left, right))
            }

            IfElse {
                predicate,
                when_true,
                when_false,
            } => {
                // Create new basic blocks
                let true_block = builder.create_block();
                let else_block = builder.create_block();
                let merge_block = builder.create_block();

                // Merge takes the result from either block
                builder.append_block_param(merge_block, self.ty.into());

                // Jump if predicate is zero, otherwise fall through
                let predicate = predicate.visit_semantic(builder, ());
                builder.ins().brz(predicate, else_block, &[]);
                builder.ins().jump(true_block, &[]);

                // Setup when_true block
                builder.switch_to_block(true_block);
                builder.seal_block(true_block);
                let when_true = when_true.visit_semantic(builder, ());
                builder.ins().jump(merge_block, &[when_true]);

                // Same as above but for when_false
                builder.switch_to_block(else_block);
                builder.seal_block(else_block);
                let when_false = when_false.visit_semantic(builder, ());
                builder.ins().jump(merge_block, &[when_false]);

                // Finish the merge and return result
                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);
                builder.block_params(merge_block)[0]
            }
        }
    }
}

impl SemanticVisitor for semantic::Literal {
    type Param = ();
    type Output = Value;

    fn visit_semantic(&self, builder: &mut FunctionBuilder, _: Self::Param) -> Self::Output {
        match self {
            Self::I32(num) => builder.ins().iconst(types::I32, i64::from(*num)),
            Self::F32(num) => builder.ins().f32const(*num),
        }
    }
}

impl SemanticVisitor for semantic::PrefixOperator {
    type Param = (semantic::Ty, Value);
    type Output = Value;

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        (ty, value): Self::Param,
    ) -> Self::Output {
        match ty {
            semantic::Ty::I32 => match self {
                Self::Negate => builder.ins().ineg(value),
            },
            semantic::Ty::F32 => match self {
                Self::Negate => builder.ins().fneg(value),
            },
        }
    }
}

impl SemanticVisitor for semantic::InfixOperator {
    type Param = (semantic::Ty, Value, Value);
    type Output = Value;

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        (ty, left, right): Self::Param,
    ) -> Self::Output {
        match ty {
            semantic::Ty::I32 => match self {
                Self::Add => builder.ins().iadd(left, right),
                Self::Subtract => builder.ins().isub(left, right),
                Self::Multiply => builder.ins().imul(left, right),
                Self::Divide => builder.ins().sdiv(left, right),

                Self::Equal => builder.ins().icmp(IntCC::Equal, left, right),
                Self::NotEqual => builder.ins().icmp(IntCC::NotEqual, left, right),
                Self::GreaterThan => builder.ins().icmp(IntCC::SignedGreaterThan, left, right),
                Self::LessThan => builder.ins().icmp(IntCC::SignedLessThan, left, right),
                Self::GreaterOrEqual => {
                    builder
                        .ins()
                        .icmp(IntCC::SignedGreaterThanOrEqual, left, right)
                }
                Self::LessOrEqual => builder
                    .ins()
                    .icmp(IntCC::SignedLessThanOrEqual, left, right),
            },
            semantic::Ty::F32 => match self {
                Self::Add => builder.ins().fadd(left, right),
                Self::Subtract => builder.ins().fsub(left, right),
                Self::Multiply => builder.ins().fmul(left, right),
                Self::Divide => builder.ins().fdiv(left, right),

                Self::Equal => builder.ins().fcmp(FloatCC::Equal, left, right),
                Self::NotEqual => builder.ins().fcmp(FloatCC::NotEqual, left, right),
                Self::GreaterThan => builder.ins().fcmp(FloatCC::GreaterThan, left, right),
                Self::LessThan => builder.ins().fcmp(FloatCC::LessThan, left, right),
                Self::GreaterOrEqual => {
                    builder.ins().fcmp(FloatCC::GreaterThanOrEqual, left, right)
                }
                Self::LessOrEqual => builder.ins().fcmp(FloatCC::LessThanOrEqual, left, right),
            },
        }
    }
}
