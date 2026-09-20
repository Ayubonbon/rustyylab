use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut filepath = String::new();
    let mut query = String::new();

    println!("Enter file path:");
    io::stdin().read_line(&mut filepath)?;

    println!("Enter word to search:");
    io::stdin().read_line(&mut query)?;

    let filepath = filepath.trim();
    let query = query.trim();

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if line.contains(query) {
            println!("{}", line);
        }
    }

    Ok(())
}
