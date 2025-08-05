use crate::{lexer::TokenKind, runtime::environment::value::Number};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(Number),
    Boolean(bool),
    String(String),
    Variable(String),
    Array(Vec<Expr>),
    Object(Vec<(String, Expr)>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Block(Vec<Stmt>),
    BinaryOp {
        left: Box<Expr>,
        op: TokenKind,
        right: Box<Expr>,
    },
    UnaryOp {
        op: TokenKind,
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
        body: Expr,
    },
    Result(Expr),
    Return(Option<Expr>),
}
