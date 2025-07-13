// 简单编程语言实现

mod ast;
mod interpreter;
mod lexer;
mod parser;

use std::{
    env, fs,
    io::{self, Write},
    result::Result,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // 文件执行模式
        let filename = &args[1];
        let content = fs::read_to_string(filename)?;

        let mut env = interpreter::Environment::new();
        let tokens = lexer::tokenize(&content)?;
        let ast = parser::parse(tokens);
        let result = interpreter::eval_with_env(ast, &mut env)?;
        println!("执行结果: {result:?}");
        return Ok(());
    }

    println!("欢迎使用Mp语言! (输入help查看帮助)");
    let mut env = interpreter::Environment::new();

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim().to_owned();
        if trimmed.is_empty() {
            continue;
        }

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

        // 支持多行输入
        while trimmed.ends_with('\\') {
            print!(".. ");
            io::stdout().flush()?;
            input.pop(); // 移除末尾的\
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            input.push_str(&line);
        }

        match lexer::tokenize(&input) {
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

    Ok(())
}
