use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
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

    println!("--- Rusty-Search v1.1.1 ---");
    println!("Suche nach '{}' in: {:?}\n", args.pattern, args.path);

    let files: Vec<_> = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let total_count = files.len() as u64;

    let pb = ProgressBar::new(total_count);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("#>-"),
    );

    files.par_iter().for_each(|entry| {
        if let Err(e) = search_in_file(entry.path(), &args, &pb) {
            pb.println(format!("Fehler in Datei {:?}: {}", entry.path(), e));
        }
        pb.inc(1);
    });

    pb.finish_with_message("Fertig!");

    Ok(())
}

fn search_in_file(path: &Path, args: &Cli, pb: &ProgressBar) -> Result<()> {
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
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                continue;
            }
        };
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
            pb.println(format!(
                "{}:{}:{}",
                path.display().to_string().magenta(),
                line_num.to_string().green(),
                line
            ));
        }
    }

    Ok(())
}
