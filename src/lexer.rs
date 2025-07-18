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
            LexerError::UnknownOperator(c, span) => write!(f, "{span}: 未知运算符: '{c}'"),
            LexerError::InvalidNumber(s, span) => write!(f, "{span}: 无效数字: '{s}'"),
            LexerError::UnexpectedChar(c, span) => write!(f, "{span}: 意外字符: '{c}'"),
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
