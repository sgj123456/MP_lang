// 词法分析器

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum LexerError {
    UnknownOperator(char, Span),
    InvalidNumber(String, Span),
    UnexpectedChar(char, Span),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnknownOperator(c, span) => write!(f, "{}: 未知运算符: '{c}'", span),
            LexerError::InvalidNumber(s, span) => write!(f, "{}: 无效数字: '{s}'", span),
            LexerError::UnexpectedChar(c, span) => write!(f, "{}: 意外字符: '{c}'", span),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(f64),
    Boolean(bool),
    String(String),
    Comment(String),
    Comma,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LeftParen,
    RightParen,
    #[allow(dead_code)]
    LeftBracket,
    #[allow(dead_code)]
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Newline,
    Identifier(String),
    Let,
    Fn,
    If,
    Else,
    While,
    Return,
    Eof,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&c) = chars.peek() {
        let span = Span { line, column };

        match c {
            ' ' | '\t' | '\r' => {
                chars.next();
                column += 1;
            }
            '\n' => {
                tokens.push(Token {
                    kind: TokenKind::Newline,
                    span,
                });
                chars.next();
                line += 1;
                column = 1;
            }
            '+' => {
                tokens.push(Token {
                    kind: TokenKind::Plus,
                    span,
                });
                chars.next();
                column += 1;
            }
            '-' => {
                tokens.push(Token {
                    kind: TokenKind::Minus,
                    span,
                });
                chars.next();
                column += 1;
            }
            '*' => {
                tokens.push(Token {
                    kind: TokenKind::Multiply,
                    span,
                });
                chars.next();
                column += 1;
            }
            '/' => {
                chars.next();
                column += 1;
                match chars.peek() {
                    Some('/') => {
                        // 单行注释，跳过直到行尾(包括换行符)
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    }
                    Some('*') => {
                        // 多行注释
                        chars.next(); // 跳过*
                        column += 1;
                        let mut comment = String::new();
                        let mut closed = false;

                        while let Some(&c) = chars.peek() {
                            if c == '*' {
                                chars.next();
                                column += 1;
                                if let Some(&'/') = chars.peek() {
                                    closed = true;
                                    chars.next();
                                    column += 1;
                                    break;
                                } else {
                                    comment.push('*');
                                }
                            } else if c == '\n' {
                                comment.push(c);
                                line += 1;
                                column = 1;
                                chars.next();
                            } else {
                                comment.push(c);
                                chars.next();
                                column += 1;
                            }
                        }

                        if !closed {
                            return Err(LexerError::UnexpectedChar('*', span));
                        }

                        tokens.push(Token {
                            kind: TokenKind::Comment(comment),
                            span,
                        });
                    }
                    _ => {
                        tokens.push(Token {
                            kind: TokenKind::Divide,
                            span,
                        });
                    }
                }
            }
            '=' => {
                chars.next();
                column += 1;
                if let Some('=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        kind: TokenKind::Equal,
                        span,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Equal,
                        span,
                    });
                }
            }
            '!' => {
                chars.next();
                column += 1;
                if let Some('=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        kind: TokenKind::NotEqual,
                        span,
                    });
                } else {
                    return Err(LexerError::UnknownOperator('!', span));
                }
            }
            '>' => {
                chars.next();
                column += 1;
                if let Some('=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        kind: TokenKind::GreaterThanOrEqual,
                        span,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::GreaterThan,
                        span,
                    });
                }
            }
            '<' => {
                chars.next();
                column += 1;
                if let Some('=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push(Token {
                        kind: TokenKind::LessThanOrEqual,
                        span,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::LessThan,
                        span,
                    });
                }
            }
            '(' => {
                tokens.push(Token {
                    kind: TokenKind::LeftParen,
                    span,
                });
                chars.next();
                column += 1;
            }
            ')' => {
                tokens.push(Token {
                    kind: TokenKind::RightParen,
                    span,
                });
                chars.next();
                column += 1;
            }
            ',' => {
                tokens.push(Token {
                    kind: TokenKind::Comma,
                    span,
                });
                chars.next();
                column += 1;
            }
            '{' => {
                tokens.push(Token {
                    kind: TokenKind::LeftBrace,
                    span,
                });
                chars.next();
                column += 1;
            }
            '}' => {
                tokens.push(Token {
                    kind: TokenKind::RightBrace,
                    span,
                });
                chars.next();
                column += 1;
            }
            ';' => {
                tokens.push(Token {
                    kind: TokenKind::Semicolon,
                    span,
                });
                chars.next();
                column += 1;
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                match num.parse::<f64>() {
                    Ok(n) => tokens.push(Token {
                        kind: TokenKind::Number(n),
                        span,
                    }),
                    Err(_) => return Err(LexerError::InvalidNumber(num, span)),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                // 简单关键字识别
                match ident.as_str() {
                    "let" => {
                        tokens.push(Token {
                            kind: TokenKind::Let,
                            span,
                        });
                    }
                    "fn" => {
                        tokens.push(Token {
                            kind: TokenKind::Fn,
                            span,
                        });
                    }
                    "if" => {
                        tokens.push(Token {
                            kind: TokenKind::If,
                            span,
                        });
                    }
                    "else" => {
                        tokens.push(Token {
                            kind: TokenKind::Else,
                            span,
                        });
                    }
                    "while" => {
                        tokens.push(Token {
                            kind: TokenKind::While,
                            span,
                        });
                    }
                    "return" => {
                        tokens.push(Token {
                            kind: TokenKind::Return,
                            span,
                        });
                    }
                    "true" => {
                        tokens.push(Token {
                            kind: TokenKind::Boolean(true),
                            span,
                        });
                    }
                    "false" => {
                        tokens.push(Token {
                            kind: TokenKind::Boolean(false),
                            span,
                        });
                    }
                    _ => {
                        tokens.push(Token {
                            kind: TokenKind::Identifier(ident),
                            span,
                        });
                    }
                }
            }
            '"' => {
                chars.next(); // 跳过开始引号
                column += 1;
                let mut s = String::new();
                let mut escaped = false;
                let mut closed = false;

                while let Some(&c) = chars.peek() {
                    if escaped {
                        match c {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            _ => return Err(LexerError::UnexpectedChar(c, span)),
                        }
                        escaped = false;
                        chars.next();
                        column += 1;
                    } else if c == '\\' {
                        escaped = true;
                        chars.next();
                        column += 1;
                    } else if c == '"' {
                        closed = true;
                        chars.next();
                        column += 1;
                        break;
                    } else {
                        s.push(c);
                        chars.next();
                        column += 1;
                    }
                }

                if !closed {
                    return Err(LexerError::UnexpectedChar('"', span));
                }

                tokens.push(Token {
                    kind: TokenKind::String(s),
                    span,
                });
            }
            _ => {
                return Err(LexerError::UnexpectedChar(c, span));
            }
        }
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span { line, column },
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(tokens[1].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[1].span, Span { line: 2, column: 1 });
        assert_eq!(tokens[2].kind, TokenKind::Eof);

        // 测试注释后的代码
        let tokens = tokenize("123 // 数字\n+ 456").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(123.0));
        assert_eq!(tokens[0].span, Span { line: 1, column: 1 });
        assert_eq!(tokens[2].kind, TokenKind::Plus);
        assert_eq!(tokens[2].span, Span { line: 2, column: 1 });
        assert_eq!(tokens[3].kind, TokenKind::Number(456.0));
        assert_eq!(tokens[3].span, Span { line: 2, column: 3 });
        assert_eq!(tokens[4].kind, TokenKind::Eof);

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
