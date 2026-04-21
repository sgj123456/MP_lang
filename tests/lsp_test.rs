#[cfg(test)]
mod tests {
    use mp_lang::lsp::completion::MpCompleter;
    use mp_lang::lsp::diagnostics::MpDiagnostics;
    use mp_lang::lsp::hover::MpHover;
    use tower_lsp_server::ls_types::Position;

    #[test]
    fn test_diagnostics_empty_file() {
        let diagnostics = MpDiagnostics::new();
        let content = "";
        let result = diagnostics.analyze(content);

        // Empty file should have no diagnostics
        assert_eq!(result.0.len(), 0, "Empty file should have no diagnostics");
    }

    #[test]
    fn test_diagnostics_valid_code() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = 10\nprint(x)";
        let result = diagnostics.analyze(content);

        // Valid code should have no diagnostics
        assert_eq!(result.0.len(), 0, "Valid code should have no diagnostics");
    }

    #[test]
    fn test_diagnostics_lexer_invalid_number() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = 12.34.56";
        let result = diagnostics.analyze(content);

        assert!(
            result.0.len() > 0,
            "Should have lexer error for invalid number"
        );
        assert_eq!(
            result.0[0].code,
            Some(tower_lsp_server::ls_types::NumberOrString::String(
                "MP001".to_string()
            ))
        );
    }

    #[test]
    fn test_diagnostics_lexer_unexpected_character() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = @";
        let result = diagnostics.analyze(content);

        assert!(
            result.0.len() > 0,
            "Should have lexer error for unexpected character"
        );
        assert_eq!(
            result.0[0].code,
            Some(tower_lsp_server::ls_types::NumberOrString::String(
                "MP001".to_string()
            ))
        );
    }

    #[test]
    fn test_diagnostics_lexer_unclosed_string() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = \"hello";
        let result = diagnostics.analyze(content);

        assert!(
            result.0.len() > 0,
            "Should have lexer error for unclosed string"
        );
        assert_eq!(
            result.0[0].code,
            Some(tower_lsp_server::ls_types::NumberOrString::String(
                "MP001".to_string()
            ))
        );
    }

    #[test]
    fn test_diagnostics_lexer_unclosed_comment() {
        let diagnostics = MpDiagnostics::new();
        let content = "/* this is a comment";
        let result = diagnostics.analyze(content);

        assert!(
            result.0.len() > 0,
            "Should have lexer error for unclosed comment"
        );
        assert_eq!(
            result.0[0].code,
            Some(tower_lsp_server::ls_types::NumberOrString::String(
                "MP001".to_string()
            ))
        );
    }

    #[test]
    fn test_diagnostics_lexer_escape_sequences() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = \"hello\\nworld\"";
        let result = diagnostics.analyze(content);

        assert_eq!(
            result.0.len(),
            0,
            "Valid escape sequences should not produce errors"
        );
    }

    #[test]
    fn test_diagnostics_parser_unexpected_token() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = ";
        let result = diagnostics.analyze(content);

        assert!(result.0.len() > 0, "Should have parser error");
        assert_eq!(
            result.0[0].code,
            Some(tower_lsp_server::ls_types::NumberOrString::String(
                "MP002".to_string()
            ))
        );
    }

    #[test]
    fn test_diagnostics_lexer_error_stops_parsing() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = @\nlet y = 10";
        let result = diagnostics.analyze(content);

        assert!(result.0.len() > 0, "Should have lexer error");
        assert_eq!(
            result.0[0].code,
            Some(tower_lsp_server::ls_types::NumberOrString::String(
                "MP001".to_string()
            ))
        );
    }

    #[test]
    fn test_diagnostics_span_position() {
        let diagnostics = MpDiagnostics::new();
        let content = "let x = @";
        let result = diagnostics.analyze(content);

        assert!(result.0.len() > 0, "Should have diagnostic");
        let range = &result.0[0].range;
        assert!(range.start.line == 0, "Should have valid line at 0");
        assert!(
            range.start.character > 0,
            "Should have character position > 0"
        );
    }

    #[test]
    fn test_completion_keywords() {
        let completer = MpCompleter::new();
        let content = "l";
        let completions = completer.complete(
            content,
            Position {
                line: 0,
                character: 1,
            },
        );

        let has_let = completions.iter().any(|c| c.label == "let");
        assert!(has_let, "Completion should include 'let' keyword");
    }

    #[test]
    fn test_completion_builtin_functions() {
        let completer = MpCompleter::new();
        let content = "p";
        let completions = completer.complete(
            content,
            Position {
                line: 0,
                character: 1,
            },
        );

        let has_print = completions.iter().any(|c| c.label == "print");
        assert!(has_print, "Completion should include 'print' function");
    }

    #[test]
    fn test_completion_variables() {
        let completer = MpCompleter::new();
        let content = "let x = 10\nx";
        let completions = completer.complete(
            content,
            Position {
                line: 1,
                character: 1,
            },
        );

        let has_x = completions.iter().any(|c| c.label == "x");
        assert!(has_x, "Completion should include variable 'x'");
    }

    #[test]
    fn test_completion_empty_line() {
        let completer = MpCompleter::new();
        let content = "";
        let completions = completer.complete(
            content,
            Position {
                line: 0,
                character: 0,
            },
        );

        assert!(
            !completions.is_empty(),
            "Should have completions on empty line"
        );
    }

    #[test]
    fn test_hover_builtin_functions() {
        let hover = MpHover::new();
        let content = "print";
        let result = hover.hover(
            content,
            Position {
                line: 0,
                character: 0,
            },
        );

        assert!(result.is_some(), "Should have hover information for print");
    }

    #[test]
    fn test_hover_keywords() {
        let hover = MpHover::new();
        let content = "let";
        let result = hover.hover(
            content,
            Position {
                line: 0,
                character: 0,
            },
        );

        assert!(result.is_some(), "Should have hover information for let");
    }

    #[test]
    fn test_hover_numbers() {
        let hover = MpHover::new();
        let content = "42";
        let result = hover.hover(
            content,
            Position {
                line: 0,
                character: 0,
            },
        );

        assert!(
            result.is_some(),
            "Should have hover information for numbers"
        );
    }

    #[test]
    fn test_hover_operators() {
        let hover = MpHover::new();
        let content = "1 + 2";
        let result = hover.hover(
            content,
            Position {
                line: 0,
                character: 2,
            },
        );

        assert!(
            result.is_some(),
            "Should have hover information for operators"
        );
    }

    #[test]
    fn test_hover_strings() {
        let hover = MpHover::new();
        let content = "\"hello\"";
        let result = hover.hover(
            content,
            Position {
                line: 0,
                character: 0,
            },
        );

        assert!(
            result.is_some(),
            "Should have hover information for strings"
        );
    }
}
