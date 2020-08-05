pub mod from_semantic;

use super::semantic;
use cranelift::{codegen::binemit::NullTrapSink, prelude::*};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBackend, ObjectBuilder};
use from_semantic::SemanticVisitor;

pub fn backend_pass(file: semantic::File) -> Result<Vec<u8>, BackendError> {
    let mut module = create_module()?;

    let mut ctx = module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    for func in file.items {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        func.visit_semantic(&mut builder, ());

        let mut return_sig = module.make_signature();
        return_sig.returns.push(AbiParam::new(func.ty.into()));
        ctx.func.signature = return_sig;

        let func = module.declare_function(
            func.name.as_string(),
            func.scope.into(),
            &ctx.func.signature,
        )?;

        module
            .define_function(func, &mut ctx, &mut NullTrapSink {})
            .unwrap();

        module.clear_context(&mut ctx);
    }

    module.finalize_definitions();

    Ok(module.finish().emit()?)
}

pub fn create_module() -> Result<Module<ObjectBackend>, BackendError> {
    let isa = isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));

    let builder = ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;

    let module = Module::new(builder);

    Ok(module)
}

impl From<semantic::Scope> for Linkage {
    fn from(scope: semantic::Scope) -> Self {
        match scope {
            semantic::Scope::Local => Linkage::Local,
            semantic::Scope::Export => Linkage::Export,
        }
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
