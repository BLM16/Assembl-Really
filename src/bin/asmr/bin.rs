use std::{io::{self, BufRead}, path::Path, fs::File, env};

use assembl_really as asmr;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: An input file must be specified.");
        return;
    }

    // Get the input asmr file
    let file_path = &args[1];

    // Read the file by line
    let lines = read_lines(file_path.trim()).unwrap()
                    .filter(|r| r.is_ok())
                    .map(|r| r.unwrap());
                
    // Parse the file into an AST
    let ast = asmr::parse_lines(lines).unwrap();

    println!("{:#?}", ast);
}

/// Reads the specified file by line with a BufReader.
fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
