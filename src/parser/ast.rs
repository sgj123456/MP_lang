use crate::lexer::{Span, TokenKind};
use crate::runtime::environment::value::Number;

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Number(Number),
    Boolean(bool),
    String(String),
    Variable(String),
    Array(Vec<Expr>),
    Object(Vec<(String, Expr)>),
    Parenthesized(Box<Expr>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Block(Vec<StmtKind>),
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
        body: Box<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    GetProperty {
        object: Box<Expr>,
        property: String,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
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
    Break,
    Continue,
    Result(Expr),
    Return(Option<Expr>),
}

impl Stmt {
    pub fn span(&self) -> Span {
        self.span
    }
}
