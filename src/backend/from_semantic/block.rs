pub use super::*;

impl SemanticVisitor for semantic::Block {
    type Param = ();
    type Output = Value;

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        context: &BackendContext,
        _: Self::Param,
    ) -> Self::Output {
        for stmt in self.body.iter() {
            stmt.visit_semantic(builder, context, ());
        }

        self.trailing.visit_semantic(builder, context, ())
    }
}

impl SemanticVisitor for semantic::Statement {
    type Param = ();
    type Output = ();

    fn visit_semantic(
        &self,
        builder: &mut FunctionBuilder,
        context: &BackendContext,
        _: Self::Param,
    ) -> Self::Output {
        match self {
            Self::LetBinding {
                value, symbol_id, ..
            } => {
                let ty = value.ty.into();

                let symbol_id = symbol_id.into();
                let value = value.visit_semantic(builder, context, ());

                builder.declare_var(symbol_id, ty);
                builder.def_var(symbol_id, value);
            }
            Self::SideEffect(expr) => {
                expr.visit_semantic(builder, context, ());
            }
        }
    }
}
