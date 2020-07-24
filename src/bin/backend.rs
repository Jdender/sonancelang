use sonancelang::backend::*;

fn main() {
    println!(
        "{}",
        Function {
            name: "test".to_string(),
            id: FunctionId(0),
            arguments: vec![Type::I32, Type::I32],
            result: Some(Type::I32),
            body: vec![BasicBlock {
                id: BlockId(0),
                arguments: vec![(VariableId(0), Type::I32), (VariableId(1), Type::I32)],
                instructions: vec![],
                terminator: Terminator::Return { argument: None }
            }],
        }
    );
}
