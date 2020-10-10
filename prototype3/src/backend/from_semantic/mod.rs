mod block;
mod expr_misc;
mod expression;

pub use {
    super::{super::semantic::type_check as semantic, BackendContext},
    cranelift::prelude::*,
};

impl semantic::Function {
    pub fn visit_semantic(self, builder: &mut FunctionBuilder, context: &BackendContext) {
        let block = builder.create_block();

        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        for (i, param) in self.params.iter().enumerate() {
            builder.declare_var(param.symbol_id.into(), ty_to_type(param.ty, context));
            builder.def_var(param.symbol_id.into(), builder.block_params(block)[i]);
        }

        let result = self.body.visit_semantic(builder, context);

        builder.ins().return_(&[result]);

        builder.seal_all_blocks();
        builder.finalize();
    }
}

pub fn ty_to_type(ty: semantic::Ty, context: &BackendContext) -> Type {
    use semantic::Ty;
    match ty {
        Ty::I8 | Ty::U8 => types::I8,
        Ty::I16 | Ty::U16 => types::I16,
        Ty::I32 | Ty::U32 => types::I32,
        Ty::I64 | Ty::U64 => types::I64,
        Ty::ISize | Ty::USize => context.module.target_config().pointer_type(),
        Ty::F32 => types::F32,
        Ty::F64 => types::F64,
    }
}

impl From<semantic::SymbolId> for Variable {
    fn from(id: semantic::SymbolId) -> Self {
        Variable::with_u32(id.as_u32())
    }
}
