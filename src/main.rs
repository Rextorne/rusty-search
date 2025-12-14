use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("--- Rusty-Search v0.1 ---");
    println!("Suche nach '{}' in: {:?}\n", args.pattern, args.path);

    for entry in WalkDir::new(&args.path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().is_file() {
            search_in_file(entry.path(), &args.pattern)?;
        }
    }

    Ok(())
}

fn search_in_file(path: &Path, pattern: &str) -> Result<()> {
    let file = File::open(path).expect("Unable to open file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            println!("Found pattern in File '{:?}'", path);
        }
    }

    Ok(())
}
