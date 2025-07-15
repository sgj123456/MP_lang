// 抽象语法树节点定义

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Boolean(bool),
    String(String),
    Variable(String),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Block(Vec<Stmt>),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    UnaryOp {
        op: Token,
        expr: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        value: Expr,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Result(Expr),
    Return(Option<Expr>),
}
