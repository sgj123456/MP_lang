#[cfg(test)]
mod tests {
    use mp_lang::{
        lexer::{Span, TokenKind, tokenize},
        runtime::environment::value::Number,
    };

    #[test]
    fn test_number() {
        let tokens = tokenize("123 45.67").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(Number::Int(123)));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::Number(Number::Float(45.67)));
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
    fn test_string() {
        let tokens = tokenize("\"hello\" \"world\"").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::String("hello".to_string()));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::String("world".to_string()));
        assert_eq!(tokens[1].span, Span { line: 1, column: 9 });
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_punctuation() {
        let tokens = tokenize(", ; ( ) [ ] { }").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Comma);
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[1].kind, TokenKind::Semicolon);
        assert_eq!(tokens[1].span, Span { line: 1, column: 3 });
        assert_eq!(tokens[2].kind, TokenKind::LeftParen);
        assert_eq!(tokens[2].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[3].kind, TokenKind::RightParen);
        assert_eq!(tokens[3].span, Span { line: 1, column: 7 });
        assert_eq!(tokens[4].kind, TokenKind::LeftBracket);
        assert_eq!(tokens[4].span, Span { line: 1, column: 9 });
        assert_eq!(tokens[5].kind, TokenKind::RightBracket);
        assert_eq!(
            tokens[5].span,
            Span {
                line: 1,
                column: 11
            }
        );
        assert_eq!(tokens[6].kind, TokenKind::LeftBrace);
        assert_eq!(
            tokens[6].span,
            Span {
                line: 1,
                column: 13
            }
        );
        assert_eq!(tokens[7].kind, TokenKind::RightBrace);
        assert_eq!(
            tokens[7].span,
            Span {
                line: 1,
                column: 15
            }
        );
        assert_eq!(tokens[8].kind, TokenKind::Eof);
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

        assert!(tokenize("\"unclosed").is_err());
    }

    #[test]
    fn test_comments() {
        let tokens = tokenize("// This is a comment.\n123").unwrap();
        assert_eq!(
            tokens[0].kind,
            TokenKind::Comment(" This is a comment.".into())
        );
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[2].kind, TokenKind::Number(Number::Int(123)));
        assert_eq!(tokens[2].span, Span { line: 2, column: 1 });
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        let tokens = tokenize("123 // This is a number.\n+ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(Number::Int(123)));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[3].kind, TokenKind::Plus);
        assert_eq!(tokens[3].span, Span { line: 2, column: 1 });
        assert_eq!(tokens[4].kind, TokenKind::Number(Number::Int(456)));
        assert_eq!(tokens[4].span, Span { line: 2, column: 3 });
        assert_eq!(tokens[5].kind, TokenKind::Eof);

        let tokens = tokenize("123 /* This is a multi-line\ncomment */ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(Number::Int(123)));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(
            tokens[1].kind,
            TokenKind::Comment(" This is a multi-line\ncomment ".to_string())
        );
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[2].kind, TokenKind::Number(Number::Int(456)));
        assert_eq!(
            tokens[2].span,
            Span {
                line: 2,
                column: 12
            }
        );
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        let tokens = tokenize("123 /* let x = 5 */ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(Number::Int(123)));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(
            tokens[1].kind,
            TokenKind::Comment(" let x = 5 ".to_string())
        );
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });
        assert_eq!(tokens[2].kind, TokenKind::Number(Number::Int(456)));
        assert_eq!(
            tokens[2].span,
            Span {
                line: 1,
                column: 21
            }
        );
        assert_eq!(tokens[3].kind, TokenKind::Eof);

        assert!(tokenize("123 /* Unclosed comment").is_err());
    }

    #[test]
    fn test_position_tracking() {
        let input = "let x = 123\nif x > 0 {\n  return x\n}";
        let tokens = tokenize(input).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });

        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[1].span, Span { line: 1, column: 5 });

        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[2].span, Span { line: 1, column: 7 });

        assert_eq!(tokens[3].kind, TokenKind::Number(Number::Int(123)));
        assert_eq!(tokens[3].span, Span { line: 1, column: 9 });

        assert_eq!(tokens[5].kind, TokenKind::If);
        assert_eq!(tokens[5].span, Span { line: 2, column: 1 });

        assert_eq!(tokens[14].kind, TokenKind::RightBrace);
        assert_eq!(tokens[14].span, Span { line: 4, column: 1 });
    }
}
