pub use super::*;

impl SemanticVisitor for semantic::Literal {
    type Param = ();
    type Output = Value;

    fn visit_semantic(
        self,
        builder: &mut FunctionBuilder,
        _: &BackendContext,
        _: Self::Param,
    ) -> Self::Output {
        match self {
            Self::I8(num) => builder.ins().iconst(types::I8, num as i64),
            Self::I16(num) => builder.ins().iconst(types::I16, num as i64),
            Self::I32(num) => builder.ins().iconst(types::I32, num as i64),
            Self::I64(num) => builder.ins().iconst(types::I32, num),
            Self::U8(num) => builder.ins().iconst(types::I8, num as i64),
            Self::U16(num) => builder.ins().iconst(types::I16, num as i64),
            Self::U32(num) => builder.ins().iconst(types::I32, num as i64),
            Self::U64(num) => builder.ins().iconst(types::I64, num as i64),
            Self::F32(num) => builder.ins().f32const(num),
            Self::F64(num) => builder.ins().f64const(num),
        }
    }
}

impl SemanticVisitor for semantic::PrefixOperator {
    type Param = (semantic::Ty, Value);
    type Output = Value;

    fn visit_semantic(
        self,
        builder: &mut FunctionBuilder,
        _: &BackendContext,
        (ty, value): Self::Param,
    ) -> Self::Output {
        use semantic::Ty::*;
        match ty {
            I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 => match self {
                Self::Negate => builder.ins().ineg(value),
            },
            F32 | F64 => match self {
                Self::Negate => builder.ins().fneg(value),
            },
        }
    }
}

impl SemanticVisitor for semantic::InfixOperator {
    type Param = (semantic::Ty, Value, Value);
    type Output = Value;

    fn visit_semantic(
        self,
        builder: &mut FunctionBuilder,
        _: &BackendContext,
        (ty, left, right): Self::Param,
    ) -> Self::Output {
        use semantic::Ty::*;
        match ty {
            I8 | I16 | I32 | I64 => match self {
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
            U8 | U16 | U32 | U64 => match self {
                Self::Add => builder.ins().iadd(left, right),
                Self::Subtract => builder.ins().isub(left, right),
                Self::Multiply => builder.ins().imul(left, right),
                Self::Divide => builder.ins().udiv(left, right),

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
            F32 | F64 => match self {
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
