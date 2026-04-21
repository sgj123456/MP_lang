pub mod formatter;
pub mod lexer;
pub mod lsp;
pub mod parser;
pub mod runtime;

pub use formatter::format_code;
pub use lsp::MpLanguageServer;
pub use runtime::environment::{BuiltinFunction, Environment, UserFunction, Value};
pub use runtime::error::InterpreterError;

use rustyline::{
    Completer, Config, Editor, Helper, Highlighter, Hinter, Validator, error::ReadlineError,
    highlight::MatchingBracketHighlighter, history::FileHistory,
    validate::MatchingBracketValidator,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, result::Result};

pub fn run_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let tokens = lexer::tokenize(&content)?;
    let ast = parser::parse(tokens);
    let result = runtime::eval::eval(ast);
    match result {
        Ok(_) | Err(InterpreterError::Return(_)) => {}
        Err(e) => eprintln!("Execution error: {e}"),
    }
    Ok(())
}

pub fn handle_command(cmd: &str, env: &Rc<RefCell<Environment>>) -> bool {
    match cmd {
        "exit" => return false,
        "help" => {
            println!("Available commands:");
            println!("  exit     - exit the program");
            println!("  help     - display this help message");
            println!("  clear    - clear the environment");
        }
        "clear" => {
            println!("Environment cleared.");
        }
        _ => match lexer::tokenize(cmd) {
            Ok(tokens) => {
                let ast = parser::parse(tokens);
                match runtime::eval::eval_with_env(ast, env) {
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
    let env = Rc::new(RefCell::new(Environment::new_root()));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed)?;
                if !handle_command(trimmed, &env) {
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
