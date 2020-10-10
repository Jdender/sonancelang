use {
    clap::Clap,
    sonancelang_prototype4::{run, Result},
    std::{env::current_dir, fs::read_to_string},
};

#[derive(Clap)]
#[clap(version = "0.0")]
struct Options {
    #[clap(default_value = "test/input.son")]
    input: String,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let cwd = current_dir()?;

    let input = read_to_string(cwd.join(options.input))?;
    run(&input)?;

    Ok(())
}
