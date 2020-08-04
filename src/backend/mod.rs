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
    pub fn new() -> Result<Self, BackendError> {
        let isa =
            isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));

        let builder =
            ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;

        let module = Module::new(builder);

        Ok(Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
        })
    }

    pub fn compile_func(mut self, input: semantic::File) -> Result<Vec<u8>, BackendError> {
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        input.visit_semantic(&mut builder, ());

        let mut return_sig = self.module.make_signature();
        return_sig.returns.push(AbiParam::new(input.ty.into()));
        self.ctx.func.signature = return_sig;

        let func = self.module.declare_function(
            input.name.as_string(),
            Linkage::Export,
            &self.ctx.func.signature,
        )?;

        self.module
            .define_function(func, &mut self.ctx, &mut NullTrapSink {})
            .unwrap();

        self.module.clear_context(&mut self.ctx);

        self.module.finalize_definitions();

        Ok(self.module.finish().emit()?)
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Failed to lookup instruction set")]
    Lookup(#[from] cranelift::codegen::isa::LookupError),
    #[error("Error while using cranelift Module")]
    Module(#[from] cranelift_module::ModuleError),
    #[error("Error while emitting object blob")]
    Object(#[from] cranelift_object::object::write::Error),
}
