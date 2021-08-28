use structopt::StructOpt;

use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Cli {
    name: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    for line in content.lines() {
        if line.contains(&args.name) {
            println!("{}", line);
        }
    }
    Ok(())
}
