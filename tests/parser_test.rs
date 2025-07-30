#[cfg(test)]
mod tests {
    use mp_lang::{
        lexer::{TokenKind, tokenize},
        parser::ast::{Expr, Stmt},
        parser::parse,
    };

    #[test]
    fn test_number_expr() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast, vec![Stmt::Result(Expr::Number(123.0))]);
    }

    #[test]
    fn test_binary_op() {
        let tokens = tokenize("1 + 2").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: TokenKind::Plus,
                right: Box::new(Expr::Number(2.0))
            })]
        );
    }

    #[test]
    fn test_variable_decl() {
        let tokens = tokenize("let x = 5").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Let {
                name: "x".to_string(),
                value: Expr::Number(5.0)
            }]
        );
    }

    #[test]
    fn test_if_expr() {
        let tokens = tokenize("if 1 < 2 {3} else {4}").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::If {
                condition: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: TokenKind::LessThan,
                    right: Box::new(Expr::Number(2.0))
                }),
                then_branch: Box::new(Expr::Block(vec![Stmt::Result(Expr::Number(3.0))])),
                else_branch: Some(Box::new(Expr::Block(vec![Stmt::Result(Expr::Number(4.0))])))
            })]
        );
    }

    #[test]
    fn test_operator_precedence() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: TokenKind::Plus,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(2.0)),
                    op: TokenKind::Multiply,
                    right: Box::new(Expr::Number(3.0))
                })
            })]
        );
    }

    #[test]
    fn test_function_decl() {
        let tokens = tokenize("fn add(a, b) { a + b }").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Function {
                name: "add".to_string(),
                params: vec!["a".to_string(), "b".to_string()],
                body: Expr::Block(vec![Stmt::Result(Expr::BinaryOp {
                    left: Box::new(Expr::Variable("a".to_string())),
                    op: TokenKind::Plus,
                    right: Box::new(Expr::Variable("b".to_string()))
                })])
            }]
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = tokenize("add(1, 2)").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![Expr::Number(1.0), Expr::Number(2.0)]
            })]
        );
    }

    #[test]
    fn test_nested_function_call() {
        let tokens = tokenize("add(1, mul(2, 3))").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![
                    Expr::Number(1.0),
                    Expr::FunctionCall {
                        name: "mul".to_string(),
                        args: vec![Expr::Number(2.0), Expr::Number(3.0)]
                    }
                ]
            })]
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let tokens = tokenize("let x = 1; let y = 2").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![
                Stmt::Let {
                    name: "x".to_string(),
                    value: Expr::Number(1.0)
                },
                Stmt::Let {
                    name: "y".to_string(),
                    value: Expr::Number(2.0)
                }
            ]
        );
    }

    #[test]
    fn test_multiple_semicolons() {
        let tokens = tokenize("let x = 1;;; let y = 2").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![
                Stmt::Let {
                    name: "x".to_string(),
                    value: Expr::Number(1.0)
                },
                Stmt::Let {
                    name: "y".to_string(),
                    value: Expr::Number(2.0)
                }
            ]
        );
    }

    #[test]
    fn test_semicolon_after_expr() {
        let tokens = tokenize("1 + 2; 3 * 4").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![
                Stmt::Expr(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: TokenKind::Plus,
                    right: Box::new(Expr::Number(2.0))
                }),
                Stmt::Result(Expr::BinaryOp {
                    left: Box::new(Expr::Number(3.0)),
                    op: TokenKind::Multiply,
                    right: Box::new(Expr::Number(4.0))
                })
            ]
        );
    }
}
