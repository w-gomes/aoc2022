use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("inputs/day?.txt")?;
    let file = File::open("inputs/day?.txt")?;
    let file = BufReader::new(file);

    Ok(())
}
