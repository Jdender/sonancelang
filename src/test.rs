use crate::compile;
use wasmer_runtime::{imports, instantiate, DynFunc, Value};

fn compile_and_run(input: &str) -> i32 {
    let import_object = imports! {};
    let wasm = compile(input).unwrap().wasm.to_bytes().unwrap();
    let instance = instantiate(&wasm, &import_object).unwrap();

    let values = instance
        .exports
        .get::<DynFunc>("main")
        .unwrap()
        .call(&[])
        .unwrap();

    if let Value::I32(num) = values[0] {
        num
    } else {
        Err(()).unwrap()
    }
}

fn process_int_helper(cases: Vec<(i32, &str)>) {
    for (num, input) in cases {
        assert_eq!(
            num,
            compile_and_run(&format!(
                "func main() -> I32 {{
                    return {};
                }}",
                input
            ))
        );
    }
}

#[test]
fn process_int_test() {
    process_int_helper(vec![
        (12345, "12345"),
        (-308, "20 + 56 - 32 * 72 / 6"),
        (1, "123 || 0"),
        (456, "123 && 456"),
        (0, "!0 && !1"),
    ]);
}
