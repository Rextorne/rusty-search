use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::Colorize;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

mod algorithms;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    #[arg(short, long)]
    ignore_case: bool,
    #[arg(short, long, value_enum, default_value_t = Algo::Regex)]
    algo: Algo,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algo {
    Regex,
    Boyer,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("--- Rusty-Search v1.1.0 ---");
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

        if let Err(e) = search_in_file(entry.path(), &args) {
            eprintln!("Fehler in Datei {:?}: {}", entry.path(), e);
        }
    });

    Ok(())
}

fn search_in_file(path: &Path, args: &Cli) -> Result<()> {
    let ignore_case = args.ignore_case;
    let pattern = &args.pattern;
    let algorithm = args.algo;

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

        let found = match algorithm {
            Algo::Regex => {
                let check_line = if ignore_case {
                    line.to_lowercase()
                } else {
                    line.clone()
                };
                check_line.contains(&check_pattern)
            }

            Algo::Boyer => algorithms::boyer_moore_contains(line.as_bytes(), pattern.as_bytes()),
        };
        if found {
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
