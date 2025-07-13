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
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LeftParen,
    RightParen,
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
                tokens.push(Token::Divide);
                chars.next();
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
                    "let" | "fn" | "if" | "else" | "then" => {
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
        let tokens = tokenize("let if then else").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("let".to_string()),
                Token::Keyword("if".to_string()),
                Token::Keyword("then".to_string()),
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
}
