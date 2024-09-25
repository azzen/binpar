use std::env;
use std::fs::File;
use std::io::{self, Read};

use pattern::BinaryPattern;
use thiserror::Error;

mod pattern;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Standard Error")]
    StdError(#[from] Box<dyn std::error::Error>),
}

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <file_path> <pattern>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let pattern: BinaryPattern = args[2].parse()?;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let matches = pattern.search(&buffer);

    if matches.is_empty() {
        println!("Pattern not found");
    } else {
        println!("Pattern found at offsets: {:#04X?}", matches);
    }

    Ok(())
}
