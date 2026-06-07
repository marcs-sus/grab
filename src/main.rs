use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // Fetch arguments
    let args = Cli::parse();

    // Open file
    let f = File::open(&args.path)
        .with_context(|| format!("Could not open file `{}`", args.path.display()))?;

    // Create reading and writing buffers
    let reader = BufReader::new(f);
    let mut writer = BufWriter::new(io::stdout().lock());

    // Write found lines
    for line in reader.lines() {
        let line =
            line.with_context(|| format!("Could not read line from `{}`", args.path.display()))?;

        if line.contains(&args.pattern) {
            writeln!(writer, "{line}")?;
        }
    }

    Ok(())
}
