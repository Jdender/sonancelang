mod block;
mod expr_misc;
mod expression;

pub use {
    super::{super::semantic, BackendContext},
    cranelift::prelude::*,
};

pub trait SemanticVisitor {
    type Param;
    type Output;

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        context: &BackendContext,
        param: Self::Param,
    ) -> Self::Output;
}

impl SemanticVisitor for semantic::Function {
    type Param = ();
    type Output = ();

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        context: &BackendContext,
        _: Self::Param,
    ) -> Self::Output {
        let block = builder.create_block();

        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        for param in self.head.params.iter() {
            builder.declare_var((&param.symbol_id).into(), param.ty.into());
        }

        let result = self.body.visit_semantic(builder, context, ());

        builder.ins().return_(&[result]);

        builder.seal_all_blocks();
        builder.finalize();
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
