use std::{io::{self, BufRead}, path::Path, fs::File, env};

use assembl_really::parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: An input file must be specified.");
        return;
    }

    // Get the input asmr file
    let file_path = &args[1];

    match read_lines(file_path.trim()) {
        Ok(file_lines) => {
            match parser::parse_lines(file_lines.filter(|r| r.is_ok()).map(|r| r.unwrap())) {
                Ok(lines) => {
                    println!("{:#?}", lines);
                },
                Err(e) => eprintln!("{}", e),
            }
        },
        Err(e) => eprintln!("{}", e),
    }
}

/// Reads the specified file by line with a BufReader.
fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
