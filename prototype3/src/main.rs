use {
    anyhow::Result,
    clap::Clap,
    sonancelang::compile,
    std::{
        env::current_dir,
        fs::{read_to_string, OpenOptions},
        io::Write,
    },
};

#[derive(Clap)]
#[clap(version = "0.0")]
struct Options {
    input: String,
    output: String,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let cwd = current_dir()?;

    let input = read_to_string(cwd.join(options.input))?;
    let compiled = compile(&input)?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(options.output)?
        .write_all(&compiled)?;

    Ok(())
}
