#[cfg(test)]
mod tests {
    use mp_lang::{
        lexer::{TokenKind, tokenize},
        parser::{ExprKind, StmtKind, parse},
        runtime::environment::value::Number,
    };

    #[test]
    fn test_number_expr() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::Number(Number::Int(123))));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_string_expr() {
        let tokens = tokenize("\"hello\"").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::String(s) if s == "hello"));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_boolean_expr() {
        let tokens = tokenize("true").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::Boolean(true)));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_array_expr() {
        let tokens = tokenize("[1, 2, 3]").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                if let ExprKind::Array(arr) = &expr.kind {
                    assert_eq!(arr.len(), 3);
                } else {
                    panic!("Expected Array");
                }
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_object_expr() {
        let tokens = tokenize("{\"a\": 1, \"b\": 2}").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                if let ExprKind::Object(obj) = &expr.kind {
                    assert_eq!(obj.len(), 2);
                } else {
                    panic!("Expected Object");
                }
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_binary_op() {
        let tokens = tokenize("1 + 2").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(
                    &expr.kind,
                    ExprKind::BinaryOp {
                        op: TokenKind::Plus,
                        ..
                    }
                ));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_variable_decl() {
        let tokens = tokenize("let x = 5").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Let { name, .. } => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_if_expr() {
        let tokens = tokenize("if 1 < 2 {3} else {4}").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::If { .. }));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_operator_precedence() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                if let ExprKind::BinaryOp { left: _, op, right } = &expr.kind {
                    assert!(matches!(op, TokenKind::Plus));
                    if let ExprKind::BinaryOp { op: mul_op, .. } = &right.kind {
                        assert!(matches!(mul_op, TokenKind::Multiply));
                    } else {
                        panic!("Expected nested BinaryOp");
                    }
                } else {
                    panic!("Expected BinaryOp");
                }
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_function_decl() {
        let tokens = tokenize("fn add(a, b) { a + b }").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Function { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params, &vec!["a".to_string(), "b".to_string()]);
            }
            _ => panic!("Expected Function statement"),
        }
    }

    #[test]
    fn test_function_call() {
        let tokens = tokenize("add(1, 2)").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::FunctionCall { name, .. } if name == "add"));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_nested_function_call() {
        let tokens = tokenize("add(1, multiply(2, 3))").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                if let ExprKind::FunctionCall { name, args, .. } = &expr.kind {
                    assert_eq!(name, "add");
                    assert_eq!(args.len(), 2);
                } else {
                    panic!("Expected FunctionCall");
                }
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_semicolon_separator() {
        let tokens = tokenize("let x = 1; let y = 2").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 2);
    }

    #[test]
    fn test_multiple_semicolons() {
        let tokens = tokenize("let x = 1;;; let y = 2").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 2);
    }

    #[test]
    fn test_semicolon_after_expr() {
        let tokens = tokenize("1 + 2; 3 * 4").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 2);
    }

    #[test]
    fn test_array_index_expression() {
        let tokens = tokenize("arr[0]").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::Index { .. }));
            }
            _ => panic!("Expected Result statement"),
        }
    }

    #[test]
    fn test_object_property_expression() {
        let tokens = tokenize("obj:name").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast.len(), 1);
        match &ast[0].kind {
            StmtKind::Result(expr) => {
                assert!(matches!(&expr.kind, ExprKind::GetProperty { .. }));
            }
            _ => panic!("Expected Result statement"),
        }
    }
}
