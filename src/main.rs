// 简单编程语言实现

mod ast;
mod interpreter;
mod lexer;
mod parser;

use rustyline::{Config, Editor, error::ReadlineError, history::FileHistory};
use std::{env, fs, result::Result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // 文件执行模式
        let filename = &args[1];
        let content = fs::read_to_string(filename)?;

        let mut env = interpreter::Environment::new();
        let tokens = lexer::tokenize(&content)?;
        let ast = parser::parse(tokens);
        let result = interpreter::eval_with_env(ast, &mut env);
        match result {
            Ok(value) | Err(interpreter::InterpreterError::Return(value)) => {
                println!("=> {value:?}")
            }
            Err(e) => eprintln!("执行错误: {e}"),
        }
        return Ok(());
    }

    println!("欢迎使用Mp语言! (输入help查看帮助)");
    let config = Config::builder().auto_add_history(true).build();
    let mut rl = Editor::<(), FileHistory>::with_config(config)?;
    let mut env = interpreter::Environment::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim().to_owned();
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed.as_str())?;

                match trimmed.as_str() {
                    "exit" => break,
                    "help" => {
                        println!("可用命令:");
                        println!("  exit     - 退出交互模式");
                        println!("  help     - 显示帮助信息");
                        println!("  clear    - 清空环境变量");
                        println!("  其他输入 - 执行Mp代码");
                        continue;
                    }
                    "clear" => {
                        env = interpreter::Environment::new();
                        println!("环境已清空");
                        continue;
                    }
                    _ => {}
                }

                match lexer::tokenize(&line) {
                    Ok(tokens) => {
                        let ast = parser::parse(tokens);
                        match interpreter::eval_with_env(ast, &mut env) {
                            Ok(result) => println!("=> {result:?}"),
                            Err(e) => eprintln!("执行错误: {e}"),
                        }
                    }
                    Err(e) => eprintln!("词法分析错误: {e}"),
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
