use super::ast::*;
use cranelift::{codegen::binemit::NullTrapSink, prelude::*};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBackend, ObjectBuilder};

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Module<ObjectBackend>,
}

impl JIT {
    pub fn new() -> Result<Self, String> {
        let isa = isa::lookup(target_lexicon::HOST)
            .map_err(|e| e.to_string())?
            .finish(settings::Flags::new(settings::builder()));

        let builder = ObjectBuilder::new(isa, "test", cranelift_module::default_libcall_names())
            .map_err(|e| e.to_string())?;

        let module = Module::new(builder);

        Ok(Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
        })
    }

    pub fn compile(mut self, input: File) -> Result<Vec<u8>, String> {
        let mut return_sig = self.module.make_signature();
        return_sig.returns.push(AbiParam::new(types::I32));

        let func = self
            .module
            .declare_function(input.name.as_string(), Linkage::Export, &return_sig)
            .map_err(|e| e.to_string())?;

        self.ctx.func.signature = return_sig;
        self.ctx.func.name = ExternalName::user(0, func.as_u32());

        {
            let mut bcx: FunctionBuilder =
                FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let block = bcx.create_block();

            let Expression::Literal(num) = input.number;

            bcx.switch_to_block(block);
            bcx.append_block_params_for_function_params(block);
            let cst = bcx.ins().iconst(types::I32, i64::from(num));
            bcx.ins().return_(&[cst]);
            bcx.seal_all_blocks();
            bcx.finalize();
        }

        self.module
            .define_function(func, &mut self.ctx, &mut NullTrapSink {})
            .unwrap();
        self.module.clear_context(&mut self.ctx);

        self.module.finalize_definitions();

        Ok(self.module.finish().emit().map_err(|e| e.to_string())?)
    }
}
