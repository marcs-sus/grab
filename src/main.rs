use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let f = File::open(&args.path)
        .with_context(|| format!("Could not open file `{}`", args.path.display()))?;
    let mut reader = BufReader::new(f);

    let mut content = String::new();
    let _len = reader
        .read_to_string(&mut content)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }

    Ok(())
}
