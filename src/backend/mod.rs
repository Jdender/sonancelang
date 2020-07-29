pub mod from_semantic;

use super::semantic;
use cranelift::{codegen::binemit::NullTrapSink, prelude::*};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBackend, ObjectBuilder};
use from_semantic::SemanticVisitor;

pub struct Backend {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Module<ObjectBackend>,
}

impl Backend {
    pub fn new() -> Result<Self, String> {
        let isa = isa::lookup(target_lexicon::HOST)
            .map_err(|e| e.to_string())?
            .finish(settings::Flags::new(settings::builder()));

        let builder = ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())
            .map_err(|e| e.to_string())?;

        let module = Module::new(builder);

        Ok(Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
        })
    }

    pub fn compile_func(mut self, input: semantic::File) -> Result<Vec<u8>, String> {
        let mut builder: FunctionBuilder =
            FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        input.visit_semantic(&mut builder, ());

        let mut return_sig = self.module.make_signature();
        return_sig.returns.push(AbiParam::new(types::I32));
        self.ctx.func.signature = return_sig;

        let func = self
            .module
            .declare_function(
                input.name.as_string(),
                Linkage::Export,
                &self.ctx.func.signature,
            )
            .map_err(|e| e.to_string())?;

        self.module
            .define_function(func, &mut self.ctx, &mut NullTrapSink {})
            .unwrap();

        self.module.clear_context(&mut self.ctx);

        self.module.finalize_definitions();

        Ok(self.module.finish().emit().map_err(|e| e.to_string())?)
    }
}
