#[cfg(test)]
mod tests {
    use mp_lang::lexer::{Span, TokenKind, tokenize};

    #[test]
    fn test_number() {
        let tokens = tokenize("123 45.67").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::Number(45.67));
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_boolean() {
        let tokens = tokenize("true false").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Boolean(true));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::Boolean(false));
        assert_eq!(tokens[1].span, Span { line: 1, column: 6 });
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("+ - * /").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[1].span, Span { line: 1, column: 3 });
        assert_eq!(tokens[2].kind, TokenKind::Multiply);
        assert_eq!(tokens[2].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[3].kind, TokenKind::Divide);
        assert_eq!(tokens[3].span, Span { line: 1, column: 7 });
        assert_eq!(tokens[4].kind, TokenKind::Eof);
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize("let if else").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::If);
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[2].kind, TokenKind::Else);
        assert_eq!(tokens[2].span, Span { line: 1, column: 8 });
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize("x y_z").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::Identifier("y_z".to_string()));
        assert_eq!(tokens[1].span, Span { line: 1, column: 3 });
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_strings() {
        let tokens = tokenize("\"hello\" \"world\\n\" \"say \\\"hi\\\"\"").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::String("hello".to_string()));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::String("world\n".to_string()));
        assert_eq!(tokens[1].span, Span { line: 1, column: 9 });
        assert_eq!(tokens[2].kind, TokenKind::String("say \"hi\"".to_string()));
        assert_eq!(
            tokens[2].span,
            Span {
                line: 1,
                column: 19
            }
        );
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        // 测试未闭合的字符串
        assert!(tokenize("\"unclosed").is_err());
    }

    #[test]
    fn test_comments() {
        // 测试单行注释
        let tokens = tokenize("// 这是一个注释\n123").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Comment(" 这是一个注释".into()));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[2].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[2].span, Span { line: 2, column: 1 });
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        // 测试注释后的代码
        let tokens = tokenize("123 // 数字\n+ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[3].kind, TokenKind::Plus);
        assert_eq!(tokens[3].span, Span { line: 2, column: 1 });
        assert_eq!(tokens[4].kind, TokenKind::Number(456.0));
        assert_eq!(tokens[4].span, Span { line: 2, column: 3 });
        assert_eq!(tokens[5].kind, TokenKind::Eof);

        // 测试多行注释
        let tokens = tokenize("123 /* 多行\n注释 */ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(
            tokens[1].kind,
            TokenKind::Comment(" 多行\n注释 ".to_string())
        );
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[2].kind, TokenKind::Number(456.0));
        assert_eq!(tokens[2].span, Span { line: 2, column: 7 });
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        // 测试多行注释中的代码
        let tokens = tokenize("123 /* let x = 5 */ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(
            tokens[1].kind,
            TokenKind::Comment(" let x = 5 ".to_string())
        );
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[2].kind, TokenKind::Number(456.0));
        assert_eq!(
            tokens[2].span,
            Span {
                line: 1,
                column: 21
            }
        );
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        // 测试未闭合的多行注释
        assert!(tokenize("123 /* 未闭合注释").is_err());
    }

    #[test]
    fn test_position_tracking() {
        let input = "let x = 123\nif x > 0 {\n  return x\n}";
        let tokens = tokenize(input).unwrap();

        // 验证let的位置
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });

        // 验证x的位置
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });

        // 验证=的位置
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[2].span, Span { line: 1, column: 7 });

        // 验证123的位置
        assert_eq!(tokens[3].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[3].span, Span { line: 1, column: 9 });

        // 验证if的位置
        assert_eq!(tokens[5].kind, TokenKind::If);
        assert_eq!(tokens[5].span, Span { line: 2, column: 1 });

        // 验证}的位置
        assert_eq!(tokens[14].kind, TokenKind::RightBrace);
        assert_eq!(tokens[14].span, Span { line: 4, column: 1 });
    }
}
