pub mod lexer;
pub mod parser;
pub mod runtime;

pub use runtime::environment::{BuiltinFunction, Environment, UserFunction, Value};
pub use runtime::error::InterpreterError;

use rustyline::{
    Completer, Config, Editor, Helper, Highlighter, Hinter, Validator, error::ReadlineError,
    highlight::MatchingBracketHighlighter, history::FileHistory,
    validate::MatchingBracketValidator,
};
use std::{fs, result::Result};

use crate::runtime::eval::eval_with_env;

pub fn run_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let mut env = Environment::new();
    let tokens = lexer::tokenize(&content)?;
    let ast = parser::parse(tokens)?;
    let result = eval_with_env(ast, &mut env);
    match result {
        Ok(value) | Err(InterpreterError::Return(value)) => {
            println!("=> {value:?}")
        }
        Err(e) => eprintln!("Execution error: {e}"),
    }
    Ok(())
}

pub fn handle_command(cmd: &str, env: &mut Environment) -> bool {
    match cmd {
        "exit" => return false,
        "help" => {
            println!("Available commands:");
            println!("  exit     - exit the program");
            println!("  help     - display this help message");
            println!("  clear    - clear the environment");
        }
        "clear" => {
            *env = Environment::new();
            println!("Environment cleared.");
        }
        _ => match lexer::tokenize(cmd) {
            Ok(tokens) => {
                let ast = match parser::parse(tokens) {
                    Ok(ast) => ast,
                    Err(e) => {
                        eprintln!("Grammar error: {e}");
                        return true;
                    }
                };
                match eval_with_env(ast, env) {
                    Ok(result) => println!("=> {result:?}"),
                    Err(e) => eprintln!("Execution error: {e}"),
                }
            }
            Err(e) => eprintln!("Lexical error: {e}"),
        },
    }
    true
}

#[derive(Helper, Completer, Highlighter, Validator, Hinter)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
    #[rustyline(Highlighter)]
    hightlighter: MatchingBracketHighlighter,
}

pub fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Mp Lang! (type 'help' for help)");
    let config = Config::builder().auto_add_history(true).build();
    let mut rl: Editor<InputValidator, FileHistory> = Editor::with_config(config)?;
    rl.set_helper(Some(InputValidator {
        brackets: MatchingBracketValidator::new(),
        hightlighter: MatchingBracketHighlighter::new(),
    }));
    let mut env = Environment::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed)?;

                if !handle_command(trimmed, &mut env) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Using `Ctrl-D` to exit.");
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Read error: {err:?}");
                break;
            }
        }
    }

    Ok(())
}
