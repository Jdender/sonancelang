pub use super::*;

impl semantic::Block {
    pub fn visit_semantic(self, builder: &mut FunctionBuilder, context: &BackendContext) -> Value {
        for stmt in self.body {
            stmt.visit_semantic(builder, context);
        }

        self.trailing.visit_semantic(builder, context)
    }
}

impl semantic::Statement {
    pub fn visit_semantic(self, builder: &mut FunctionBuilder, context: &BackendContext) {
        match self {
            Self::LetBinding {
                value, symbol_id, ..
            } => {
                let ty = ty_to_type(value.ty, context);

                let symbol_id = symbol_id.into();
                let value = value.visit_semantic(builder, context);

                builder.declare_var(symbol_id, ty);
                builder.def_var(symbol_id, value);
            }
            Self::SideEffect(expr) => {
                expr.visit_semantic(builder, context);
            }
        }
    }
}
