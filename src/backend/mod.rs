pub mod from_semantic;

use super::semantic;
use cranelift::{codegen::binemit::NullTrapSink, prelude::*};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBackend, ObjectBuilder};
use from_semantic::SemanticVisitor;
// use std::collections::HashMap;

pub fn backend_pass(file: semantic::File) -> Result<Vec<u8>, BackendError> {
    let mut module = create_module()?;

    let mut bodies = Vec::with_capacity(file.items.len());
    // let func_table = HashMap::with_capacity(file.items.len());

    for func in file.items {
        let mut signature = module.make_signature();
        signature.returns.push(AbiParam::new(func.head.ty.into()));

        let id = module.declare_function(
            func.head.name.as_string(),
            func.head.scope.into(),
            &signature,
        )?;

        bodies.push((id, signature, func));
        // func_table.insert(func.symbol_id, id);
    }

    let mut ctx = module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    for (id, signature, func) in bodies {
        ctx.func.signature = signature;

        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        func.visit_semantic(&mut builder, ());

        module
            .define_function(id, &mut ctx, &mut NullTrapSink {})
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
