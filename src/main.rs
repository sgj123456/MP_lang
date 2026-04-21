use mp_lang::{format_code, run_file, run_repl};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "--format" || args[1] == "-f" {
            if args.len() > 2 {
                let source = fs::read_to_string(&args[2])?;
                match format_code(&source) {
                    Ok(formatted) => print!("{}", formatted),
                    Err(e) => eprintln!("Format error: {}", e),
                }
            } else {
                eprintln!("Usage: mp --format <file>");
            }
            return Ok(());
        }
        run_file(&args[1])?;
        return Ok(());
    }

    run_repl()
}
