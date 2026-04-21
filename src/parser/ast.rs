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
    StructInstance {
        name: String,
        args: Vec<Expr>,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn children(&self) -> Vec<&Expr> {
        let mut children = Vec::new();
        match &self.kind {
            ExprKind::Number(_)
            | ExprKind::Boolean(_)
            | ExprKind::String(_)
            | ExprKind::Variable(_) => {}
            ExprKind::Array(items) => children.extend(items),
            ExprKind::Object(fields) => children.extend(fields.iter().map(|(_, v)| v)),
            ExprKind::Parenthesized(expr) => children.push(expr),
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                children.push(condition);
                children.push(then_branch);
                if let Some(else_b) = else_branch {
                    children.push(else_b);
                }
            }
            ExprKind::Block(stmts) => {
                for stmt in stmts {
                    if let StmtKind::Expr(expr) = stmt {
                        children.push(expr);
                    }
                }
            }
            ExprKind::BinaryOp { left, right, .. } => {
                children.push(left);
                children.push(right);
            }
            ExprKind::UnaryOp { expr, .. } => {
                children.push(expr);
            }
            ExprKind::FunctionCall { args, .. } => {
                children.extend(args);
            }
            ExprKind::While { condition, body } => {
                children.push(condition);
                children.push(body);
            }
            ExprKind::Index { object, index } => {
                children.push(object);
                children.push(index);
            }
            ExprKind::GetProperty { object, .. } => {
                children.push(object);
            }
            ExprKind::StructInstance { args, .. } => {
                children.extend(args);
            }
        }
        children
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
        name_span: Span,
        value: Expr,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
    Struct {
        name: String,
        fields: Vec<(String, Option<Expr>)>,
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
