use sonancelang::compile;
use std::{env::current_dir, fs::File, io::prelude::*};

fn main() {
    let program = r"
        let t = 22.0 * 44.0 + 66.0;
        let a = 12 * 1 + 5;
        32 + 64
    ";

    let compiled = compile(program).unwrap();

    let path = current_dir().unwrap().join("output/num.wasm");

    File::create(&path)
        .unwrap()
        .write_all(&compiled[..])
        .unwrap();
}
