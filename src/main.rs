use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

// Print found pattern matches
fn find_matches(reader: impl BufRead, writer: &mut impl Write, pattern: &str) -> Result<()> {
    for line in reader.lines() {
        let line = line?;

        if line.contains(pattern) {
            writeln!(writer, "{line}")?;
        }
    }

    Ok(())
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

    // Find and write found matches
    find_matches(reader, &mut writer, &args.pattern)
        .with_context(|| format!("Could not write matches to stdout"))?;

    Ok(())
}
