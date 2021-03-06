pub mod from_semantic;

use {
    super::semantic::type_check as semantic,
    cranelift::{codegen::binemit::NullTrapSink, prelude::*},
    cranelift_module::{FuncId, Linkage, Module},
    cranelift_object::{ObjectBackend, ObjectBuilder},
    from_semantic::ty_to_type,
    std::collections::HashMap,
};

pub fn backend_pass(file: semantic::File) -> Result<Vec<u8>, BackendError> {
    let mut context = BackendContext::new(file.items.len())?;

    let mut ctx = context.module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    file.items
        .into_iter()
        .filter_map(|item| match item {
            semantic::Item::Declare(declare) => {
                for func in declare.functions {
                    let mut signature = context.module.make_signature();
                    signature
                        .returns
                        .push(AbiParam::new(ty_to_type(func.ty, &context)));

                    for arg in func.params.iter() {
                        signature
                            .params
                            .push(AbiParam::new(ty_to_type(arg.ty, &context)));
                    }

                    let id = context.module.declare_function(
                        func.name.as_string(),
                        Linkage::Import,
                        &signature,
                    );

                    let id = match id {
                        Ok(id) => id,
                        Err(e) => return Some(Err(e.into())),
                    };

                    context.func_table.insert(func.symbol_id, id);
                }
                None
            }

            semantic::Item::Function(func) => {
                let mut signature = context.module.make_signature();
                signature
                    .returns
                    .push(AbiParam::new(ty_to_type(func.ty, &context)));

                for arg in func.params.iter() {
                    signature
                        .params
                        .push(AbiParam::new(ty_to_type(arg.ty, &context)));
                }

                let id = context.module.declare_function(
                    func.name.as_string(),
                    func.scope.into(),
                    &signature,
                );

                let id = match id {
                    Ok(id) => id,
                    Err(e) => return Some(Err(e.into())),
                };

                context.func_table.insert(func.symbol_id, id);
                Some(Ok((id, signature, func)))
            }
        })
        .collect::<Result<Vec<_>, BackendError>>()?
        .into_iter()
        .for_each(|(id, signature, func)| {
            ctx.func.signature = signature;

            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

            func.visit_semantic(&mut builder, &context);

            context
                .module
                .define_function(id, &mut ctx, &mut NullTrapSink {})
                .unwrap();

            context.module.clear_context(&mut ctx);
        });

    context.module.finalize_definitions();

    Ok(context.module.finish().emit()?)
}

pub struct BackendContext {
    func_table: HashMap<semantic::SymbolId, FuncId>,
    module: Module<ObjectBackend>,
}

impl BackendContext {
    fn new(capacity: usize) -> Result<Self, BackendError> {
        let isa =
            isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));

        let builder =
            ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;

        Ok(Self {
            module: Module::new(builder),
            func_table: HashMap::with_capacity(capacity),
        })
    }
}

impl From<semantic::Scope> for Linkage {
    fn from(scope: semantic::Scope) -> Self {
        match scope {
            semantic::Scope::Local => Linkage::Local,
            semantic::Scope::Public => Linkage::Export,
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
