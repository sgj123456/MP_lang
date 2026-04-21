#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use mp_lang::{
        lexer::tokenize_with_errors,
        parser::parse,
        runtime::{
            environment::value::{Number, Value},
            eval::eval,
        },
    };

    #[test]
    fn test_number_eval() {
        let (tokens, errors) = tokenize_with_errors("123");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(123)));
    }

    #[test]
    fn test_binary_op_eval() {
        let (tokens, errors) = tokenize_with_errors("1 + 2 * 3");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(7)));
    }

    #[test]
    fn test_variable_eval() {
        let (tokens, errors) = tokenize_with_errors("let x = 5; x + 3");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(8)));
    }

    #[test]
    fn test_if_expr_eval() {
        let (tokens, errors) = tokenize_with_errors("if 1 < 2 {3} else {4}");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(3)));
    }

    #[test]
    fn test_undefined_variable() {
        let (tokens, errors) = tokenize_with_errors("x;");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_operation() {
        let (tokens, errors) = tokenize_with_errors("true + 1;");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let (tokens, errors) = tokenize_with_errors("if 1 + true {2} else {3}");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_unary_op() {
        let (tokens, errors) = tokenize_with_errors("-true");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_unsupported_expression() {
        let (tokens, errors) = tokenize_with_errors("unsupported");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_block_expr_eval() {
        let (tokens, errors) = tokenize_with_errors("{ let x = 1; x + 2 }");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(3)));
    }

    #[test]
    fn test_nested_block_scope() {
        let (tokens, errors) = tokenize_with_errors("{ let x = 1; { let x = 2; x } }");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(2)));
    }

    #[test]
    fn test_while_loop() {
        let (tokens, errors) = tokenize_with_errors("{ let x = 0; while x < 3 { x = x + 1 } }");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(
            result,
            Value::Array(Rc::new(RefCell::new(vec![
                Value::Number(Number::Int(1)),
                Value::Number(Number::Int(2)),
                Value::Number(Number::Int(3))
            ])))
        );
    }

    #[test]
    fn test_while_with_condition_false() {
        let (tokens, errors) = tokenize_with_errors("while false { 1 };");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_nested_while_loops() {
        let (tokens, errors) = tokenize_with_errors(
            "{
            let x = 0;
            let y = 0;
            while x < 2 {
                x = x + 1;
                while y < 3 {
                    y = y + 1;
                }
            };
            y
        }",
        );
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(3)))
    }

    #[test]
    fn test_vector_operations() {
        let (tokens, errors) = tokenize_with_errors("let v = [1, 2, 3]; push(v, 4); pop(v)");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(4)));
    }

    #[test]
    fn test_function_return() {
        let (tokens, errors) = tokenize_with_errors("fn add(a, b) { return a + b; }; add(2, 3)");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(5)));
    }

    #[test]
    fn test_early_return() {
        let (tokens, errors) = tokenize_with_errors("fn test() { return 10; 20; }; test()");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(10)));
    }

    #[test]
    fn test_array_index_access() {
        let (tokens, errors) = tokenize_with_errors("let arr = [10, 20, 30]; arr[1]");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(20)));
    }

    #[test]
    fn test_object_property_access() {
        let (tokens, errors) =
            tokenize_with_errors("let obj = {\"name\": \"John\", \"age\": 30}; obj:age");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(30)));
    }

    #[test]
    fn test_builtin_len() {
        let (tokens, errors) = tokenize_with_errors("len(\"hello\")");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(5)));
    }

    #[test]
    fn test_builtin_type() {
        let (tokens, errors) = tokenize_with_errors("type(123)");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::String("int".to_string()));
    }

    #[test]
    fn test_builtin_str() {
        let (tokens, errors) = tokenize_with_errors("str(42)");
        assert!(errors.is_empty());
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::String("42".to_string()));
    }

    #[test]
    fn test_examples() {
        use std::fs;
        use std::path::Path;

        let examples_dir = Path::new("examples");
        let mut example_files: Vec<_> = fs::read_dir(examples_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                path.extension().map_or(false, |ext| ext == "mp")
            })
            .collect();

        example_files.sort_by_key(|entry| entry.path());

        for entry in example_files {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            println!("Testing: {}", file_name);

            let content = fs::read_to_string(&path).unwrap();
            let (tokens, errors) = tokenize_with_errors(&content);
            assert!(errors.is_empty());
            let ast = parse(tokens);
            let result = eval(ast);

            match result {
                Ok(_) | Err(mp_lang::InterpreterError::Return(_)) => {
                    println!("  ✓ {} passed", file_name);
                }
                Err(e) => {
                    panic!("  ✗ {} failed: {:?}", file_name, e);
                }
            }
        }
    }
}
