pub mod lexer;
pub mod parser;
pub mod runtime;

use rustyline::{Config, Editor, error::ReadlineError, history::FileHistory};
use std::{fs, result::Result};

use crate::runtime::{environment::Environment, error::InterpreterError, eval::eval_with_env};

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
        Err(e) => eprintln!("执行错误: {e}"),
    }
    Ok(())
}

pub fn handle_command(cmd: &str, env: &mut Environment) -> bool {
    match cmd {
        "exit" => return false,
        "help" => {
            println!("可用命令:");
            println!("  exit     - 退出交互模式");
            println!("  help     - 显示帮助信息");
            println!("  clear    - 清空环境变量");
            println!("  其他输入 - 执行Mp代码");
        }
        "clear" => {
            *env = Environment::new();
            println!("环境已清空");
        }
        _ => match lexer::tokenize(cmd) {
            Ok(tokens) => {
                let ast = match parser::parse(tokens) {
                    Ok(ast) => ast,
                    Err(e) => {
                        eprintln!("语法分析错误: {e}");
                        return true;
                    }
                };
                match eval_with_env(ast, env) {
                    Ok(result) => println!("=> {result:?}"),
                    Err(e) => eprintln!("执行错误: {e}"),
                }
            }
            Err(e) => eprintln!("词法分析错误: {e}"),
        },
    }
    true
}

pub fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("欢迎使用Mp语言! (输入help查看帮助)");
    let config = Config::builder().auto_add_history(true).build();
    let mut rl = Editor::<(), FileHistory>::with_config(config)?;
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
                println!("使用Ctrl-D退出");
            }
            Err(ReadlineError::Eof) => {
                println!("再见!");
                break;
            }
            Err(err) => {
                eprintln!("读取错误: {err:?}");
                break;
            }
        }
    }

    Ok(())
}
