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

fn compile_run_eq(num: i32, input: &str) {
    assert_eq!(num, compile_and_run(input));
}

#[test]
fn return_int_test() {
    compile_run_eq(
        1234,
        r"
             func main() -> I32 {
                return 1234;
            }
        ",
    );
}
