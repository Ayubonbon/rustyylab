use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> std::io::Result<()> {

    println!(
        r#"
  ██████╗ ██████╗ ███████╗██████╗ ██╗   ██╗
 ██╔════╝ ██╔══██╗██╔════╝██╔══██╗╚██╗ ██╔╝
 ██║  ███╗██████╔╝█████╗  ██████╔╝ ╚████╔╝
 ██║   ██║██╔══██╗██╔══╝  ██╔═══╝   ╚██╔╝
 ╚██████╔╝██║  ██║███████╗██║        ██║
  ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝        ╚═╝

              Welcome to Grepy
        A tiny grep-like tool written in Rust
"#
    );

    println!("Usage:");
    println!("  <filepath> <word> [options]");
    println!("(please do not be a bitch and only write the filename , provide a complete file path)");

    println!("Options:");
    println!("  -i    Ignore uppercase/lowercase");
    println!("  -n    Show line numbers");
    println!("  -v    Show lines that do NOT match");
    println!();

    println!("Example:");
    println!("  notes.txt rust -i -n");
    println!();

    print!("grepy> ");

        use std::io::Write;
    io::stdout().flush()?;

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() < 2 {
        println!("Error: please provide a file and a search word.");
        return Ok(());
    }

    let filepath = args[0];
    let query = args[1];

    let ignore_case = args.contains(&"-i");
    let show_line_numbers = args.contains(&"-n");
    let invert_match = args.contains(&"-v");

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {

        let line = line?;

        let matches = if ignore_case {
            line.to_lowercase()
                .contains(&query.to_lowercase())
        } else {
            line.contains(query)
        };

        let should_print = if invert_match {
            !matches
        } else {
            matches
        };

        if should_print {

            if show_line_numbers {
                println!("{}: {}", line_number + 1, line);
            } else {
                println!("{}", line);
            }

        }
    }

    Ok(())
}
