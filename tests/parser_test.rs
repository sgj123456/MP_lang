#[cfg(test)]
mod tests {
    use mp_lang::{
        lexer::{TokenKind, tokenize},
        parser::{
            ast::{Expr, Stmt},
            parse,
        },
        runtime::environment::value::Number,
    };

    #[test]
    fn test_number_expr() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast, vec![Stmt::Result(Expr::Number(Number::Int(123)))]);
    }

    #[test]
    fn test_string_expr() {
        let tokens = tokenize("\"hello\"").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast, vec![Stmt::Result(Expr::String("hello".to_string()))]);
    }

    #[test]
    fn test_boolean_expr() {
        let tokens = tokenize("true").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast, vec![Stmt::Result(Expr::Boolean(true))]);
    }

    #[test]
    fn test_array_expr() {
        let tokens = tokenize("[1, 2, 3]").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::Array(vec![
                Expr::Number(Number::Int(1)),
                Expr::Number(Number::Int(2)),
                Expr::Number(Number::Int(3))
            ]))]
        );
    }

    #[test]
    fn test_object_expr() {
        let tokens = tokenize("{\"a\": 1, \"b\": 2}").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::Object(vec![
                ("a".to_string(), Expr::Number(Number::Int(1))),
                ("b".to_string(), Expr::Number(Number::Int(2)))
            ]))]
        );
    }

    #[test]
    fn test_binary_op() {
        let tokens = tokenize("1 + 2").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::BinaryOp {
                left: Box::new(Expr::Number(Number::Int(1))),
                op: TokenKind::Plus,
                right: Box::new(Expr::Number(Number::Int(2)))
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
                value: Expr::Number(Number::Int(5))
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
                    left: Box::new(Expr::Number(Number::Int(1))),
                    op: TokenKind::LessThan,
                    right: Box::new(Expr::Number(Number::Int(2)))
                }),
                then_branch: Box::new(Expr::Block(vec![Stmt::Result(Expr::Number(Number::Int(
                    3
                )))])),
                else_branch: Some(Box::new(Expr::Block(vec![Stmt::Result(Expr::Number(
                    Number::Int(4)
                ))])))
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
                left: Box::new(Expr::Number(Number::Int(1))),
                op: TokenKind::Plus,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(Number::Int(2))),
                    op: TokenKind::Multiply,
                    right: Box::new(Expr::Number(Number::Int(3)))
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
                args: vec![Expr::Number(Number::Int(1)), Expr::Number(Number::Int(2))]
            })]
        );
    }

    #[test]
    fn test_nested_function_call() {
        let tokens = tokenize("add(1, multiply(2, 3))").unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(
            ast,
            vec![Stmt::Result(Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![
                    Expr::Number(Number::Int(1)),
                    Expr::FunctionCall {
                        name: "multiply".to_string(),
                        args: vec![Expr::Number(Number::Int(2)), Expr::Number(Number::Int(3))]
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
                    value: Expr::Number(Number::Int(1))
                },
                Stmt::Let {
                    name: "y".to_string(),
                    value: Expr::Number(Number::Int(2))
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
                    value: Expr::Number(Number::Int(1))
                },
                Stmt::Let {
                    name: "y".to_string(),
                    value: Expr::Number(Number::Int(2))
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
                    left: Box::new(Expr::Number(Number::Int(1))),
                    op: TokenKind::Plus,
                    right: Box::new(Expr::Number(Number::Int(2)))
                }),
                Stmt::Result(Expr::BinaryOp {
                    left: Box::new(Expr::Number(Number::Int(3))),
                    op: TokenKind::Multiply,
                    right: Box::new(Expr::Number(Number::Int(4)))
                })
            ]
        );
    }
}
