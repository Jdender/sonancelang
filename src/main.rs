use clap::Clap;
use parity_wasm::serialize_to_file;
use sonancelang::compile;
use std::{
    env::current_dir,
    fs::{read_to_string, write},
};

#[derive(Clap)]
#[clap(version = "0.0")]
struct Options {
    #[clap(default_value = "test/input.txt")]
    input: String,

    #[clap(default_value = "test/output.wasm")]
    wasm_output: String,

    #[clap(default_value = "test/output.txt")]
    formatter_output: String,
}

fn main() {
    let options = Options::parse();
    let cwd = current_dir().expect("Failed to get current working dir.");

    let input = read_to_string(cwd.join(options.input)).expect("File not found.");
    let compiled = compile(&input).expect("Failed to compile");

    serialize_to_file(cwd.join(options.wasm_output), compiled.wasm)
        .expect("Failed to write to output file.");

    write(cwd.join(options.formatter_output), compiled.formatted)
        .expect("Failed to write to formatted file.");
}
