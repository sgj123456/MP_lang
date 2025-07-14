// 词法分析器

use std::fmt;

use std::error::Error;

impl Error for LexerError {}

#[derive(Debug)]
pub enum LexerError {
    UnknownOperator(char),
    InvalidNumber(String),
    UnexpectedChar(char),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnknownOperator(c) => write!(f, "未知运算符: '{c}'"),
            LexerError::InvalidNumber(s) => write!(f, "无效数字: '{s}'"),
            LexerError::UnexpectedChar(c) => write!(f, "意外字符: '{c}'"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Identifier(String),
    Keyword(String),
    Eof,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next(); // 跳过空白字符
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                chars.next();
                match chars.peek() {
                    Some('/') => {
                        // 单行注释，跳过直到行尾
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
                        let mut comment = String::new();
                        let mut closed = false;

                        while let Some(&c) = chars.peek() {
                            if c == '*' {
                                chars.next();
                                if let Some(&'/') = chars.peek() {
                                    closed = true;
                                    chars.next();
                                    break;
                                } else {
                                    comment.push('*');
                                }
                            } else {
                                comment.push(c);
                                chars.next();
                            }
                        }

                        if !closed {
                            return Err(LexerError::UnexpectedChar('*'));
                        }

                        tokens.push(Token::Comment(comment));
                    }
                    _ => {
                        tokens.push(Token::Divide);
                    }
                }
            }
            '=' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    tokens.push(Token::Keyword("==".to_string()));
                    chars.next();
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    tokens.push(Token::Keyword("!=".to_string()));
                    chars.next();
                } else {
                    return Err(LexerError::UnknownOperator('!'));
                }
            }
            '>' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    tokens.push(Token::Keyword(">=".to_string()));
                    chars.next();
                } else {
                    tokens.push(Token::Keyword(">".to_string()));
                }
            }
            '<' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    tokens.push(Token::Keyword("<=".to_string()));
                    chars.next();
                } else {
                    tokens.push(Token::Keyword("<".to_string()));
                }
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match num.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err(LexerError::InvalidNumber(num)),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // 简单关键字识别
                match ident.as_str() {
                    "let" | "fn" | "if" | "else" | "while" => {
                        tokens.push(Token::Keyword(ident));
                    }
                    "true" => {
                        tokens.push(Token::Boolean(true));
                    }
                    "false" => {
                        tokens.push(Token::Boolean(false));
                    }
                    _ => {
                        tokens.push(Token::Identifier(ident));
                    }
                }
            }
            '"' => {
                chars.next(); // 跳过开始引号
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
                            _ => return Err(LexerError::UnexpectedChar(c)),
                        }
                        escaped = false;
                        chars.next();
                    } else if c == '\\' {
                        escaped = true;
                        chars.next();
                    } else if c == '"' {
                        closed = true;
                        chars.next();
                        break;
                    } else {
                        s.push(c);
                        chars.next();
                    }
                }

                if !closed {
                    return Err(LexerError::UnexpectedChar('"'));
                }

                tokens.push(Token::String(s));
            }
            _ => {
                return Err(LexerError::UnexpectedChar(c));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let tokens = tokenize("123 45.67").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(123.0), Token::Number(45.67), Token::Eof]
        );
    }

    #[test]
    fn test_boolean() {
        let tokens = tokenize("true false").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Boolean(true), Token::Boolean(false), Token::Eof]
        );
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("+ - * /").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Multiply,
                Token::Divide,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize("let if else").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("let".to_string()),
                Token::Keyword("if".to_string()),
                Token::Keyword("else".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize("x y_z").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x".to_string()),
                Token::Identifier("y_z".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_strings() {
        let tokens = tokenize("\"hello\" \"world\\n\" \"say \\\"hi\\\"\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::String("hello".to_string()),
                Token::String("world\n".to_string()),
                Token::String("say \"hi\"".to_string()),
                Token::Eof
            ]
        );

        // 测试未闭合的字符串
        assert!(tokenize("\"unclosed").is_err());
    }

    #[test]
    fn test_comments() {
        // 测试单行注释
        let tokens = tokenize("// 这是一个注释\n123").unwrap();
        assert_eq!(tokens, vec![Token::Number(123.0), Token::Eof]);

        // 测试注释后的代码
        let tokens = tokenize("123 // 数字\n+ 456").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Plus,
                Token::Number(456.0),
                Token::Eof
            ]
        );

        // 测试多行注释
        let tokens = tokenize("123 /* 多行\n注释 */ 456").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Comment(" 多行\n注释 ".to_string()),
                Token::Number(456.0),
                Token::Eof
            ]
        );

        // 测试多行注释中的代码
        let tokens = tokenize("123 /* let x = 5 */ 456").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Comment(" let x = 5 ".to_string()),
                Token::Number(456.0),
                Token::Eof
            ]
        );

        // 测试未闭合的多行注释
        assert!(tokenize("123 /* 未闭合注释").is_err());
    }
}
