use std::{io::{self, BufRead, Lines, BufReader}, path::Path, fs::File, env, process::ExitCode};

use assembl_really as asmr;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: An input file must be specified.");
        return ExitCode::FAILURE;
    }

    // Get the input asmr file
    let file_path = &args[1];

    // Read the file by line
    let lines = read_lines(file_path.trim());
    if let Err(e) = lines {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    // Parse the file into an AST
    let ast = asmr::parse_lines(lines.unwrap().map(|r| r.unwrap()));
    if let Err(ref e) = ast {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    // Execute the parsed AST
    let exit_code = asmr::execute(ast.unwrap());
    if let Err(ref e) = exit_code {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    exit_code.unwrap()
}

/// Reads the specified file by line with a BufReader.
fn read_lines<'a, P>(file_path: P) -> Result<Lines<BufReader<File>>, &'a str>
where
    P: AsRef<Path>
{
    let file = File::open(file_path);
    if let Err(_) = file {
        return Err("There was an error opening the file.");
    }

    Ok(io::BufReader::new(file.unwrap()).lines())
}
