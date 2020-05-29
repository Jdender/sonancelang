use sonancelang::compile;
use std::{env::current_dir, fs::File, io::prelude::*};

fn main() {
    let program = r"
        22. * 44. + 66.
    ";

    let compiled = compile(program).unwrap();

    let path = current_dir().unwrap().join("output/num.wasm");

    File::create(&path)
        .unwrap()
        .write_all(&compiled[..])
        .unwrap();
}
