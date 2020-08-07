pub use super::*;

impl SemanticVisitor for semantic::Expression {
    type Param = ();
    type Output = Value;

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        context: &BackendContext,
        _: Self::Param,
    ) -> Self::Output {
        use semantic::ExpressionKind::*;
        match &self.kind {
            Literal(literal) => literal.visit_semantic(builder, context, ()),

            Lookup { symbol_id, .. } => builder.use_var(symbol_id.into()),

            Block(block) => block.visit_semantic(builder, context, ()),

            Assignment {
                symbol_id, value, ..
            } => {
                let value = value.visit_semantic(builder, context, ());
                builder.def_var(symbol_id.into(), value);
                builder.ins().iconst(types::I32, 0)
            }

            FuncCall {
                args, symbol_id, ..
            } => {
                let call = context
                    .func_table
                    .get(symbol_id)
                    .expect("Func should exist");

                let call = context
                    .module
                    .declare_func_in_func(*call, &mut builder.func);

                let args = args
                    .iter()
                    .map(|a| a.visit_semantic(builder, context, ()))
                    .collect::<Vec<_>>();

                let call = builder.ins().call(call, &args);
                builder.inst_results(call)[0]
            }

            PrefixCall { operator, value } => {
                let ty = value.ty;
                let value = value.visit_semantic(builder, context, ());
                operator.visit_semantic(builder, context, (ty, value))
            }

            InfixCall {
                left,
                operator,
                right,
            } => {
                let ty = left.ty;
                let left = left.visit_semantic(builder, context, ());
                let right = right.visit_semantic(builder, context, ());
                operator.visit_semantic(builder, context, (ty, left, right))
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
                let predicate = predicate.visit_semantic(builder, context, ());
                builder.ins().brz(predicate, else_block, &[]);
                builder.ins().jump(true_block, &[]);

                // Setup when_true block
                builder.switch_to_block(true_block);
                builder.seal_block(true_block);
                let when_true = when_true.visit_semantic(builder, context, ());
                builder.ins().jump(merge_block, &[when_true]);

                // Same as above but for when_false
                builder.switch_to_block(else_block);
                builder.seal_block(else_block);
                let when_false = when_false.visit_semantic(builder, context, ());
                builder.ins().jump(merge_block, &[when_false]);

                // Finish the merge and return result
                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);
                builder.block_params(merge_block)[0]
            }
        }
    }
}
