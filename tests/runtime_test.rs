#[cfg(test)]
mod tests {
    use mp_lang::{
        lexer::tokenize,
        parser::parse,
        runtime::{
            environment::{value::Value, Environment},
            eval::{eval, eval_with_env},
        },
    };

    #[test]
    fn test_number_eval() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(123.0));
    }

    #[test]
    fn test_binary_op_eval() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_variable_eval() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0));

        let tokens = tokenize("x + 3").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval_with_env(ast, &mut env).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_if_expr_eval() {
        let tokens = tokenize("if 1 < 2 {3} else {4}").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_undefined_variable() {
        let tokens = tokenize("x;").unwrap();
        let ast = parse(tokens).unwrap();
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_operation() {
        let tokens = tokenize("true + 1;").unwrap();
        let ast = parse(tokens).unwrap();
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let tokens = tokenize("if 1 + true {2} else {3}").unwrap();
        let ast = parse(tokens).unwrap();
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_unary_op() {
        let tokens = tokenize("-true").unwrap();
        let ast = parse(tokens).unwrap();
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_unsupported_expression() {
        let tokens = tokenize("unsupported").unwrap();
        let ast = parse(tokens).unwrap();
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_block_expr_eval() {
        let tokens = tokenize("{ let x = 1; x + 2 }").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_nested_block_scope() {
        let tokens = tokenize("{ let x = 1; { let x = 2; x } }").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_while_loop() {
        let tokens = tokenize("{ let x = 0; while x < 3 { x = x + 1 } }").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(
            result,
            Value::Vector(Vec::from([
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
        );
    }

    #[test]
    fn test_while_with_condition_false() {
        let tokens = tokenize("while false { 1 };").unwrap();
        let ast = parse(tokens).unwrap();
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
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0))
    }

    #[test]
    fn test_vector_operations() {
        let tokens = tokenize("let v = vector(1, 2, 3); push(v, 4); pop(v)").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_function_return() {
        let tokens = tokenize("fn add(a, b) { return a + b; }; add(2, 3)").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_early_return() {
        let tokens = tokenize("fn test() { return 10; 20; }; test()").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(10.0));
    }
}
