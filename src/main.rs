use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("--- Rusty-Search v0.1 ---");
    println!("Suche nach '{}' in: {:?}\n", args.pattern, args.path);

    let entries = WalkDir::new(&args.path);

    entries.into_iter().par_bridge().for_each(|entry| {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return,
        };

        if !entry.file_type().is_file() {
            return;
        }

        if let Err(e) = search_in_file(entry.path(), &args.pattern, args.ignore_case) {
            eprintln!("Fehler in Datei {:?}: {}", entry.path(), e);
        }
    });

    Ok(())
}

fn search_in_file(path: &Path, pattern: &str, ignore_case: bool) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let check_pattern = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line_num = index + 1;

        let check_line = if ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        if check_line.contains(&check_pattern) {
            println!(
                "{}:{}:{}",
                path.display().to_string().magenta(),
                line_num.to_string().green(),
                line
            );
        }
    }

    Ok(())
}
