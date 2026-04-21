#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use mp_lang::{
        lexer::tokenize,
        parser::parse,
        runtime::{
            environment::value::{Number, Value},
            eval::eval,
        },
    };

    #[test]
    fn test_number_eval() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(123)));
    }

    #[test]
    fn test_binary_op_eval() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(7)));
    }

    #[test]
    fn test_variable_eval() {
        let tokens = tokenize("let x = 5; x + 3").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(8)));
    }

    #[test]
    fn test_if_expr_eval() {
        let tokens = tokenize("if 1 < 2 {3} else {4}").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(3)));
    }

    #[test]
    fn test_undefined_variable() {
        let tokens = tokenize("x;").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_operation() {
        let tokens = tokenize("true + 1;").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let tokens = tokenize("if 1 + true {2} else {3}").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_unary_op() {
        let tokens = tokenize("-true").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_unsupported_expression() {
        let tokens = tokenize("unsupported").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_block_expr_eval() {
        let tokens = tokenize("{ let x = 1; x + 2 }").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(3)));
    }

    #[test]
    fn test_nested_block_scope() {
        let tokens = tokenize("{ let x = 1; { let x = 2; x } }").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(2)));
    }

    #[test]
    fn test_while_loop() {
        let tokens = tokenize("{ let x = 0; while x < 3 { x = x + 1 } }").unwrap();
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
        let tokens = tokenize("while false { 1 };").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_nested_while_loops() {
        let tokens = tokenize(
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
        )
        .unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(3)))
    }

    #[test]
    fn test_vector_operations() {
        let tokens = tokenize("let v = [1, 2, 3]; push(v, 4); pop(v)").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(4)));
    }

    #[test]
    fn test_function_return() {
        let tokens = tokenize("fn add(a, b) { return a + b; }; add(2, 3)").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(5)));
    }

    #[test]
    fn test_early_return() {
        let tokens = tokenize("fn test() { return 10; 20; }; test()").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(10)));
    }

    #[test]
    fn test_array_index_access() {
        let tokens = tokenize("let arr = [10, 20, 30]; arr[1]").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(20)));
    }

    #[test]
    fn test_object_property_access() {
        let tokens = tokenize("let obj = {\"name\": \"John\", \"age\": 30}; obj:age").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(30)));
    }

    #[test]
    fn test_builtin_len() {
        let tokens = tokenize("len(\"hello\")").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(Number::Int(5)));
    }

    #[test]
    fn test_builtin_type() {
        let tokens = tokenize("type(123)").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::String("int".to_string()));
    }

    #[test]
    fn test_builtin_str() {
        let tokens = tokenize("str(42)").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::String("42".to_string()));
    }
}
