use std::{fs, io};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn open_puzzle_input(file_name: &str) -> Result<io::BufReader<fs::File>> {
    let f = fs::File::open(format!("puzzle_inputs/{}", file_name))?;
    Ok(io::BufReader::new(f))
}
