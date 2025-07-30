use mp_lang::{run_file, run_repl};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_file(&args[1])?;
        return Ok(());
    }

    run_repl()
}
