use clap::Clap;
use sonancelang::compile;
use std::{
    env::current_dir,
    fs::{read_to_string, OpenOptions},
    io::Write,
};

#[derive(Clap)]
#[clap(version = "0.0")]
struct Options {
    #[clap(default_value = "test/input.txt")]
    input: String,

    #[clap(default_value = "test/output.o")]
    output: String,
}

fn main() -> Result<(), String> {
    let options = Options::parse();
    let cwd = current_dir().map_err(|e| e.to_string())?;

    let input = read_to_string(cwd.join(options.input)).map_err(|e| e.to_string())?;
    let compiled = compile(&input)?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(options.output)
        .map_err(|e| e.to_string())?
        .write_all(&compiled)
        .map_err(|e| e.to_string())?;

    Ok(())
}
