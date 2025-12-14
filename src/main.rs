use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "src/test.txt";
    let word = "Rust";

    println!("--- Rusty-Search v0.1 ---");
    println!("Suche nach '{}' in '{}'", word, path);

    let file = File::open(path).expect("Unable to open file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(word) {
            println!("Found word in Line '{}'", line);
        }
    }

    Ok(())
}
