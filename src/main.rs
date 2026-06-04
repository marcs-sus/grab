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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let f = File::open(&args.path)?;
    let mut reader = BufReader::new(f);

    let mut content = String::new();
    let _len = reader.read_to_string(&mut content)?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }

    Ok(())
}
