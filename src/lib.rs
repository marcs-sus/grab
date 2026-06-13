use anyhow::{Ok, Result};
use std::io::{BufRead, Write};

// Print found pattern matches
pub fn find_matches(reader: impl BufRead, writer: &mut impl Write, pattern: &str) -> Result<()> {
    for line in reader.lines() {
        let line = line?;

        if line.contains(pattern) {
            writeln!(writer, "{line}")?;
        }
    }

    Ok(())
}
