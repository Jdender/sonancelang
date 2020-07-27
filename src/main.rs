use clap::Clap;
use sonancelang::compile;
use std::{env::current_dir, fs::read_to_string};

#[derive(Clap)]
#[clap(version = "0.0")]
struct Options {
    #[clap(default_value = "test/input.txt")]
    input: String,
}

fn main() {
    let options = Options::parse();
    let cwd = current_dir().expect("Failed to get current working dir.");

    let input = read_to_string(cwd.join(options.input)).expect("File not found.");
    let compiled = compile(&input).expect("Failed to compile");

    dbg!(compiled);
}
